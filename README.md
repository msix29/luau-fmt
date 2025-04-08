# Luau Formatter

This crate provides a formatter for `CSTs` provided by the [`luau_parser`](https://github.com/msix29/luau-parser) crate.

## Usage

```rust
use luau_parser::prelude::Parser;

let code = "";
let uri = "";
let mut parser = Parser::new(code);
let cst = parser.parse(uri);

if let Ok(formatted_code) = luau_formatter::format(&cst) {
    println!("{}", formatted_code);
}
```
