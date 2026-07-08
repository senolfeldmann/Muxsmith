severity-error = error
severity-warning = warning
severity-info = info

unsupported-profile-version = Unsupported profile_version { $found } (supported: { $supported }).
parse-error = The profile could not be parsed: { $detail }
no-track-rules = The profile defines no track rules; at least one is required.
empty-match-expression = This match expression is empty and would match every track.
empty-extensions = The extensions list must not be empty.
invalid-regex = Invalid regular expression: { $detail }
unknown-property = Unknown property "{ $property }". It is not part of the mkvmerge identification model.
not-string-property = Property "{ $property }" has type { $actual_type }; { $condition } conditions require a string property.
value-type-mismatch = Value for "{ $property }" has type { $found }, expected { $expected }.
unknown-settable-property = "{ $property }" is not a settable track property.
invalid-keyword = Invalid keyword "{ $found }". Allowed: { $allowed }.
locator-conflict = match_to_source and match_pattern are mutually exclusive; set only one.
invalid-template = Invalid template: { $kind ->
    [unclosed-brace] unclosed brace at position { $pos }
   *[empty-field] empty field at position { $pos }
}
unknown-template-field = Unknown template field "{ $field }". Available fields: { $allowed }.
unknown-template-filter = Unknown template filter "{ $name }".
path-separator-in-template = Filename templates must not contain path separators.
attachment-rule-shape = Each attachment rule needs exactly one of select, drop, add (found { $found }).
provable-overlap = Rules { $rule_a } and { $rule_b } provably overlap: every track matching one also matches the other. Add a distinguishing condition to one of them.
ambiguous-rule = Rule matches { $count } tracks; it must match exactly one.
overlapping-rules = Rules { $rule_a } and { $rule_b } both claim track { $track }.
missing-track = No track matches this non-optional rule.
missing-external = No file matches this external locator.
ambiguous-external = { $count } files match this external locator; exactly one is required.
output-collision = Output path { $path } collides with an existing file or another planned output.
source-overwrite = Output path { $path } would overwrite a source file. This is never allowed.
duplicate-identifier = Files { $file_a } and { $file_b } share the identifier "{ $identifier }".
donor-is-primary = External donor file { $donor } is itself a primary source.
ignored-file = File matches the extension list but not the input pattern.
multiple-identifier-matches = The input pattern matches more than once in "{ $name }"; the first match is used.
unknown-property-skew = Property "{ $property }" is unknown to this Muxsmith build but reported by the local mkvmerge; it is matched untyped.
