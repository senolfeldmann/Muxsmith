# Rules (attachments)

An ordered list of rules over the source file's attachments. Each rule does exactly one of three things:

- `Select` keeps the attachments its expression matches.
- `Drop` removes the attachments its expression matches.
- `Add` attaches an external file from disk, found via a locator (path, extensions, pairing - the same mechanism external track sources use).

## First match wins

Rules resolve in list order, and for every attachment the first rule whose expression matches decides its fate. Attachments no rule matches fall through to the `Unmatched` policy - see that topic. Drag rows to reorder: place a specific `Drop` rule above a broad `Select`, or the select claims the attachment first.

## Sets, not single picks

Unlike track rules, attachment rules are not uniqueness-constrained: one rule may match many attachments - fonts come in sets, and one `Select` can keep them all. Zero matches is not an error either: a `Select` or `Drop` that matches nothing simply does nothing, and an `Add` whose locator finds no file emits a warning but does not fail the plan.

## Matching

Expressions use the same match algebra as track rules (`exact`, `substring`, `regex`, `any`, `not` - see the Match topic), over three attachment properties: `file_name`, `content_type`, `description`. Example: keep all fonts with a `Select` whose expression is `substring: { content_type: font }`.
