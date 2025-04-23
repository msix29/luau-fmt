macro_rules! handle_parameters_and_returns {
    ($self: ident, $string: ident, $indentation: ident, $config: ident) => {{
        let parameters = $self.parameters.format_with($indentation, $config, ", ");

        if $string.len() + parameters.len() > $config.column_width {
            $string.push_str(&$self.parameters.format_with($indentation, $config, ","));
        } else {
            $string.push_str(&parameters);
        }
    }

    if $self.colon.is_some() {
        $string.push_str(&$self.colon.format($indentation, $config));
        $string.push(' ');

        let returns = $self.return_type.format($indentation, $config);

        if $string.len() + returns.len() > $config.column_width {
            $string.push_str(&$self.return_type.expand($indentation, $config));
        } else {
            $string.push_str(&returns);
        }
    }};
}

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
    }
}

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
        // $self.attributes.format($indentation, $config);
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
        handle_parameters_and_returns!($self, string, $indentation, $config);
        string.push_str(&$self.body.format($indentation + 1, $config));
        string.push_str(&$self.end_keyword.format($indentation, $config));

        string
    }};
}
