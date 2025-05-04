//! [`NamingConvention`] enum.

/// Different naming conventions
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NamingConvention {
    /// `camelCase`.
    #[serde(rename = "camelCase")]
    Camel,

    /// `PascalCase`
    #[serde(rename = "PascalCase")]
    Pascal,

    /// `snake_case`
    #[serde(rename = "snake_case")]
    Snake,

    /// Keep it as it is.
    #[default]
    #[serde(rename = "none")]
    None,
}

#[inline]
fn is_splitter(char: char) -> bool {
    char == '_' || char.is_uppercase()
}

/// Splits the passed identifier into words
fn get_words(identifier: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();

    for (i, char) in identifier.chars().enumerate() {
        if i != 0 && is_splitter(char) {
            words.push(current_word);
            current_word = String::new();
        }

        current_word.push(char);
    }

    words.push(current_word);
    words
}

/// Capitalizes the first letter if the passed word if it's at least one
/// character long
fn capitalize_first_letter(word: &mut str) -> &str {
    if word.len() > 1 {
        word[..1].make_ascii_uppercase();
    }

    word
}

impl NamingConvention {
    /// Applies self onto the passed identifier and returns the new one.
    pub fn apply(&self, identifier: &str) -> String {
        if *self == Self::None {
            return identifier.to_string()
        }

        let mut words = get_words(identifier);

        match self {
            NamingConvention::Camel => words
                        .iter_mut()
                        .enumerate()
                        .map(|(i, word)| {
                            if i == 0 {
                                word.to_lowercase()
                            } else {
                                capitalize_first_letter(word).to_string()
                            }
                        })
                        .collect::<String>(),
            NamingConvention::Pascal => words
                        .iter_mut()
                        .map(|word| capitalize_first_letter(word))
                        .collect::<String>(),
            NamingConvention::Snake => words
                        .iter()
                        .map(|word| word.to_lowercase())
                        .collect::<Vec<String>>()
                        .join("_"),
            NamingConvention::None => unreachable!(),
                        }
    }
}
