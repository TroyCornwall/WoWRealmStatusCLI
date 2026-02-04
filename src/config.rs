// WoW Realm Status CLI - Query WoW realm status via Battle.net API
// Copyright (C) 2026
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
        }
    }
}

pub struct ConfigStore {
    dir: PathBuf,
}

impl ConfigStore {
    /// Default location: `$WOWRE_CONFIG_DIR` if set, otherwise `~/.config/wowre`.
    pub fn default() -> Self {
        let dir = std::env::var("WOWRE_CONFIG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
                PathBuf::from(home).join(".config").join("wowre")
            });
        Self { dir }
    }

    fn path(&self) -> PathBuf {
        self.dir.join("auth.json")
    }

    fn ensure(&self) {
        fs::create_dir_all(&self.dir).ok();
    }

    pub fn load(&self) -> AuthConfig {
        self.ensure();
        fs::read_to_string(self.path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
}
