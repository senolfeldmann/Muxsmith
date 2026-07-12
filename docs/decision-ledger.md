# Muxsmith decision ledger (Tier 1)

Every considered approach and its outcome, recorded deterministically. This
is the low-visibility tier: **always written, not always loaded**. The
always-checked house rules live in `docs/CONVENTIONS.md` (Tier 2); an item
is promoted there when its recurrence count reaches **3**.

Mechanism (the controller is the single writer; counting, promotion, and
the deliberation trigger): software-dev-process doctrine, section 7.

## Format

One entry per considered `(topic, approach)`. The controller records a
fresh consideration, or increments an existing one, when a subagent report
or a session surfaces it.

```
### <topic> :: <approach>
- outcome: accepted | rejected | deferred(blocked-on: <B>)
- count: <n>
- last: <ISO date>
- reasoning: <one line; link the memo/finding if any>
```

Promotion: count reaches 3 -> move to `CONVENTIONS.md` (as a pattern,
restraint, or non-decision by outcome), leaving a one-line stub here that
points there. An item that keeps recurring while still *contested* (no
clear stable outcome) is not just incremented -> it triggers the bounded
deliberation (doctrine §7).

## Entries

(none yet.)

Threshold 3 is deliberately conservative, to avoid overfitting one-offs
into the rulebook. If many genuine convention-worthy items accumulate at
count 1-2, that is the signal to discuss lowering it - a learning
experience, not a fixed constant.
