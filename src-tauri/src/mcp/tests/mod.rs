use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tempfile::TempDir;

pub static ENV_LOCK: Mutex<()> = Mutex::new(());

pub struct EnvGuard {
    saved: Vec<(&'static str, Option<OsString>)>,
}

impl EnvGuard {
    pub fn new(keys: &[&'static str]) -> Self {
        let saved = keys.iter().map(|key| (*key, env::var_os(*key))).collect();
        Self { saved }
    }

    pub fn set_path(&self, key: &'static str, path: &Path) {
        env::set_var(key, path);
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            match value {
                Some(v) => env::set_var(key, v),
                None => env::remove_var(key),
            }
        }
    }
}

pub fn set_temp_home(temp_dir: &TempDir) -> EnvGuard {
    let env_guard = EnvGuard::new(&["HOME", "USERPROFILE", "APPDATA", "XDG_CONFIG_HOME"]);
    env_guard.set_path("HOME", temp_dir.path());
    env_guard.set_path("USERPROFILE", temp_dir.path());

    let appdata = temp_dir.path().join("AppData");
    let xdg_config = temp_dir.path().join(".config");
    fs::create_dir_all(&appdata).unwrap();
    fs::create_dir_all(&xdg_config).unwrap();
    env_guard.set_path("APPDATA", &appdata);
    env_guard.set_path("XDG_CONFIG_HOME", &xdg_config);

    env_guard
}

mod fixtures;
mod get_config_tests;
mod update_config_tests;
