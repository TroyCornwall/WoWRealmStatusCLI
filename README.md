# WoW Realm Status CLI

A command-line tool to check World of Warcraft realm status (population, online/offline, queue info) using automated Battle.net authentication.

## Features

- **Automated Browser Login**: Uses Chrome to securely authenticate with Battle.net
- **Real-time Realm Data**: Fetches current realm status directly from Battle.net API
- **2FA Support**: Handles Battle.net Authenticator (TOTP) automatically
- **Color-coded Output**: Easy-to-read status with color-coded population levels
- **Config Support**: Save credentials to avoid repeated login prompts

## Prerequisites

- Rust toolchain (latest stable)
- Google Chrome installed
- Battle.net account

## Installation

```bash
git clone <repository-url>
cd wowre
cargo build --release
```

The binary will be available at `target/release/wow-realm-status-cli`.

## Usage

### Basic Usage

```bash
cargo run -- realms
```

This will:
1. Prompt for your Battle.net username and password (if not saved in config)
2. Open a Chrome window for automated login
3. Handle 2FA if enabled on your account
4. Fetch and display all realm status information

### Command Options

```bash
cargo run -- realms [OPTIONS]

Options:
  -r, --region <REGION>    Region (us, eu, kr, tw) [default: us]
  -l, --locale <LOCALE>    Locale (e.g., en_US, en_GB) [default: en_US]
  -v, --verbose            Show detailed debug logging
  -h, --help               Print help
```

### Examples

```bash
# US region (default)
cargo run -- realms

# EU region with German locale
cargo run -- realms --region eu --locale de_DE

# Enable verbose debug output
cargo run -- realms --verbose
```

## Configuration

The tool can save your credentials to avoid repeated prompts.

**Config file location**: `~/.config/wowre/auth.json`

```json
{
  "username": "your-email@example.com",
  "password": "your-password"
}
```

