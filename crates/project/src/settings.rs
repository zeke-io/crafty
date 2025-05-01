use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SourceFile {
    pub path: PathBuf,
    pub target: PathBuf,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectSettings {
    #[serde(default)]
    pub jvm_options: Vec<String>,
    #[serde(default)]
    pub server_args: Vec<String>,
    #[serde(default)]
    pub files: Vec<SourceFile>,
}

impl ProjectSettings {
    pub fn parse_to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let buffer = serde_yaml::to_string(self)?;
        Ok(buffer.into_bytes())
    }
}

pub(crate) fn load_settings<P: AsRef<Path>>(
    path: P,
    profile_name: Option<String>,
) -> anyhow::Result<ProjectSettings> {
    fn inner(path: PathBuf) -> anyhow::Result<ProjectSettings> {
        let settings_file = fs::read_to_string(&path).context(format!(
            "Could not find settings file \"{}\"",
            &path.display()
        ))?;

        let settings: ProjectSettings = serde_yaml::from_str(&settings_file).context(format!(
            "The settings file at \"{}\" is invalid.",
            path.display()
        ))?;

        Ok(settings)
    }

    let path = path.as_ref();

    if let Some(profile_name) = profile_name {
        match inner(path.join(format!("settings.{}.yml", profile_name))) {
            Ok(settings) => return Ok(settings),
            Err(err) => log::warn!("{}\nAttempting to load \"settings.yml\" file...", err),
        }
    }

    inner(path.join("settings.yml"))
}
