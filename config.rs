
// manage user config file

use std::path::{PathBuf, Path};

use super::common::{BoxResult};

#[derive(Debug)]
pub struct UserConfig {
    pub site_url: String,
    pub config_dir: PathBuf
}

/*
pub fn load_config() -> BoxResult<UserConfig> {


    Err(())
}
*/
