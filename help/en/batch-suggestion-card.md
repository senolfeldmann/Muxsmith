# Suggestion card

A dry run can find a conflict that narrowing one rule's match would fix: a rule matches several tracks where it must match exactly one, or several rules claim the same track. The report then proposes the fix as a suggestion card. The card names the affected rule by its config path (for example `tracks[2].match`) and shows the proposed rule fragment as YAML, exactly the text that would land in the profile.

## Copy or apply

**Copy** puts the YAML fragment on the clipboard so you can paste it into the profile yourself.

**Apply** performs the whole round trip in one click: it loads the profile fresh from disk, applies the narrowing, and saves the profile file back to disk immediately. There is no separate confirmation or save step; after Apply, the file on disk has changed. Saving rewrites the file canonically, so comments and formatting in it are not preserved (see the Editor topic).

## What apply will never do

An applied suggestion only ever narrows the match of the one conflicted rule. It never reorders rules, never touches any other rule, and never loosens a match. Applying suggestions repeatedly therefore converges on a resolved profile instead of oscillating.

## After applying

The report on screen is not refreshed: it still shows the state from before the apply. Run the dry run again to see the effect; the applied change is guaranteed to survive that next dry run.
