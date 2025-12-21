/*
 * Copyright (c) 2025 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::magic::XDG_SUBDIRECTORY;
use std::env::VarError;
use std::error::Error;
use std::fs::DirBuilder;
#[cfg(target_family = "unix")]
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

// https://specifications.freedesktop.org/basedir-spec/0.8/

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct XDGDirectories {
    config: PathBuf,
    data: PathBuf,
    state: PathBuf,
}

impl XDGDirectories {
    pub fn new(subdirectory: Option<&'static str>) -> Result<Self> {
        let config = Self::env("BLACKNET_CONFIGDIR").or_else(|_| Self::cfg(subdirectory))?;
        Self::mkdirs(&config)?;
        let data = Self::env("BLACKNET_DATADIR").or_else(|_| Self::dat(subdirectory))?;
        Self::mkdirs(&data)?;
        let state = Self::env("BLACKNET_STATEDIR").or_else(|_| Self::stat(subdirectory))?;
        Self::mkdirs(&state)?;
        Ok(Self {
            config,
            data,
            state,
        })
    }

    pub fn config(&self) -> &Path {
        &self.config
    }

    pub fn data(&self) -> &Path {
        &self.data
    }

    pub fn state(&self) -> &Path {
        &self.state
    }

    fn mkdirs(path: &Path) -> Result<()> {
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        #[cfg(target_family = "unix")]
        builder.mode(0o700);
        Ok(builder.create(path)?)
    }

    fn home() -> Result<PathBuf> {
        #[cfg(target_family = "unix")]
        let var = "HOME";
        #[cfg(target_os = "windows")]
        let var = "USERPROFILE";
        Self::env(var)
    }

    fn env(name: &'static str) -> Result<PathBuf> {
        match std::env::var(name) {
            Ok(val) => Ok(PathBuf::from(val)),
            Err(VarError::NotUnicode(val)) => Ok(PathBuf::from(val)),
            Err(VarError::NotPresent) => {
                Err(format!("Environment variable {name} is not set").into())
            }
        }
    }

    fn cfg(subdirectory: Option<&'static str>) -> Result<PathBuf> {
        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        let dir = Self::xdg("XDG_CONFIG_HOME", subdirectory, &[".config"]);
        #[cfg(target_os = "windows")]
        let dir = Self::win(subdirectory);
        #[cfg(target_os = "macos")]
        let dir = Self::mac(subdirectory);
        dir
    }

    fn dat(subdirectory: Option<&'static str>) -> Result<PathBuf> {
        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        let dir = Self::xdg("XDG_DATA_HOME", subdirectory, &[".local", "share"]);
        #[cfg(target_os = "windows")]
        let dir = Self::win(subdirectory);
        #[cfg(target_os = "macos")]
        let dir = Self::mac(subdirectory);
        dir
    }

    fn stat(subdirectory: Option<&'static str>) -> Result<PathBuf> {
        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        let dir = Self::xdg("XDG_STATE_HOME", subdirectory, &[".local", "state"]);
        #[cfg(target_os = "windows")]
        let dir = Self::win(subdirectory);
        #[cfg(target_os = "macos")]
        let dir = Self::mac(subdirectory);
        dir
    }

    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    fn xdg(
        base: &'static str,
        subdirectory: Option<&'static str>,
        fallback: &[&'static str],
    ) -> Result<PathBuf> {
        let base = Self::env(base);
        let mut dir = if base.as_ref().is_ok_and(|base| base.is_absolute()) {
            let mut dir = base?;
            dir.push(XDG_SUBDIRECTORY);
            dir
        } else {
            let mut dir = Self::home()?;
            fallback.iter().for_each(|d| dir.push(d));
            dir.push(XDG_SUBDIRECTORY);
            dir
        };
        if let Some(subdirectory) = subdirectory {
            dir.push(subdirectory)
        };
        Ok(dir)
    }

    #[cfg(target_os = "windows")]
    fn win(subdirectory: Option<&'static str>) -> Result<PathBuf> {
        let mut dir = Self::home()?;
        dir.push("AppData");
        dir.push("Local");
        dir.push(XDG_SUBDIRECTORY);
        if let Some(subdirectory) = subdirectory {
            dir.push(subdirectory)
        };
        Ok(dir)
    }

    #[cfg(target_os = "macos")]
    fn mac(subdirectory: Option<&'static str>) -> Result<PathBuf> {
        let mut dir = Self::home()?;
        dir.push("Library");
        dir.push("Application Support");
        dir.push(XDG_SUBDIRECTORY);
        if let Some(subdirectory) = subdirectory {
            dir.push(subdirectory)
        };
        Ok(dir)
    }
}
