# weather-cli

A small CLI tool in Rust that fetches the current weather for a city
using the [Open-Meteo](https://open-meteo.com/) API. No API key required.

## Features
- Look up weather by city and country
- Choose temperature unit (Celsius or Fahrenheit)
- Case-insensitive input (`berlin`, `Berlin`, `BERLIN` all work)

## Installation

First, clone the repository:

```bash
git clone <repo-url>
```

Then install the binary with cargo. You have two options:

**Option A:** change into the project directory and install from there:

```bash
cd weather-cli
cargo install --path .
```

The `--path .` tells cargo to install the project in the current directory,
so you need to run this command from inside the `weather-cli` folder (where
`Cargo.toml` lives).

**Option B:** pass the full path to the project and run the command from
anywhere:

```bash
cargo install --path /path/to/weather-cli
```

Either way, this builds the binary and places it in `~/.cargo/bin/`, which
is on your `PATH`, so you can call `weather-cli` from any directory.

## Uninstall

To remove the binary again:

```bash
cargo uninstall weather-cli
```

This can be run from anywhere — cargo knows where the binary lives.

## Usage

```bash
weather-cli --city <CITY> --nation <COUNTRY> --unit <celsius|fahrenheit>
```

Short flags work too:

```bash
weather-cli -c Berlin -n Germany -u C
weather-cli -c "New York" -n "United States" -u F
```

## Example output

```
you entered: Berlin - Germany - Celsius
Located at: 52.52 - 13.405
It is 18.3 Celsius and Clear in Berlin
```

## License
MIT — see [LICENSE](LICENSE).
