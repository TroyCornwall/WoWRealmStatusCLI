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

use clap::{Parser, Subcommand};

mod battlenet_api;
mod browser_auth;
mod colors;
mod config;

// ── CLI schema ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "WoWRealmStatusCLI", about = "WoW retail realm status — automated Battle.net login")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query realm list from Battle.net API (shows realm status during maintenance)
    Realms {
        /// Region (us, eu, kr, tw)
        #[arg(short, long, default_value = "us")]
        region: String,
        /// Locale (e.g., en_US, en_GB)
        #[arg(short, long, default_value = "en_US")]
        locale: String,
        /// Show detailed debug logging
        #[arg(short, long)]
        verbose: bool,
    },
}

// ── entry point ───────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Realms { region, locale, verbose } => {
            // Set verbose flag globally
            std::env::set_var("WOWRE_VERBOSE", if verbose { "1" } else { "0" });
            use rpassword::read_password;
            use std::io::{self, Write};

            let config_store = config::ConfigStore::default();
            let config = config_store.load();

            // Get username from config or prompt
            let username = if let Some(user) = config.username.clone() {
                user
            } else {
                print!("Battle.net username: ");
                io::stdout().flush()?;
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                user_input.trim().to_string()
            };

            let password = if let Some(pw) = config.password {
                pw
            } else {
                print!("Password: ");
                io::stdout().flush()?;
                read_password()?
            };

            println!("\n{}", colors::bold("  Automated Browser Login"));
            println!("  A Chrome window will open - please complete any 2FA if prompted");
            println!();

            let sso_token = match browser_auth::get_sso_token_with_retry(&username, &password, &region, 2) {
                Ok(token) => {
                    println!("{} Successfully obtained SSO token!", colors::green("✓"));
                    token
                }
                Err(e) => {
                    eprintln!("\n{} Failed to get SSO token: {}", colors::red("[error]"), e);
                    std::process::exit(1);
                }
            };

            println!("\n{}", colors::bold("  wowre realms"));
            println!("  {}", "─".repeat(70));
            println!();

            match battlenet_api::query_realm_status(&sso_token, &region, &locale) {
                Ok(realms) => {
                    if realms.is_empty() {
                        println!("  No realms returned.");
                        return Ok(());
                    }

                    println!("  {:<30} {:<12} {:<10} {}", "Realm Name", "Population", "Status", "Queue");
                    println!("  {} {} {} {}", "─".repeat(30), "─".repeat(12), "─".repeat(10), "─".repeat(5));

                    for realm in &realms {
                        let pop_label = format!("{:?}", realm.population.pop_type);
                        let pop_colored = match realm.population.pop_type {
                            battlenet_api::RealmPopulation::Low => colors::green(&pop_label),
                            battlenet_api::RealmPopulation::Medium => colors::yellow(&pop_label),
                            battlenet_api::RealmPopulation::High => colors::red(&pop_label),
                            battlenet_api::RealmPopulation::Full => colors::red(&pop_label),
                            battlenet_api::RealmPopulation::Recommended => colors::cyan(&pop_label),
                        };

                        let status_label = format!("{:?}", realm.status.status_type);
                        let status_colored = match realm.status.status_type {
                            battlenet_api::RealmStatusType::Up => colors::green(&status_label),
                            battlenet_api::RealmStatusType::Down => colors::red(&status_label),
                        };

                        let queue = if realm.has_queue {
                            colors::yellow("Yes")
                        } else {
                            colors::gray("No")
                        };

                        println!(
                            "  {:<30} {} {} {}",
                            realm.name, pop_colored, status_colored, queue
                        );
                    }
                    println!();
                    println!("  {} {} realms", colors::bold("Total:"), realms.len());
                }
                Err(e) => {
                    eprintln!("\n{} {}", colors::red("[error]"), e);
                    eprintln!("\nTip: Make sure your SSO token is fresh (they expire quickly).");
                    eprintln!("     Get a new one at: https://{}.account.battle.net/login/en/?ref=localhost", region);
                    std::process::exit(1);
                }
            }
        }

    }

    Ok(())
}
