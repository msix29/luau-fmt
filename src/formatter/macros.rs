//! Helper macros.

/// Formats the parameters and returns. Expands the parameters if they're too long.
macro_rules! handle_parameters_and_returns {
    (
        ( $parameters:expr, $(+ $space:literal +)? $symbol:expr, $return_type:expr ),
        $string: ident,
        $indentation: ident,
        $config: ident
    ) => {
        let start = $string.rfind('\n').unwrap_or_default();
        {
            let parameters = $parameters.format_with($indentation + 1, $config, ", ");

            if $string.len() + parameters.len() - start > $config.column_width {
                $string.push_str(&$parameters.expand_with(
                    $indentation + 1,
                    $config,
                    &(",".to_string()
                        + $config.newline_style.as_str()
                        + &$config.indent_style.to_string($indentation + 1, $config))
                ));
            } else {
                $string.push_str(&parameters);
            }
        }

        if let Some(symbol) = &$symbol {
            $( $string.push($space); )?
            $string.push_str(&symbol.format($indentation, $config));
            $string.push(' ');

            let returns = $return_type.format($indentation, $config);

            if $string.len() + returns.len() - start > $config.column_width {
                $string.push_str(&$return_type.expand($indentation, $config));
            } else {
                $string.push_str(&returns);
            }
        }
    };
}

/// A helper function for [`format_function!`] macro. It's mainly to handle type
/// functions since they may have a preceeding `export` keyword.
macro_rules! format_function_start_inner {
    ($self:ident . $export:ident, $indentation: ident, $config: ident) => {
        if $self.$export.is_some() {
            let mut string = $self.$export.format($indentation, $config);
            string.push(' ');
            string
        } else {
            String::new()
        }
    };
    ($self: ident, $indentation: ident, $config: ident) => {
        $self.attributes.format($indentation, $config)
    };
}

/// Formats all function types, local, global, closures, and type functions.
macro_rules! format_function {
    (
        $self: ident,
        $indentation: ident,
        $config: ident,
        $(let export = $export:ident;)?
        $(let keyword = $keyword:ident;)?
        $(let name = $function_name:ident;)?
    ) => {{
        let mut string = format_function_start_inner!($self $(. $export)?, $indentation, $config);
        $(
            string.push_str(&$self.$keyword.format($indentation, $config));
            string.push(' ');
        )?
        string.push_str(&$self.function_keyword.format($indentation, $config));
        $(
            string.push(' ');
            string.push_str(&$self.$function_name.format($indentation, $config));
        )?
        string.push_str(&$self.generics.format_with($indentation, $config, ", "));
        handle_parameters_and_returns!(
            ($self.parameters, $self.colon, $self.return_type),
            string,
            $indentation,
            $config
        );
        string.push_str(&$self.body.format($indentation + 1, $config));
        string.push_str(&$self.end_keyword.format($indentation, $config));

        string
    }};
}
