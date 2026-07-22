# Rules (tracks)

The rules grid is the heart of a profile: an ordered list of rules, each selecting exactly one track per source file and optionally changing its properties. Each grid row summarizes one rule - source, match expression, optional flag, changes.

## Order is output order

The row order defines the output track order. Drag a row to reorder; the new grid order is the new output order. One caveat: under `Unmatched: keep`, primary tracks keep their source order at the front of the output and only donor tracks follow in rule order - see the Unmatched (tracks) topic.

## Editing a rule

Click a row to select it; the detail panel below the grid edits the selected rule - its source (the primary file or an external donor), its match expression, the optional flag, and the property changes.

## Exactly one track per rule

Each rule must resolve to exactly one track per source file. Two candidate tracks is an ambiguity error - narrow the match expression (the suggestion cards on the Batch view can do that narrowing for you). Zero candidates is an error too, unless the rule is `Optional` - see the Optional topic.

## When the list may be empty

An empty rule list is legal only under `Unmatched: keep` - a pure passthrough remux. Under `drop` at least one rule is required, since otherwise every track would be dropped.
