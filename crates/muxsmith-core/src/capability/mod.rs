//! mkvtoolnix capability model (spec 4.4 / 9). Matchable properties are
//! generated from the identification schema; settable properties and
//! codec_kind aliases are curated here. Runtime queries (--list-types,
//! --list-languages, --version) arrive in Plan 2.
// Generated from identification schema v20.

mod generated;
pub mod runtime;

/// The mkvmerge identification-output schema version this build was generated
/// against (spec 9). A file whose `-J` `identification_format_version` exceeds
/// this triggers the `UnknownPropertySkew` warning (untyped forward matching).
pub const PINNED_IDENTIFICATION_FORMAT_VERSION: u64 = 20;

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

/// The closed set of `type` values mkvmerge reports for a track. Curated
/// rather than generated: the upstream identification schema (v20) types
/// `type` as a plain string with no enum, and mkvmerge's track types are
/// long-stable. Verified against `mkvmerge -J` output, where `track.type` is
/// exactly one of these.
pub static TYPE_VALUES: &[&str] = &["audio", "buttons", "subtitles", "video"];

/// The closed set of legal values for a matchable property, or `None` if the
/// property is open-valued (free text, numbers) or has a runtime-only domain.
/// Backs the config-time `InvalidPropertyValue` check (spec 4.4, D2): `type`
/// and `codec_kind` are curated closed sets; `language`'s domain needs
/// `mkvmerge --list-languages` and is validated at plan time, so it is `None`
/// here.
pub fn matchable_domain(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "type" => Some(TYPE_VALUES),
        "codec_kind" => Some(CODEC_KIND_NAMES),
        _ => None,
    }
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

/// The curated `codec_kind` alias names ([`CODEC_KINDS`] keys), the closed
/// domain of the `codec_kind` virtual property. Kept in sync with
/// `CODEC_KINDS` by the `codec_kind_domain_matches_kinds` test.
pub static CODEC_KIND_NAMES: &[&str] = &[
    "srt", "ass", "pgs", "vobsub", "webvtt", "aac", "ac3", "eac3", "dts", "truehd", "flac", "opus",
    "mp3", "h264", "h265", "av1", "vp9",
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
    fn value_domains_are_closed_for_type_and_codec_kind() {
        let type_domain = matchable_domain("type").expect("type has a closed domain");
        assert!(type_domain.contains(&"video"));
        assert!(type_domain.contains(&"audio"));
        assert!(type_domain.contains(&"subtitles"));
        let ck = matchable_domain("codec_kind").expect("codec_kind has a closed domain");
        assert!(ck.contains(&"srt"));
        assert!(ck.contains(&"h264"));
        assert_eq!(matchable_domain("track_name"), None);
        assert_eq!(matchable_domain("language"), None); // validated at plan time
    }

    #[test]
    fn codec_kind_domain_matches_kinds() {
        let from_kinds: Vec<&str> = CODEC_KINDS.iter().map(|(k, _)| *k).collect();
        assert_eq!(from_kinds, CODEC_KIND_NAMES);
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
