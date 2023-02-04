# dotenv loader

This library is a simple environment variable setter that reads a .env file and sets env variables based on it's contents.

## Usage

You can pull dotenv_loader into your project with the following line in your Cargo.toml file under [dependencies]

```bash
dotenv_loader = { git = "https://github.com/vlad-onis/dotenv_loader", version = "0.1.0"}
```

In order to use the parser you will need to first create a a .env file with variables (See example in this repo's root).
If no path is provided, the library will default to the absolute path ".env".

You can then construct a parser and parse the file like so:

```rust
let mut dotenv_parser = Parser::new();
let _res = dotenv_parser.parse(Path::new(".env"));
```