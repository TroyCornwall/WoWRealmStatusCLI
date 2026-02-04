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
use std::error::Error;

// Battle.net launcher client ID (from packet capture)
const CLIENT_ID: &str = "33dad602838b47bfa5ca03adaebac54c";

/// Print debug message only if verbose mode is enabled
macro_rules! debug {
    ($($arg:tt)*) => {
        if std::env::var("WOWRE_VERBOSE").unwrap_or_default() == "1" {
            eprintln!($($arg)*);
        }
    };
}

// ── OAuth2 SSO Token Exchange ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

/// Exchange an SSO token (ST=...) for a Bearer access token
/// SSO token can be obtained by visiting: https://us.account.battle.net/login/en/?ref=localhost
/// After login, extract the ST= parameter from the URL
fn exchange_sso_token(sso_token: &str, region: &str) -> Result<String, Box<dyn Error>> {
    let oauth_url = format!("https://{}.battle.net/oauth/sso", region);

    debug!("[debug] Exchanging SSO token for Bearer token...");
    debug!("[debug] OAuth endpoint: {}", oauth_url);

    let scopes = vec![
        "featuredshop.composition.basic",
        "commerce.catalog.basic",
        "commerce.purchase",
        "commerce.profile.basic",
        "commerce.orders.basic:read",
        "commerce.virtualcurrency.basic",
    ].join(" ");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&oauth_url)
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", &scopes),
            ("token", sso_token),
            ("grant_type", "client_sso"),
        ])
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text()?;
        return Err(format!("OAuth SSO exchange failed ({}): {}", status, body).into());
    }

    let token_data: TokenResponse = response.json()?;
    debug!("[debug] Got Bearer token (expires in {}s)", token_data.expires_in);

    Ok(token_data.access_token)
}

// ── Realm Status API ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct RealmStatus {
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub has_queue: bool,
    #[serde(default)]
    pub status: StatusInfo,
    #[serde(default)]
    pub population: PopulationInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct StatusInfo {
    #[serde(rename = "type")]
    pub status_type: RealmStatusType,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PopulationInfo {
    #[serde(rename = "type")]
    pub pop_type: RealmPopulation,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum RealmStatusType {
    #[default]
    Up,
    Down,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum RealmPopulation {
    #[default]
    Low,
    Medium,
    High,
    Full,
    Recommended,
}

#[derive(Debug, Deserialize)]
struct ConnectedRealm {
    has_queue: bool,
    status: StatusInfo,
    population: PopulationInfo,
    realms: Vec<RealmRef>,
}

#[derive(Debug, Deserialize)]
struct RealmRef {
    name: LocalizedString,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct LocalizedString {
    en_US: String,
}

#[derive(Debug, Deserialize)]
struct ConnectedRealmsIndex {
    connected_realms: Vec<ConnectedRealmHref>,
}

#[derive(Debug, Deserialize)]
struct ConnectedRealmHref {
    href: String,
}

pub fn query_realm_status(
    sso_token: &str,
    region: &str,
    locale: &str,
) -> Result<Vec<RealmStatus>, Box<dyn Error>> {
    // 1. Exchange SSO token for Bearer token
    let bearer_token = exchange_sso_token(sso_token, region)?;

    // 2. Get connected realms index
    let api_host = format!("{}.api.blizzard.com", region);
    let index_url = format!(
        "https://{}/data/wow/connected-realm/index?namespace=dynamic-{}&locale={}",
        api_host, region, locale
    );

    debug!("[debug] Fetching connected realms index...");

    let client = reqwest::blocking::Client::new();
    let index_response = client
        .get(&index_url)
        .bearer_auth(&bearer_token)
        .send()?;

    if !index_response.status().is_success() {
        let status = index_response.status();
        let body = index_response.text()?;
        return Err(format!("Connected realms index request failed ({}): {}", status, body).into());
    }

    let index: ConnectedRealmsIndex = index_response.json()?;
    debug!("[debug] Found {} connected realm groups", index.connected_realms.len());

    // 3. Fetch each connected realm group to get individual realm status
    let mut all_realms = Vec::new();

    for (i, cr_ref) in index.connected_realms.iter().enumerate() {
        if i % 10 == 0 {
            debug!("[debug] Fetching realm group {}/{}...", i + 1, index.connected_realms.len());
        }

        let cr_response = client
            .get(&cr_ref.href)
            .bearer_auth(&bearer_token)
            .send()?;

        if !cr_response.status().is_success() {
            debug!("[warn] Failed to fetch {}: {}", cr_ref.href, cr_response.status());
            continue;
        }

        let body = cr_response.text()?;

        // Debug: print first response to see structure
        if i == 0 {
            debug!("[debug] First realm response sample:\n{}\n", &body[..body.len().min(500)]);
        }

        let connected_realm: ConnectedRealm = match serde_json::from_str(&body) {
            Ok(cr) => cr,
            Err(e) => {
                debug!("[warn] Failed to parse connected realm: {}", e);
                if i < 2 {
                    debug!("[debug] Response body: {}", &body[..body.len().min(200)]);
                }
                continue;
            }
        };

        // Each connected realm can have multiple individual realms
        for realm_ref in connected_realm.realms {
            all_realms.push(RealmStatus {
                name: realm_ref.name.en_US,
                slug: realm_ref.slug,
                has_queue: connected_realm.has_queue,
                status: connected_realm.status.clone(),
                population: connected_realm.population.clone(),
            });
        }
    }

    debug!("[debug] Total realms fetched: {}", all_realms.len());
    Ok(all_realms)
}
