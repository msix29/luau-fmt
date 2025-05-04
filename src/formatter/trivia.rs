//! All `impl` blocks for [`Trivia`].

use luau_parser::{
    prelude::{Comment, Trivia},
    types::Print,
};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

/// Whether or not the current line starts with ` ``` `. This also checks for a
/// maximum of 3 preceeding spaces (while accounting for indentation) since that
/// still counts as a code block in markdown.
fn is_triple_backticks(line: &str, indentation: Indentation, config: &Config) -> bool {
    let mut chars = line.chars();
    let mut space_count = 0;
    let mut tabs_count = 0;
    let indent_size = config.tab_size as Indentation;
    let max_indentation = config.tab_size as Indentation * indentation + 3;

    loop {
        match chars.next() {
            Some(' ') => space_count += 1,
            Some('\t') => tabs_count += 1,
            _ => break,
        }

        if space_count + tabs_count * indent_size > max_indentation {
            return false;
        }
    }

    chars.as_str().starts_with("```")
}

/// Used for comment formatting. Takes the passed text and expands it to multiple
/// lines, with every line starting with the passed prefix.
fn wrap_text_with_prefix(
    text: &str,
    prefix: &str,
    indentation: Indentation,
    config: &Config,
) -> String {
    let mut string = String::new();
    let available_width = config.comments_width.saturating_sub(prefix.len());
    let mut found_empty = false;
    let mut is_in_code_block = false;

    for line in text.split('\n') {
        let line = line.trim();
        if is_triple_backticks(line, indentation, config) {
            is_in_code_block = !is_in_code_block
        }

        if line.is_empty() {
            if is_in_code_block {
                string.push_str(prefix);
            } else if !found_empty {
                string.push_str(prefix);

                found_empty = true;
            } else {
                found_empty = false;
            }

            continue;
        }

        let words = line.split_whitespace();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > available_width {
                string.push_str(current_line.trim_end());
                string.push_str(config.newline_style.as_str());
                current_line = prefix.to_string();
            }

            current_line.push_str(word);
            current_line.push(' ');
        }

        string.push_str(current_line.trim_end());
    }

    string
}

impl Format for Comment {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let string = self.print();

        if string.len() > config.comments_width {
            match self {
                Comment::SingleLine(comment) => {
                    "-- ".to_string()
                        + &wrap_text_with_prefix(
                            &comment.trim_start()[2..],
                            &(config.indent_style.to_string(indentation, config) + "-- "),
                            indentation,
                            config,
                        )
                }
                Comment::MultiLine(comment) => {
                    let (comment_start, comment_end) = {
                        let stripped_comment = &comment[3..comment.len() - 1];
                        let comment_start = stripped_comment.find('[').unwrap();
                        let comment_end = stripped_comment.rfind(']').unwrap();

                        (comment_start + 4, comment_end + 3)
                    };
                    let (start, end) = (&comment[..comment_start], &comment[comment_end..]);

                    let prefix = config.indent_style.to_string(indentation + 1, config);

                    let mut string = start.to_string();
                    string.push_str(config.newline_style.as_str());
                    string.push_str(&prefix);
                    string.push_str(&wrap_text_with_prefix(
                        &comment[comment_start..comment_end],
                        &prefix,
                        indentation,
                        config,
                    ));
                    string.push_str(config.newline_style.as_str());
                    string.push_str(end);

                    string
                }
            }
        } else {
            string
        }
    }
}

/// Formatting types for [`[Trivia]`](Trivia).
pub enum TriviaFormattingType {
    /// Include only spaces.
    SpacesOnly,

    /// Include only comments, this'll include the space after every comment to avoid
    /// syntactical errors.
    CommentsOnly,
}

impl FormatWithArgs<TriviaFormattingType> for [Trivia] {
    fn format_with(
        &self,
        indentation: Indentation,
        config: &Config,
        trivia_formatting_type: TriviaFormattingType,
    ) -> String {
        match trivia_formatting_type {
            TriviaFormattingType::SpacesOnly => {
                self.iter().fold(String::new(), |str, trivia| match trivia {
                    Trivia::Spaces(smol_str) => str + smol_str,
                    Trivia::Comment(_) => str,
                })
            }
            TriviaFormattingType::CommentsOnly => {
                let mut found_comment = false;

                self.iter().fold(String::new(), |str, trivia| match trivia {
                    Trivia::Spaces(smol_str) => {
                        if found_comment {
                            found_comment = false;

                            str + smol_str
                        } else {
                            str
                        }
                    }
                    Trivia::Comment(comment) => {
                        found_comment = true;

                        str + &comment.format(indentation, config)
                    }
                })
            }
        }
    }
}
