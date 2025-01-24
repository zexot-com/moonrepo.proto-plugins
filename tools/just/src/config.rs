#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct JustPluginConfig {
    pub dist_url: String,
}

impl Default for JustPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://github.com/casey/just/releases/download/{version}/{file}".into(),
        }
    }
}
