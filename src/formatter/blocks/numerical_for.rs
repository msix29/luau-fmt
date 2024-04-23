//! Implements formatting traits for numerical for loops.

use luau_parser::types::{Expression, NumericalFor};

use crate::types::Format;

/// Formats the `step` argument.
fn format_step(step: &Expression, indentation: &mut i32) -> String {
    format!(", {}", step.format(indentation))
}

impl Format for NumericalFor {
    fn format(&self, indentation: &mut i32) -> String {
        let step = if let Some(step) = &self.step {
            if let Expression::Number(number) = step {
                let n = number.word.parse::<isize>().unwrap();
                if n == 1 {
                    // If the step is 1, just exclude it.
                    String::new()
                } else {
                    format_step(step, indentation)
                }
            } else {
                format_step(step, indentation)
            }
        } else {
            String::new()
        };

        format!(
            "for {} = {}, {}{} {}",
            self.variable.format(indentation),
            self.start.format(indentation),
            self.end.format(indentation),
            step,
            self.do_block.format(indentation),
        )
    }
}
