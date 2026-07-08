//! mkvtoolnix capability model (spec 4.4 / 9). Matchable properties are
//! generated from the identification schema; settable properties and
//! codec_kind aliases are curated here. Runtime queries (--list-types,
//! --list-languages, --version) arrive in Plan 2.
// Generated from identification schema v20.

mod generated;

/// Value type of a matchable or settable property (spec 4.4); drives
/// `validate.rs`'s scalar type-checking (`ValueTypeMismatch`,
/// `NotStringProperty`) for both `match` conditions and `changes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropType {
    /// String value (language codes, track names, codec ids, ...).
    String,
    /// Boolean flag (`default_track`, `forced_track`, ...).
    Boolean,
    /// Whole-number value (track `id`, `audio_channels`, ...).
    Integer,
    /// Floating-point value (`min_luminance`, projection pose angles, ...).
    Float,
}

/// Looks up a matchable property's type (spec 4.4): the build-time
/// generated identification schema fields (`generated::MATCHABLE_PROPERTIES`)
/// plus the `codec_kind` virtual property, a curated alias over `codec_id`
/// prefixes ([`codec_kind_prefixes`]) and therefore always `String`-typed.
/// `None` means the name is not in the capability model at all, the
/// config-time `UnknownProperty` condition (spec 5.2).
pub fn matchable_type(name: &str) -> Option<PropType> {
    if name == "codec_kind" {
        return Some(PropType::String);
    }
    generated::MATCHABLE_PROPERTIES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, t)| *t)
}

/// (profile name, value type, mkvmerge option) - spec 4.4 table.
pub static SETTABLE: &[(&str, PropType, &str)] = &[
    ("language", PropType::String, "--language"),
    ("track_name", PropType::String, "--track-name"),
    ("default_track", PropType::Boolean, "--default-track-flag"),
    ("forced_track", PropType::Boolean, "--forced-display-flag"),
    (
        "flag_hearing_impaired",
        PropType::Boolean,
        "--hearing-impaired-flag",
    ),
    (
        "flag_visual_impaired",
        PropType::Boolean,
        "--visual-impaired-flag",
    ),
    ("flag_commentary", PropType::Boolean, "--commentary-flag"),
    ("flag_original", PropType::Boolean, "--original-flag"),
    ("enabled_track", PropType::Boolean, "--track-enabled-flag"),
    ("sub_charset", PropType::String, "--sub-charset"),
];

/// Looks up a settable (`changes`) property's type and mkvmerge option
/// (spec 4.4 table). `None` means the name is not in [`SETTABLE`], the
/// config-time `UnknownSettableProperty` condition; `codec_kind` is
/// matchable-only and never resolves here.
pub fn settable(name: &str) -> Option<(PropType, &'static str)> {
    SETTABLE
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, t, o)| (*t, *o))
}

/// codec_kind aliases -> codec_id prefixes (prefix match at plan time).
pub static CODEC_KINDS: &[(&str, &[&str])] = &[
    ("srt", &["S_TEXT/UTF8"]),
    ("ass", &["S_TEXT/ASS", "S_TEXT/SSA"]),
    ("pgs", &["S_HDMV/PGS"]),
    ("vobsub", &["S_VOBSUB"]),
    ("webvtt", &["S_TEXT/WEBVTT"]),
    ("aac", &["A_AAC"]),
    ("ac3", &["A_AC3"]),
    ("eac3", &["A_EAC3"]),
    ("dts", &["A_DTS"]),
    ("truehd", &["A_TRUEHD"]),
    ("flac", &["A_FLAC"]),
    ("opus", &["A_OPUS"]),
    ("mp3", &["A_MPEG/L3"]),
    ("h264", &["V_MPEG4/ISO/AVC"]),
    ("h265", &["V_MPEGH/ISO/HEVC"]),
    ("av1", &["V_AV1"]),
    ("vp9", &["V_VP9"]),
];

/// Resolves a `codec_kind` alias (e.g. `srt`, `h264`) to the `codec_id`
/// prefixes it expands to at match time (spec 4.4). `None` if `kind` is not
/// a curated alias in [`CODEC_KINDS`].
pub fn codec_kind_prefixes(kind: &str) -> Option<&'static [&'static str]> {
    CODEC_KINDS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, p)| *p)
}

/// Attachment match properties (spec 4.9); not part of the track schema.
pub static ATTACHMENT_PROPERTIES: &[(&str, PropType)] = &[
    ("content_type", PropType::String),
    ("description", PropType::String),
    ("file_name", PropType::String),
    ("id", PropType::Integer),
    ("size", PropType::Integer),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matchable_types_from_generated_table() {
        assert_eq!(matchable_type("language"), Some(PropType::String));
        assert_eq!(matchable_type("forced_track"), Some(PropType::Boolean));
        assert_eq!(matchable_type("audio_channels"), Some(PropType::Integer));
        assert_eq!(matchable_type("type"), Some(PropType::String));
        assert_eq!(matchable_type("no_such_property"), None);
    }

    #[test]
    fn codec_kind_is_virtual_matchable() {
        assert_eq!(matchable_type("codec_kind"), Some(PropType::String));
        assert!(codec_kind_prefixes("srt").unwrap().contains(&"S_TEXT/UTF8"));
        assert!(codec_kind_prefixes("pgs").unwrap().contains(&"S_HDMV/PGS"));
        assert!(
            codec_kind_prefixes("h264")
                .unwrap()
                .contains(&"V_MPEG4/ISO/AVC")
        );
        assert!(codec_kind_prefixes("nope").is_none());
    }

    #[test]
    fn settable_maps_to_mkvmerge_options() {
        // Spec 4.4 table.
        const EXPECTED: &[(&str, PropType, &str)] = &[
            ("language", PropType::String, "--language"),
            ("track_name", PropType::String, "--track-name"),
            ("default_track", PropType::Boolean, "--default-track-flag"),
            ("forced_track", PropType::Boolean, "--forced-display-flag"),
            (
                "flag_hearing_impaired",
                PropType::Boolean,
                "--hearing-impaired-flag",
            ),
            (
                "flag_visual_impaired",
                PropType::Boolean,
                "--visual-impaired-flag",
            ),
            ("flag_commentary", PropType::Boolean, "--commentary-flag"),
            ("flag_original", PropType::Boolean, "--original-flag"),
            ("enabled_track", PropType::Boolean, "--track-enabled-flag"),
            ("sub_charset", PropType::String, "--sub-charset"),
        ];
        assert_eq!(EXPECTED.len(), SETTABLE.len());
        for &(name, ty, option) in EXPECTED {
            assert_eq!(settable(name), Some((ty, option)), "mismatch for {name}");
        }
        assert_eq!(settable("codec_kind"), None); // matchable only, never settable
    }

    #[test]
    fn attachment_properties_are_defined() {
        assert!(
            ATTACHMENT_PROPERTIES
                .iter()
                .any(|(n, t)| *n == "content_type" && *t == PropType::String)
        );
    }
}
