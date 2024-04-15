pub trait Format {
    fn format(&self, indentation: &mut i32) -> String;
}
pub trait FormatWithArgs<P> {
    fn format_with_args(&self, indentation: &mut i32, parameter: P) -> String;
}

pub trait Fold {
    fn fold(&self, formatted_string: &str) -> String {
        formatted_string.to_string()
    }
}

pub struct Config {
    pub column_width: u32,
}
