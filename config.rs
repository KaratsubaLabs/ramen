
// manage user config file

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf, Path};

use super::common;
use super::common::{BoxResult};
use super::error::{SimpleError};

#[derive(Debug)]
pub struct UserConfig {
    pub site_url: String,
    pub config_dir: PathBuf,
    pub content_path: PathBuf, // location of raw anime files
    pub static_path: PathBuf, // location of outputted static generated files
}

#[derive(Debug)]
pub struct UserConfigBuilder {
    pub site_url: Option<String>,
    pub config_dir: PathBuf,
    pub content_path: Option<PathBuf>,
    pub static_path: Option<PathBuf>,
}

impl UserConfigBuilder {
    pub fn new(config_dir: &Path) -> UserConfigBuilder {
        UserConfigBuilder{
            site_url: None,
            config_dir: config_dir.to_path_buf(),
            content_path: None,
            static_path: None,
        }
    }

    pub fn site_url(mut self, site_url: String) -> Self {
        self.site_url = Some(site_url);
        self
    }
    pub fn content_path(mut self, content_path: &str) -> Self {
        // TODO support ~ expansion + error handling
        self.content_path = Some(PathBuf::from(content_path));
        self
    }
    pub fn static_path(mut self, static_path: &str) -> Self {
        self.static_path = Some(PathBuf::from(static_path));
        self
    }

    pub fn build(self) -> Option<UserConfig> {
        Some(UserConfig {
            site_url: self.site_url?,
            config_dir: self.config_dir,
            content_path: self.content_path?,
            static_path: self.static_path?,
        })
    }
}

pub fn load_config(config_dir: &str) -> BoxResult<UserConfig> {

    let path = PathBuf::from(config_dir);
    let file = File::open(&path.join(common::CONFIG_FILE_NAME))?;
    let reader = io::BufReader::new(file);

    let mut builder = UserConfigBuilder::new(&path);

    // TODO this is duplicated code
    for line in reader.lines() {
        let line = line?;
        let split = line.split_once("=");
        if split.is_none() { continue; }
        let mut split = split.unwrap();
        split = (split.0.trim(), split.1.trim());

        builder = match split.0 {
            "site_url" => builder.site_url(split.1.to_string()),
            "content_path" => builder.content_path(split.1),
            "static_path" => builder.static_path(split.1),
            _ => builder
        }
    }

    let user_config = builder.build().ok_or(SimpleError::new("failed to build user config"))?;

    println!("{:?}", user_config);
    
    Ok(user_config)
}

