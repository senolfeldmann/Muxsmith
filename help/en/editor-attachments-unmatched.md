# Unmatched (attachments)

`Unmatched` decides what happens to attachments that no attachment rule matches: `keep` or `drop`. The default is `keep` - deliberately the opposite of the tracks default.

## Why the default keeps

Attachments are auxiliary payload, most commonly fonts - and dropping fonts silently breaks ASS/SSA subtitle rendering on playback, with no error anywhere in the chain. `keep` makes the safe behavior the default: attachments pass through unless you explicitly handle them.

## `drop`

The output contains only the attachments a `Select` rule matched, plus files added by `Add` rules. Use it to strip cover art or other unwanted payload deliberately - and then write select rules for the fonts your subtitles need, or they are gone.

## Interactions

- The rules run first, in list order, first matching rule wins per attachment; only attachments no rule touched fall through to this policy. See the Attachment rules topic.
- Same control shape as `Unmatched` under Tracks, different domain semantics: tracks default to `drop` because the output lineup is usually meant to be defined; attachments default to `keep` because they are auxiliary and dropping them fails silently.
