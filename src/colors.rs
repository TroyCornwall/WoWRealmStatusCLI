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

/// ANSI colour helpers — zero external dependencies.

pub fn bold(s: &str)   -> String { format!("\x1b[1m{}\x1b[0m",  s) }
pub fn red(s: &str)    -> String { format!("\x1b[31m{}\x1b[0m", s) }
pub fn green(s: &str)  -> String { format!("\x1b[32m{}\x1b[0m", s) }
pub fn yellow(s: &str) -> String { format!("\x1b[33m{}\x1b[0m", s) }
pub fn cyan(s: &str)   -> String { format!("\x1b[36m{}\x1b[0m", s) }
pub fn gray(s: &str)   -> String { format!("\x1b[90m{}\x1b[0m", s) }
