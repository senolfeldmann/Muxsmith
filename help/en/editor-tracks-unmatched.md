# Unmatched tracks

`Unmatched` decides what happens to tracks of the primary file that no track rule matches: `keep` or `drop`. The default is `drop`. The policy never applies to donor tracks - external tracks enter the output only through a rule.

## `drop`

The output contains exactly what the rules matched. Precise, but unforgiving: a track type you forgot to write a rule for disappears silently from the output. Under `drop` an empty rule list is an error, since every track would be dropped.

## `keep`

Every primary track passes through; rules then edit properties or add donor tracks on top. A rule that matches a primary track still applies its changes. Under `keep` an empty rule list is legal: a pure passthrough remux (normalize the container, touch only title, chapters, attachments, or tags); validate announces that case with an info notice.

## Ordering interaction

Rule order defines the output track order - but under `keep` only partially: the output lists all primary tracks first, in the primary's own order, then donor tracks in rule order. Kept-but-unmatched primary tracks count as matched for this ordering, and a rule matching a primary track does not reposition it. Reordering primary tracks therefore requires `drop`, where the rule list alone defines the order. See the Track rules topic.

## Choosing

Use `drop` when the output should have a defined track lineup (the typical series profile: video, two audio languages, selected subtitles). Use `keep` when the source structure should survive and you only adjust properties or add donor tracks.
