//! mkvtoolnix capability model (spec 4.4 / 9). Matchable properties are
//! generated from the identification schema; settable properties and
//! codec_kind aliases are curated here. Runtime queries (--list-types,
//! --list-languages, --version) arrive in Plan 2.
// Generated from identification schema v20.

mod generated;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropType {
    String,
    Boolean,
    Integer,
    Float,
}

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
        assert_eq!(
            settable("track_name"),
            Some((PropType::String, "--track-name"))
        );
        assert_eq!(
            settable("default_track"),
            Some((PropType::Boolean, "--default-track-flag"))
        );
        assert_eq!(
            settable("forced_track"),
            Some((PropType::Boolean, "--forced-display-flag"))
        );
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
