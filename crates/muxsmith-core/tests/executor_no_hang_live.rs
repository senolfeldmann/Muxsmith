//! No-hang regression (#9, Plan 5.5 Task 3): a real child process that
//! writes a non-UTF-8 line followed by more output than a pipe's default OS
//! buffer holds must not stall `run_job`. Spawns a scripted shell script
//! through the real `LiveSpawner` (not `FakeSpawner`, which is pure
//! in-memory and cannot reproduce OS pipe backpressure) - unix-only for the
//! same reason `spawn.rs`'s own `live_killer_then_wait_returns_none` is,
//! and for the same reason does not need a real mkvmerge on PATH: the
//! script stands in for the child process being spawned, mirroring that
//! test's pattern rather than `executor_live.rs`'s (which specifically
//! checks real mkvmerge's `--gui-mode` grammar).
//!
//! Before the #9 fix, `LiveJob::next_line` returned `None` on the first
//! non-UTF-8 read error, so `run_job`'s drain loop stopped there; the child
//! kept running and blocked on `write()` once the undrained pipe filled,
//! and `run_job`'s later `running.wait()` (a blocking `waitpid`) then
//! stalled forever waiting for a child that could never exit. Run on a
//! background thread with a bounded `recv_timeout` so a regression fails
//! the test cleanly instead of hanging the suite.

#![cfg(unix)]

use std::io::Write as _;
use std::sync::mpsc;
use std::time::Duration;

use muxsmith_core::executor::job::{JobProgress, JobSpec, JobState, run_job};
use muxsmith_core::executor::spawn::LiveSpawner;

#[test]
#[cfg(unix)]
fn run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let script = dir.path().join("fake-mkvmerge");
    {
        let mut f = std::fs::File::create(&script).unwrap();
        // `\377\376` are raw invalid-UTF-8 bytes (POSIX `printf` octal
        // escapes); `yes | head` emits ~140KB of filler, comfortably above
        // a pipe's usual 64KiB OS buffer. If nothing drains the pipe past
        // the bad line, the script's own `write()` calls block once that
        // buffer fills, so the child never reaches its final `printf` or
        // exits - exactly the hang this test pins.
        write!(
            f,
            "#!/bin/sh\n\
             printf 'ok line\\n'\n\
             printf 'broken \\377\\376 line\\n'\n\
             yes 'filler line xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx' | head -n 2000\n\
             printf 'TAIL_MARKER\\n'\n"
        )
        .unwrap();
    }
    let mut perms = std::fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&script, perms).unwrap();

    let spawner = LiveSpawner { mkvmerge: script };
    let spec = JobSpec {
        argv: vec![],
        output: dir.path().join("out.mkv"),
    };

    // `run_job` itself has no timeout; a hang regression would block this
    // thread forever. Drive it on a worker thread and bound the wait here,
    // so the bug reproduces as a clean, fast test failure instead of a
    // frozen `cargo test` run.
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut lines = Vec::new();
        let outcome = run_job(&spawner, &spec, &|| false, &mut |p| {
            if let JobProgress::OutputLine(l) = p {
                lines.push(l);
            }
        });
        let _ = tx.send((outcome, lines));
    });

    let (outcome, lines) = rx.recv_timeout(Duration::from_secs(10)).expect(
        "run_job did not complete within 10s - the pipe-full hang this test \
         pins (#9) is back",
    );

    assert_eq!(outcome.state, JobState::Ok, "outcome: {outcome:?}");
    assert!(
        lines.iter().any(|l| l == "ok line"),
        "the line before the bad one must survive, lines: {lines:?}"
    );
    assert!(
        lines.iter().any(|l| l.contains('\u{FFFD}')),
        "expected a decode-degraded line (U+FFFD) in the captured output, \
         lines: {lines:?}"
    );
    assert_eq!(
        lines.last().map(String::as_str),
        Some("TAIL_MARKER"),
        "the full >64KiB tail after the bad line must be captured, not \
         truncated - lines: {lines:?}"
    );
}
