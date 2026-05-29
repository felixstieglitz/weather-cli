# weather-cli

A small CLI tool in Rust that fetches the current weather for a city
using the [Open-Meteo](https://open-meteo.com/) API. No API key required.

## Features
- Look up weather by city and country
- Choose temperature unit (Celsius or Fahrenheit)
- Case-insensitive input (`berlin`, `Berlin`, `BERLIN` all work)

## Installation

Clone the repository and either install the binary globally or build it locally:

```bash
git clone <repo-url>
cd weather-cli

# Option A: install into ~/.cargo/bin so `weather-cli` is on your PATH
cargo install --path .

# Option B: just build a release binary at ./target/release/weather-cli
cargo build --release
```

If you chose Option B, the binary lives at `./target/release/weather-cli`. You
can either call it with the full path, or copy it somewhere on your `PATH`:

```bash
# Run directly
./target/release/weather-cli -c Berlin -n Germany -u C

# Or move it into a directory on your PATH (macOS / Linux)
cp target/release/weather-cli /usr/local/bin/
weather-cli -c Berlin -n Germany -u C
```

## Usage

```bash
weather-cli --city <CITY> --nation <COUNTRY> --unit <celsius|fahrenheit>
```

Short flags work too:

```bash
weather-cli -c Berlin -n Germany -u C
weather-cli -c "New York" -n "United States" -u F
```

If you haven't installed the binary, you can run it through cargo from inside
the project directory:

```bash
cargo run -- --city Berlin --nation Germany --unit celsius
```

## Example output

```
you entered: Berlin - Germany - Celsius
Located at: 52.52 - 13.405
It is 18.3 Celsius and Clear in Berlin
```

## Dependencies
- [`clap`](https://crates.io/crates/clap) — CLI parsing
- [`reqwest`](https://crates.io/crates/reqwest) — HTTP requests (blocking)
- [`serde`](https://crates.io/crates/serde) — JSON deserialisation

## License
MIT — see [LICENSE](LICENSE).
