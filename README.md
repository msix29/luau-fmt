# Luau Formatter

This crate provides a formatter for `CSTs` provided by the [`luau_parser`](https://github.com/msix29/luau-parser) crate.

## Usage (binary)

```pwsh
luaufmt.exe <PATH> [CONFIG_PATH]
```

Path can be a folder or a file, it'll format everything in the folder recursively or just format the file.
If config path isn't provided, it'll look for a `luaufmt.toml` in the current directory and use it. If not found, or failed to load, it'll use the default configuration.

## Usage (library)

```rust
use luau_parser::prelude::Parser;

let code = "";
let uri = "";
let mut parser = Parser::new(code);
let cst = parser.parse(uri);

if let Ok(formatted_code) = luau_fmt::format(&cst) {
    println!("{}", formatted_code);
} else {
    eprintln!("The CST had errors.");
}
```

## Configuration

Check [configuration.md](configuration.md)

## Skipping

Sometimes, you want to preserve your own styling, ex.

```lua
local nDimensionalArray = {
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
}
```

This will be collapsed into one line. U can add `--@luau-fmt skip` before it to ignore it.
If you want to ignore full code blocks instead, use `--@luau-fmt skip-start` before and `--@luau-fmt skip-end` after the block. If a `skip-start` is found and no `skip-end` is found, it'll basically disable formatting for the rest of the file, and the formatter will not show a warning in such cases, since it may be what the user actually wants.
