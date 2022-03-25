use std::{
    borrow::Cow,
    error::Error,
    fmt::Display,
    fs::{self, File},
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};

use app_dirs2::{get_app_dir, get_data_root, AppDataType, AppDirsError, AppInfo};
use normpath::PathExt;
use serde::{Deserialize, Serialize};

const APP_INFO: AppInfo = AppInfo {
    name: "sc-util",
    author: "devnought",
};

#[derive(Debug)]
pub enum UtilError {
    ConfigPath(AppDirsError),
    Message(Cow<'static, str>),
    IoError(io::Error),
    Json(serde_json::error::Error),
    FsExtra(fs_extra::error::Error),
}

impl Error for UtilError {}
impl Display for UtilError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(m) => write!(f, "{m}"),
            Self::ConfigPath(e) => e.fmt(f),
            Self::IoError(e) => e.fmt(f),
            Self::Json(e) => e.fmt(f),
            Self::FsExtra(e) => e.fmt(f),
        }
    }
}

impl From<AppDirsError> for UtilError {
    fn from(e: AppDirsError) -> Self {
        Self::ConfigPath(e)
    }
}

impl From<String> for UtilError {
    fn from(s: String) -> Self {
        Self::Message(s.into())
    }
}

impl From<&'static str> for UtilError {
    fn from(s: &'static str) -> Self {
        Self::Message(s.into())
    }
}

impl From<io::Error> for UtilError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<serde_json::error::Error> for UtilError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::Json(e)
    }
}

impl From<fs_extra::error::Error> for UtilError {
    fn from(e: fs_extra::error::Error) -> Self {
        Self::FsExtra(e)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    root_path: PathBuf,
}

fn config_path() -> Result<PathBuf, AppDirsError> {
    get_app_dir(AppDataType::UserCache, &APP_INFO, "config.json")
}

fn config_file() -> Result<Config, UtilError> {
    let config_path = config_path()?;

    if !config_path.exists() {
        return Err("Configuration file was never initialized".into());
    }

    let file = File::open(&config_path)?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader::<_, Config>(reader)?;

    Ok(config)
}

fn validate_root_path(config: &Config, environment: &str) -> Result<PathBuf, UtilError> {
    let root_path = [config.root_path.as_path(), Path::new(environment)]
        .iter()
        .collect::<PathBuf>();

    if !root_path.exists() {
        return Err(format!("`{}` does not exist.", root_path.display()).into());
    }

    let root_path = PathBuf::from(root_path.normalize()?);

    if !root_path.starts_with(&config.root_path) {
        return Err(format!(
            "`{}` is outside of the configured Star Citizen root directory",
            root_path.display()
        )
        .into());
    }

    Ok(root_path)
}

pub fn view_config() -> Result<(), UtilError> {
    let config = config_file()?;

    println!("{}", config.root_path.display());

    Ok(())
}

pub fn set_config(root_path: &Path) -> Result<(), UtilError> {
    if !root_path.exists() {
        return Err(format!("The root path `{}` does not exist", root_path.display()).into());
    }

    let norm_root_path = root_path.normalize()?;
    let config_path = config_path()?;

    if !config_path.exists() {
        let p = &config_path
            .parent()
            .ok_or(UtilError::Message("Cannot determine user path".into()))?;
        fs::create_dir_all(p)?;
    }

    let config = Config {
        root_path: norm_root_path.into(),
    };

    let file = File::create(&config_path)?;
    serde_json::to_writer_pretty(file, &config)?;

    Ok(())
}

pub fn delete_shaders() -> Result<(), UtilError> {
    let local_shader_path_root = [
        get_data_root(AppDataType::UserCache)?.as_path(),
        Path::new("Star Citizen"),
    ]
    .iter()
    .collect::<PathBuf>();

    if local_shader_path_root.exists() {
        fs_extra::dir::remove(local_shader_path_root)?;
    }

    Ok(())
}

pub fn delete_user_folder(environment: &str) -> Result<(), UtilError> {
    let config = config_file()?;
    let user_path = [
        validate_root_path(&config, environment)?.as_path(),
        Path::new("USER"),
    ]
    .iter()
    .collect::<PathBuf>();

    if user_path.exists() {
        fs_extra::dir::remove(user_path)?;
    }

    Ok(())
}

pub fn create_cfg(environment: &str, overwrite: bool) -> Result<(), UtilError> {
    let config = config_file()?;
    let config_path = [
        validate_root_path(&config, environment)?.as_path(),
        Path::new("USER.cfg"),
    ]
    .iter()
    .collect::<PathBuf>();

    if !overwrite && config_path.exists() {
        return Err("`USER.cfg` already exists".into());
    }

    let mut file = File::create(&config_path)?;
    file.write_all(b"r_displaySessionInfo = 1\r\nr_displayInfo = 3\r\n")?;

    Ok(())
}
