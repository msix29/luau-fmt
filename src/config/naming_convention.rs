//! [`NamingConvention`] enum.

/// Different naming conventions
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NamingConvention {
    // `camelCase`.
    Camel,

    /// `PascalCase`
    Pascal,

    /// `snake_case`
    Snake,
}

#[inline]
fn is_splitter(char: char) -> bool {
    char == '_' || char.is_uppercase()
}

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

fn capitalize_first_letter(word: &mut str) -> &str {
    if word.len() > 1 {
        word[..1].make_ascii_uppercase();
    }

    word
}

impl NamingConvention {
    pub fn apply(&self, identifier: &str) -> String {
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
        }
    }
}
