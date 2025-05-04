# Configuration

The following table lists the available configuration options. These options control various formatting behaviors such as line width, indentation style, and sorting preferences.

| Field                     | Type                  | Description                                                                                                        | Default Value       |
|---------------------------|-----------------------|--------------------------------------------------------------------------------------------------------------------|---------------------|
| `column_width`            | `usize`               | The maximum width of characters per line. It limits parts of lines (like expressions) but not the full line width. | `100`               |
| `string_width`            | `usize`               | The maximum width of a string per line. Fully overrides `column_width`.                                            | `60`                |
| `comments_width`          | `usize`               | The maximum width of characters in a comment per line.                                                             | `80`                |
| `quote_style`             | `QuoteStyle`          | Quote style to use.                                                                                                | `"prefer_double"`   |
| `compact_table`           | `CompactTable`        | Whether to use compact table format (displayed in a single line).                                                  | `"only_literals"`   |
| `indent_style`            | `IndentStyle`         | Whether to use spaces or tabs for indentation.                                                                     | `"spaces"`          |
| `tab_size`                | `IndentSize` (usize)  | Number of spaces per tab if `indent_style` is set to `Spaces`.                                                     | `4`                 |
| `newline_style`           | `NewLineStyle`        | Line ending style.                                                                                                 | `"LF"`              |
| `trailing_commas`         | `TrailingCommas`      | Whether to include trailing commas in tables.                                                                      | `"only_multi_line"` |
| `keep_statements_spacing` | `bool`                | Whether to preserve spacing between statements (do not reduce to 2 lines).                                         | `false`             |
| `semicolon`               | `Semicolon`           | Determines when to use semicolons after statements.                                                                | `"never"`           |
| `add_final_newline`       | `bool`                | Whether to add a newline at the end of the file.                                                                   | `true`              |
| `sort_requires`           | `bool`                | Whether to sort `require(...)` statements within the same block.                                                   | `true`              |
| `sort_services`           | `bool`                | Whether to sort `game:GetService(...)` and `game.<IDENT>` within the same block.                                   | `true`              |
| `function_parenthesis`    | `FunctionParenthesis` | When to include parentheses around function arguments.                                                             | `"always"`          |

There are some unstable configurations that will never be loaded from a TOML file. The only way to use them is by using the library directly and passing a config yourself (which I don't recommend).

| Field            | Type               | Description                     | Default Value |
|------------------|------------------- |---------------------------------|---------------|
| `variable_casing`| `NamingConvention` | Naming convention for variables | `none`        |
| `method_casing`  | `NamingConvention` | Naming convention for methods   | `none`        |
| `type_casing`    | `NamingConvention` | Naming convention for types     | `none`        |

## Types

For those of you unaware of `usize`, feel free to read the [Rust documentation](https://doc.rust-lang.org/std/primitive.usize.html) - it'll explain it way better than I can. But in short, it's a positive number.

As you may have noticed in the above tables, some `Type`s aren't primitives. These are the possible values for each of them.

| Type                  | Possible values                                                                               |
|-----------------------|-----------------------------------------------------------------------------------------------|
| `QuoteStyle`          | `"single"`, `"prefer_single"`, `"double"`, `"prefer_double"`                                  |
| `CompactTable`        | `"always"`, `"only_literals"`, `"single_element"`, `"never"`                                  |
| `IndentStyle`         | `"spaces"`, `"tabs"`                                                                          |
| `NewLineStyle`        | `"LF"` (`\n`), `"CRLF"` (`\r\n`)                                               |
| `TrailingCommas`      | `"always"`, `"never"`, `"only_multi_line"`                                                    |
| `Semicolon`           | `"never"`, `"always"`, `"keep"`                                                               |
| `FunctionParenthesis` | `"always"`, `"keep"`, `"remove_for_strings"`, `"remove_for_tables"`, `"remove_when_possible"` |
| `NamingConvention`    | `"camelCase"`, `"PascalCase"`, `"snake_case"`, `"none"`                                       |

For `NamingConvention`, `"none"` means keep the name as-is.

For `QuoteStyle`, the `prefer_*` variants will pick the quote style only if the string will have less (or the same number of) escape sequences.

```lua
local _, _ = "This has no escapes", "Luau's formatter!"
-- using `single`
local _, _ = 'This has no escapes', 'Luau\'s formatter'
-- using `prefer_single`
local _, _ = 'This has no escapes', "Luau's formatter"
```

For other types, the `keep` variant means to keep what the user did without changing it.

## Example Configuration

Here is an example configuration file with the default values, you should only include the fields that you edit and omit the rest:

```toml
column_width = 100
string_width = 60
comments_width = 80
quote_style = "prefer_double"
compact_table = "only_literals"
indent_style = "spaces"
tab_size = 4
newline_style = "LF"
trailing_commas = "only_multi_line"
keep_statements_spacing = false
semicolon = "never"
add_final_newline = true

# These don't have an effect, but are valid configurations. Read above.
# variable_casing = "None"
# method_casing = "None"
# type_casing = "None"

sort_requires = true
sort_services = true
function_parenthesis = "always"
```
