# Suggestion card

A dry run can find a conflict that narrowing one rule's match would fix: a rule matches several tracks where it must match exactly one, or several rules claim the same track. The report then proposes the fix as a suggestion card. The card names the affected rule by its config path (for example `tracks[2].match`) and shows, as YAML, the match constraint the suggestion would add to that rule: the same text the CLI prints for this suggestion, whose leading comment line marks it as an addition. The fragment is a preview of that addition, not the rule's future content; Apply merges the constraint into the rule's existing match.

## Copy or apply

**Copy** puts the YAML fragment on the clipboard so you can paste it into the profile yourself.

**Apply** performs the whole round trip in one click: it loads the profile fresh from disk, applies the narrowing, and saves the profile file back to disk immediately. There is no separate confirmation or save step; after Apply, the file on disk has changed. Saving rewrites the file canonically, so comments and formatting in it are not preserved (see the Editor topic).

## What apply will never do

An applied suggestion only ever narrows the match of the one conflicted rule. It never reorders rules, never touches any other rule, and never loosens a match. Applying suggestions repeatedly is therefore guaranteed to terminate instead of oscillating; conflicts the report offers no suggestion for remain and need a manual edit.

## After applying

The report on screen is not refreshed: it still shows the state from before the apply. Run the dry run again to see the effect; the applied change is guaranteed to survive that next dry run.
