use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GoConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub not_capable_style: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for GoConfig<'_> {
    fn default() -> Self {
        Self {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐹 ",
            style: "bold cyan",
            disabled: false,
            not_capable_style: "bold red",
            detect_extensions: vec!["go"],
            detect_files: vec![
                "go.mod",
                "go.sum",
                "go.work",
                "glide.yaml",
                "Gopkg.yml",
                "Gopkg.lock",
                ".go-version",
            ],
            detect_folders: vec!["Godeps"],
        }
    }
}
