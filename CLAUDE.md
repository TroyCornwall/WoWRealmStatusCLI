# WoWRealmStatusCLI

WoW retail realm status checker (Rust). Queries Battle.net API for realm
status using automated OAuth2 authentication with Chrome browser automation.

## Build & run

```sh
cargo build                    # debug build
cargo build --release          # optimized build
cargo run -- realms            # query realm status (US region)
cargo run -- realms --verbose  # with debug logging
cargo test                     # unit tests
```

## Commands

```sh
# Basic usage (US region, en_US locale)
cargo run -- realms

# EU region with German locale
cargo run -- realms --region eu --locale de_DE

# Enable verbose debug output
cargo run -- realms --verbose

# All options
cargo run -- realms --region <us|eu|kr|tw> --locale <locale> --verbose
```

## Authentication Flow

The tool uses Battle.net's OAuth2 SSO flow:

1. Prompt for Battle.net username (if not in config)
2. Prompt for password (if not in config)
3. Open Chrome browser to `{region}.account.battle.net/login`
4. Automate form filling:
   - Enter username, press Enter to advance
   - Wait for password field (multi-step form)
   - Enter password, press Enter to submit
5. Wait for user to complete 2FA (if enabled)
6. Extract SSO token from redirect URL (`ST=...` parameter)
7. Exchange SSO token for OAuth Bearer token via `POST {region}.battle.net/oauth/sso`
8. Query Battle.net Game Data API for connected realm status

## API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `https://{region}.battle.net/oauth/sso` | Exchange SSO token for Bearer token |
| `https://{region}.api.blizzard.com/data/wow/connected-realm/index` | Get list of connected realm groups |
| `https://{region}.api.blizzard.com/data/wow/connected-realm/{id}` | Get individual connected realm details |

**Client ID**: `33dad602838b47bfa5ca03adaebac54c` (from Battle.net launcher packet capture)

**OAuth Scopes**:
- `featuredshop.composition.basic`
- `commerce.catalog.basic`
- `commerce.purchase`
- `commerce.profile.basic`
- `commerce.orders.basic:read`
- `commerce.virtualcurrency.basic`

## Configuration

Save credentials to skip prompts: `~/.config/wowre/auth.json`

```json
{
  "username": "your_battle_net_email@example.com",
  "password": "your_password"
}
```

**Security Warning**: Credentials stored in plain text. Only use on trusted systems.
Never commit `auth.json` to git (already in `.gitignore`).

## Output Format

```
Realm Name                     Population   Status     Queue
────────────────────────────── ──────────── ────────── ─────
Stormrage                      Full         Up         No
Area 52                        Full         Up         No
Tichondrius                    Full         Up         No
Illidan                        Full         Up         No
```

**Population levels**: Low (green) | Medium (yellow) | High (red) | Full (red) | Recommended (cyan)

**Status**: Up (green) | Down (red)

**Queue**: Yes (yellow) | No (gray)

## Data Structures

### Connected Realms
Battle.net groups realms into "connected realms" that share:
- Population level
- Queue status
- Online/offline status

Each connected realm contains multiple individual realms with unique names/slugs.

### API Response Format
```json
{
  "has_queue": false,
  "status": { "type": "UP" },
  "population": { "type": "MEDIUM" },
  "realms": [
    {
      "name": { "en_US": "Stormrage" },
      "slug": "stormrage"
    }
  ]
}
```

## Debug Mode

Set `WOWRE_VERBOSE=1` or use `--verbose` flag to see:
- Browser automation steps
- OAuth token exchange details
- API request URLs
- Response parsing progress
- Connected realm fetch count

```sh
cargo run -- realms --verbose
```

## Architecture

- `src/main.rs`         — CLI wiring (clap), credential prompts, output formatting
- `src/browser_auth.rs` — Chrome automation for SSO token acquisition (headless_chrome)
- `src/battlenet_api.rs`— OAuth2 token exchange + Battle.net Game Data API queries
- `src/config.rs`       — Credential storage (~/.config/wowre/auth.json)
- `src/colors.rs`       — ANSI escape helpers for terminal output

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | Command-line argument parsing |
| `dotenv` | `.env` file support |
| `headless_chrome` | Chrome browser automation |
| `reqwest` | HTTP client for API calls |
| `rpassword` | Secure password input prompts |
| `serde` / `serde_json` | JSON serialization/deserialization |

## Known Issues

- Chrome must be installed and in PATH
- SSO tokens expire quickly (handled by re-authenticating on each run)
- Multi-step login form timing may vary by region
- 2FA requires manual completion in browser window

## Development Notes

### Adding New Regions

Update region validation in `src/main.rs`:
```rust
#[arg(short, long, default_value = "us")]
region: String,  // Add new region to help text
```

### Adding New Locales

Pass any valid Battle.net locale (e.g., `en_GB`, `de_DE`, `fr_FR`, `es_ES`, `pt_BR`, `ru_RU`, `ko_KR`, `zh_TW`, `zh_CN`)

### Extending API Queries

See `src/battlenet_api.rs` for OAuth token handling and API request patterns. All API calls require Bearer token authentication.
