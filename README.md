# Salut

A multilingual greeting CLI tool with profanity levels written in Rust. 

This tool is designed to greet you with motivational messages when you open your terminal.

## Features

- Support for multiple languages
- Configurable profanity levels (0-2)
- Clean command-line interface with helpful error messages
- Extensible greeting system via JSON configuration

## Usage

### Shell Integration

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# Add this line to get greeted every time you open a terminal
salut --name <YourName> --language en --profanity 1
```

Or for a more dynamic approach:

```bash
# Random profanity level each time
salut --name <YourName> --language en --profanity $((RANDOM % 3))
```

### Usage

```bash
Usage: salut [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>            Name to greet
  -l, --language <LANGUAGE>    Language (en, fr, es, de) [default: en]
  -p, --profanity <PROFANITY>  Profanity level (0-2) [default: 0]
  -h, --help                   Print help
  -V, --version                Print version
```

## Command Line Options

- `-n, --name <NAME>`: Name to greet (required)
- `-l, --language <LANGUAGE>`: Language code (default: en)
  - Available: `en` (English), `fr` (French), `es` (Spanish), `de` (German)
- `-p, --profanity <LEVEL>`: Profanity level (default: 0)
  - `0`: Clean greeting
  - `1`: Mild work motivation with profanity
  - `2`: Strong work motivation with profanity

## Examples

```bash
# English greetings
salut -n Alice -l en -p 1  # "Hello Alice! Now get to fucking work!"

# French greetings
salut -n Bob -l fr -p 2    # "Salut Bob! Bouge-toi le cul et fais péter le travail!"

# Spanish greetings
salut -n Carol -l es -p 0  # "Hola Carol!"
```

## Project Structure

```
salut/
├── Cargo.toml          # Project metadata and dependencies
├── README.md           # This file
├── greetings.json      # Greeting templates for all languages
└── src/
    └── main.rs         # Main application code
```

## Adding New Languages



## Contributing

We welcome contributions to make `salut` even more motivational! Here's how you can help:

### Build

```bash
cargo build --release
```
The binary will be available at `target/release/salut`.

### Adding New Languages

1. Fork the repository
2. Add your language to `greetings.json` with (non-)appropriate work-motivational messages. The new language should have three profanity levels :

```json
{
  "greetings": {
    "en": {
      "0": "Hello {name}!",
      "1": "Hello {name}! Now get to fucking work!",
      "2": "Hello {name}! Time to kick ass and get shit done!"
    },
  }
}
```
3. Update the language list in `src/main.rs` help text if needed
4. Add examples to the README
5. Test your changes with `cargo test`
6. Submit a pull request

### Improving Existing Languages

- Make greetings more motivational and work-focused
- Ensure cultural appropriateness while maintaining the intended spirit
- Keep the progressive profanity levels (0: clean, 1: mild, 2: strong)

### Code Improvements

- Add new features (environment variable support, config files, etc.)
- Improve error handling
- Add more comprehensive tests
- Optimize performance

### Development Setup

```bash
git clone https://github.com/yourusername/salut.git
cd salut
cargo build
cargo test
```

## Dependencies

- `clap`: Command-line argument parsing
- `serde`: JSON serialization/deserialization
- `serde_json`: JSON handling

## License

MIT