> **Security Note**: The config file stores credentials in plain text. Only use this on trusted, secure systems. Never commit `auth.json` to git (it's already in `.gitignore`).

## Sample Output

```
  Automated Browser Login
  A Chrome window will open - please complete any 2FA if prompted

If you have 2FA enabled, please enter your code in the browser.
✓ Successfully obtained SSO token!

  wowre realms
  ──────────────────────────────────────────────────────────────────────

  Realm Name                     Population   Status     Queue
  ────────────────────────────── ──────────── ────────── ─────
  Azjol-Nerub                    High         Up         No
  Muradin                        High         Up         No
  Nordrassil                     High         Up         No
  Blackrock                      High         Up         No
  Khaz Modan                     High         Up         No
  Firetree                       Medium       Up         No
  Drak'Tharon                    Medium       Up         No
  Rivendare                      Medium       Up         No
  Vashj                          Medium       Up         No
  Spirestone                     Medium       Up         No
  Malorne                        Medium       Up         No
  Frostwolf                      Medium       Up         No
  Stormscale                     Medium       Up         No
  Runetotem                      Low          Up         No
  Uther                          Low          Up         No
  Draenor                        Low          Up         No
  Echo Isles                     Low          Up         No
  Aggramar                       Full         Up         No
  Fizzcrank                      Full         Up         No
  Ravencrest                     Low          Up         No
  Uldaman                        Low          Up         No
  Stormreaver                    Low          Up         No
  Elune                          Medium       Up         No
  Laughing Skull                 Medium       Up         No
  Auchindoun                     Medium       Up         No
  Cho'gall                       Medium       Up         No
  Gilneas                        Medium       Up         No
  Bleeding Hollow                High         Up         No
  Trollbane                      High         Up         No
  Grizzly Hills                  High         Up         No
  Malfurion                      High         Up         No
  Lothar                         High         Up         No
  Kael'thas                      High         Up         No
  Gnomeregan                     High         Up         No
  Moonrunner                     High         Up         No
  Ghostlands                     High         Up         No
  Vek'nilash                     Low          Up         No
  Nazgrel                        Low          Up         No
  Nesingwary                     Low          Up         No
  Sen'jin                        High         Up         No
  Dunemaul                       High         Up         No
  Maiev                          High         Up         No
  Bloodscalp                     High         Up         No
  Quel'dorei                     High         Up         No
  Boulderfist                    High         Up         No
  Stonemaul                      High         Up         No
  Argent Dawn                    Low          Up         No
  The Scryers                    Low          Up         No
  Dragonmaw                      High         Up         No
  Uldum                          High         Up         No
  Akama                          High         Up         No
  Korialstrasz                   High         Up         No
  Eldre'Thalas                   High         Up         No
  Mug'thol                       High         Up         No
  Antonidas                      High         Up         No
  Silvermoon                     High         Up         No
  Skywall                        High         Up         No
  Terenas                        High         Up         No
  Hydraxis                       High         Up         No
  Drak'thul                      High         Up         No
  Borean Tundra                  High         Up         No
  Mok'Nathal                     High         Up         No
  Shadowsong                     High         Up         No
  Eonar                          High         Up         No
  Skullcrusher                   High         Up         No
  Gul'dan                        High         Up         No
  Zuluhed                        High         Up         No
  Ursin                          High         Up         No
  Andorhal                       High         Up         No
  Black Dragonflight             High         Up         No
  Velen                          High         Up         No
  Scilla                         High         Up         No
  Llane                          Low          Up         No
  Arygos                         Low          Up         No
  Earthen Ring                   Low          Up         No
  Malygos                        High         Up         No
  Garona                         High         Up         No
  Lightning's Blade              High         Up         No
  Icecrown                       High         Up         No
  Onyxia                         High         Up         No
  Burning Blade                  High         Up         No
  Dragonblight                   Low          Up         No
  Fenris                         Low          Up         No
  Hellscream                     Full         Up         No
  Gorefiend                      Full         Up         No
  Spinebreaker                   Full         Up         No
  Zangarmarsh                    Full         Up         No
  Wildhammer                     Full         Up         No
  Eredar                         Full         Up         No
  Kilrogg                        Low          Up         No
  Winterhoof                     Low          Up         No
  Eitrigg                        Low          Up         No
  Shu'halo                       Low          Up         No
  Feathermoon                    Low          Up         No
  Scarlet Crusade                Low          Up         No
  Proudmoore                     Full         Up         No
  Silver Hand                    Low          Up         No
  Thorium Brotherhood            Low          Up         No
  Farstriders                    Low          Up         No
  Cairne                         High         Up         No
  Cenarius                       High         Up         No
  Frostmane                      High         Up         No
  Tortheldrin                    High         Up         No
  Ner'zhul                       High         Up         No
  Korgath                        High         Up         No
  Perenolde                      High         Up         No
  Wyrmrest Accord                Full         Up         No
  Aegwynn                        Recommended  Up         No
  Gurubashi                      Recommended  Up         No
  Bonechewer                     Recommended  Up         No
  Hakkar                         Recommended  Up         No
  Garrosh                        Recommended  Up         No
  Daggerspine                    Recommended  Up         No
  Emerald Dream                  High         Up         No
  Blackhand                      Low          Up         No
  Galakrond                      Low          Up         No
  Whisperwind                    Medium       Up         No
  Dentarg                        Medium       Up         No
  Stormrage                      Full         Up         No
  Durotan                        Low          Up         No
  Ysera                          Low          Up         No
  Bloodhoof                      Low          Up         No
  Duskwood                       Low          Up         No
  Rexxar                         Low          Up         No
  Misha                          Low          Up         No
  Staghelm                       Medium       Up         No
  Dawnbringer                    Medium       Up         No
  Madoran                        Medium       Up         No
  Azuremyst                      Medium       Up         No
  Illidan                        Full         Up         No
  Sargeras                       High         Up         No
  Azgalor                        Recommended  Up         No
  Thunderlord                    Recommended  Up         No
  Destromath                     Recommended  Up         No
  Blood Furnace                  Recommended  Up         No
  Mannoroth                      Recommended  Up         No
  Nazjatar                       Recommended  Up         No
  Azshara                        Recommended  Up         No
  Magtheridon                    Low          Up         No
  Anetheron                      Low          Up         No
  Ysondre                        Low          Up         No
  Altar of Storms                Low          Up         No
  Drakkari                       Recommended  Up         No
  Aerie Peak                     Medium       Up         No
  Ragnaros                       High         Up         No
  Shadow Council                 Medium       Up         No
  Sisters of Elune               Medium       Up         No
  Cenarion Circle                Medium       Up         No
  Blackwater Raiders             Medium       Up         No
  Alexstrasza                    Low          Up         No
  Terokkar                       Low          Up         No
  Kirin Tor                      Low          Up         No
  Steamwheedle Cartel            Low          Up         No
  Sentinels                      Low          Up         No
  Kul Tiras                      Low          Up         No
  Bladefist                      Low          Up         No
  Zul'jin                        Full         Up         No
  Maelstrom                      Low          Up         No
  Twisting Nether                Low          Up         No
  Lightninghoof                  Low          Up         No
  The Venture Co                 Low          Up         No
  Ravenholdt                     Low          Up         No
  Kalecgos                       Low          Up         No
  Shattered Halls                Low          Up         No
  Executus                       Low          Up         No
  Deathwing                      Low          Up         No
  Dark Iron                      Low          Up         No
  Shattered Hand                 Low          Up         No
  Coilfang                       Low          Up         No
  Demon Soul                     Low          Up         No
  Dalvengyr                      Low          Up         No
  Agamaggan                      High         Up         No
  Kargath                        High         Up         No
  Burning Legion                 High         Up         No
  Thunderhorn                    High         Up         No
  The Underbog                   High         Up         No
  Blade's Edge                   High         Up         No
  Archimonde                     High         Up         No
  Norgannon                      High         Up         No
  Jaedenar                       High         Up         No
  Baelgun                        Low          Up         No
  Doomhammer                     Low          Up         No
  Tichondrius                    Full         Up         No
  Suramar                        Medium       Up         No
  Windrunner                     Medium       Up         No
  Darrowmere                     Medium       Up         No
  Draka                          Medium       Up         No
  Alleria                        High         Up         No
  Exodar                         High         Up         No
  Medivh                         High         Up         No
  Khadgar                        High         Up         No
  Arthas                         Low          Up         No
  Detheroc                       Low          Up         No
  Dethecus                       Low          Up         No
  Lethon                         Low          Up         No
  Blackwing Lair                 Low          Up         No
  Shadowmoon                     Low          Up         No
  Haomarush                      Low          Up         No
  Gallywix                       Recommended  Up         No
  Greymane                       Low          Up         No
  Tanaris                        Low          Up         No
  Kil'jaeden                     Low          Up         No
  Darkspear                      Low          Up         No
  Bronzebeard                    Low          Up         No
  Shandris                       Low          Up         No
  Chromaggus                     Medium       Up         No
  Nathrezim                      Medium       Up         No
  Smolderthorn                   Medium       Up         No
  Anub'arak                      Medium       Up         No
  Arathor                        Medium       Up         No
  Garithos                       Medium       Up         No
  Drenden                        Medium       Up         No
  Crushridge                     Medium       Up         No
  Warsong                        Full         Up         No
  Gorgonnash                     Full         Up         No
  The Forgotten Coast            Full         Up         No
  Balnazzar                      Full         Up         No
  Alterac Mountains              Full         Up         No
  Undermine                      Full         Up         No
  Anvilmar                       Full         Up         No
  Quel'Thalas                    High         Up         No
  Goldrinn                       Low          Up         No
  Nemesis                        Low          Up         No
  Tol Barad                      Low          Up         No
  Azralon                        High         Up         No
  Dalaran                        Full         Up         No
  Mal'Ganis                      High         Up         No
  Turalyon                       Low          Up         No
  Moon Guard                     Full         Up         No
  Area 52                        Full         Up         No
  Thrall                         High         Up         No
  Hyjal                          Medium       Up         No
  Kel'Thuzad                     Medium       Up         No
  Lightbringer                   Medium       Up         No
  Caelestrasz                    High         Up         No
  Nagrand                        High         Up         No
  Saurfang                       High         Up         No
  Barthilas                      Medium       Up         No
  Frostmourne                    High         Up         No
  Dreadmaul                      High         Up         No
  Thaurissan                     High         Up         No
  Jubei'Thos                     High         Up         No
  Gundrak                        High         Up         No
  Khaz'goroth                    High         Up         No
  Aman'Thul                      High         Up         No
  Dath'Remar                     High         Up         No

  Total: 246 realms
```

## Population Colors

- **Low** - Green
- **Medium** - Yellow
- **High** - Red
- **Full** - Red
- **Recommended** - Cyan

## Status Colors

- **Up** - Green
- **Down** - Red

## How It Works

1. **OAuth2 Authentication**: Uses Battle.net's OAuth2 flow with SSO token exchange
2. **Chrome Automation**: Automates the login process using headless Chrome
3. **Battle.net API**: Queries the official Battle.net Game Data API
4. **Connected Realms**: Fetches all connected realm groups and individual realm status

## Troubleshooting

### Chrome not found

Make sure Google Chrome is installed and available in your system PATH.

### Authentication fails

- Check your username and password are correct
- Make sure you complete the 2FA prompt in the browser window
- Delete `~/.config/wowre/auth.json` to re-enter credentials

### SSO token expired

SSO tokens expire quickly. The tool automatically gets fresh tokens on each run, so this shouldn't be an issue.

### Verbose logging

Use the `--verbose` flag to see detailed debug output:

```bash
cargo run -- realms --verbose
```

## Security

- Credentials are stored in `~/.config/wowre/auth.json` in plain text
- Only use config file storage on trusted systems
- The tool uses official Battle.net OAuth2 endpoints
- No credentials are sent to third-party services

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

**GPLv3 Summary**: You are free to use, modify, and distribute this software, but any derivative works must also be distributed under the same license. This ensures the software remains free and open source.

## Contributing

[Add contribution guidelines here]
