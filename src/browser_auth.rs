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

use headless_chrome::{Browser, LaunchOptions};
use std::error::Error;
use std::time::Duration;

/// Print debug message only if verbose mode is enabled
macro_rules! debug {
    ($($arg:tt)*) => {
        if std::env::var("WOWRE_VERBOSE").unwrap_or_default() == "1" {
            eprintln!($($arg)*);
        }
    };
}

/// Automate Battle.net login using Chrome to get SSO token
pub fn get_sso_token_interactive(
    username: &str,
    password: &str,
    region: &str,
) -> Result<String, Box<dyn Error>> {
    debug!("[browser] Launching Chrome...");

    let browser = Browser::new(LaunchOptions {
        headless: false, // Show browser window so user can see what's happening
        window_size: Some((1200, 900)),
        ..Default::default()
    })?;

    let tab = browser.new_tab()?;

    // Navigate to login page
    let login_url = format!("https://{}.account.battle.net/login/en/?ref=localhost", region);
    debug!("[browser] Navigating to {}...", login_url);
    tab.navigate_to(&login_url)?;
    tab.wait_until_navigated()?;

    // Wait for login form to load
    debug!("[browser] Waiting for login form...");
    std::thread::sleep(Duration::from_secs(2));

    // Fill in username
    debug!("[browser] Entering username...");
    let username_selector = "input[type='email'], input[name='accountName']";
    if let Ok(elem) = tab.wait_for_element(username_selector) {
        elem.click()?;
        elem.type_into(username)?;

        // Press Enter to advance to next field/step
        debug!("[browser] Submitting username (pressing Enter)...");
        tab.press_key("Enter")?;
        std::thread::sleep(Duration::from_secs(2));
    } else {
        return Err("Could not find username field".into());
    }

    // Wait for password field to appear (multi-step form)
    debug!("[browser] Waiting for password field...");
    std::thread::sleep(Duration::from_secs(1));

    // Fill in password
    debug!("[browser] Entering password...");
    let password_selector = "input[type='password']";
    if let Ok(elem) = tab.wait_for_element_with_custom_timeout(password_selector, Duration::from_secs(10)) {
        elem.click()?;
        std::thread::sleep(Duration::from_millis(500));
        elem.type_into(password)?;

        // Press Enter to submit
        debug!("[browser] Submitting password (pressing Enter)...");
        tab.press_key("Enter")?;
    } else {
        // Try clicking a "Continue" or "Next" button if password field not visible
        debug!("[browser] Password field not found, looking for Continue button...");
        let continue_selectors = vec![
            "button[type='submit']",
            "button:contains('Continue')",
            "button:contains('Next')",
            ".submit-button",
        ];

        let mut found_button = false;
        for selector in continue_selectors {
            if let Ok(elem) = tab.find_element(selector) {
                debug!("[browser] Found button, clicking...");
                elem.click()?;
                found_button = true;
                std::thread::sleep(Duration::from_secs(2));
                break;
            }
        }

        if !found_button {
            return Err("Could not find password field or continue button".into());
        }

        // Try password field again
        if let Ok(elem) = tab.wait_for_element_with_custom_timeout(password_selector, Duration::from_secs(5)) {
            elem.click()?;
            std::thread::sleep(Duration::from_millis(500));
            elem.type_into(password)?;
            tab.press_key("Enter")?;
        } else {
            return Err("Could not find password field after clicking continue".into());
        }
    }

    debug!("[browser] Waiting for authentication...");
    println!("If you have 2FA enabled, please enter your code in the browser.");

    // Wait for redirect with ST= in URL (up to 60 seconds for 2FA)
    let mut sso_token: Option<String> = None;
    for _ in 0..60 {
        std::thread::sleep(Duration::from_secs(1));

        let url = tab.get_url();
        debug!("[browser] Current URL: {}", url);

        // Look for ST= parameter
        if url.contains("ST=") {
            if let Some(start) = url.find("ST=") {
                let token_start = start + 3;
                let token_end = url[token_start..]
                    .find('&')
                    .map(|i| token_start + i)
                    .unwrap_or(url.len());

                sso_token = Some(url[token_start..token_end].to_string());
                debug!("[browser] ✓ Got SSO token!");
                break;
            }
        }

        // Also check for error messages
        if url.contains("error") {
            return Err("Login failed - check credentials".into());
        }
    }

    if let Some(token) = sso_token {
        debug!("[browser] Closing browser...");
        // Keep browser open for a moment so user can see success
        std::thread::sleep(Duration::from_secs(2));
        Ok(token)
    } else {
        Err("Timeout waiting for SSO token - did you complete 2FA?".into())
    }
}

/// Get SSO token with automatic retry on failure
pub fn get_sso_token_with_retry(
    username: &str,
    password: &str,
    region: &str,
    max_attempts: u32,
) -> Result<String, Box<dyn Error>> {
    for attempt in 1..=max_attempts {
        debug!("\n[attempt {}/{}] Trying to get SSO token...", attempt, max_attempts);

        match get_sso_token_interactive(username, password, region) {
            Ok(token) => return Ok(token),
            Err(e) => {
                debug!("[error] Attempt {} failed: {}", attempt, e);
                if attempt < max_attempts {
                    debug!("[retry] Waiting 2 seconds before retry...");
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }

    Err(format!("Failed to get SSO token after {} attempts", max_attempts).into())
}
