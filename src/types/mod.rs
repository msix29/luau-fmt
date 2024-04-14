pub trait Format {
    fn format(&self) -> String;
}

pub trait Fold {
    fn fold(&self, formatted_string: &str) -> String {
        formatted_string.to_string()
    }
}

pub struct Config {}
