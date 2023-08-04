use super::Crossword;
use crate::util::escape_sequences;
use std::fmt;
use unicode_width::UnicodeWidthChar;

impl fmt::Display for Crossword<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}×{}", self.width, self.height)?;
        write!(f, "{}", escape_sequences::WHITE_ON_DEFAULT)?;
        for row in self.rows() {
            let mut row = row.iter().peekable();
            while let Some(cell) = row.next() {
                if cell.highlighted() {
                    escape_sequences::write_grayscale(f, cell.highlighting)?;
                    write!(f, "{}", cell.char)?;

                    if cell.char.width() == Some(1) {
                        if let Some(next_cell) = row.peek() {
                            if next_cell.highlighted() {
                                write!(f, " ")?;
                                continue;
                            }
                        }

                        write!(f, " {}", escape_sequences::WHITE_ON_DEFAULT)?;
                    } else {
                        write!(f, "{}", escape_sequences::WHITE_ON_DEFAULT)?;
                    }
                } else if cell.char.width() > Some(1) || row.peek().is_none() {
                    write!(f, "{}", cell.char)?;
                } else {
                    write!(f, "{} ", cell.char)?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "{}", escape_sequences::RESET)?;

        let mut words = self.words.iter().peekable();
        let mut found_word_count = 0;
        while let Some(word) = words.next() {
            if word.marked {
                write!(f, "{}{}", escape_sequences::DARK_GRAY_FOREGROUND, word.str)?;
                found_word_count += 1;
            } else {
                write!(f, "{}", word.str)?;
            }
            if words.peek().is_some() {
                write!(f, ", ")?;
            }
            if word.marked {
                write!(f, "{}", escape_sequences::RESET)?;
            }
        }
        writeln!(f)?;

        if self.solved {
            let not_found_word_count = self.words.len() - found_word_count;
            if not_found_word_count != 0 {
                write!(f, "{}", escape_sequences::RED_FOREGROUND)?;
                if not_found_word_count == 1 {
                    writeln!(f, "1 word not found!")?;
                } else if not_found_word_count == self.words.len() {
                    writeln!(f, "No words found!")?;
                } else {
                    writeln!(f, "{} words not found!", not_found_word_count)?;
                }
            } else {
                writeln!(f, "{}All words found!", escape_sequences::GREEN_FOREGROUND)?;
            }
        } else {
            writeln!(f, "{}Unsolved.", escape_sequences::YELLOW_FOREGROUND)?;
        }
        write!(f, "{}", escape_sequences::RESET)?;
        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::{formatdoc, indoc};

    #[test]
    fn test_fmt() {
        let crossword = Crossword::try_from(indoc! {
            "aaaaaaaaaaaaaaaaaa
             aaaaaaiaaaaaaaaaaa
             aaaaaasaaaatheaaaa
             aformattedaaaaaaao
             aaaaaaaaaaaaaaaaau
             aaaaaaaaaaaaaaaaat
             aaaaaaaaaaaaaaaaap
             aaaaaaaaaaaaaaaaau
             aaaaaaaaaaaaaatest
             this is the formatted output test"
        })
        .unwrap();

        //assert_eq!(
        //    format!("{}", crossword),
        //    formatdoc!(
        //        "18×9\n\
        //         {}a a a a a a a a a a a a a a a a a a
        //         a a a a a a i a a a a a a a a a a a
        //         a a a a a a s a a a a t h e a a a a
        //         a f o r m a t t e d a a a a a a a o
        //         a a a a a a a a a a a a a a a a a u
        //         a a a a a a a a a a a a a a a a a t
        //         a a a a a a a a a a a a a a a a a p
        //         a a a a a a a a a a a a a a a a a u
        //         a a a a a a a a a a a a a a t e s t
        //         {}this, is, the, formatted, output, test
        //         {}Unsolved.
        //         {}",
        //        escape_sequences::WHITE_ON_DEFAULT,
        //        escape_sequences::RESET,
        //        escape_sequences::YELLOW_FOREGROUND,
        //        escape_sequences::RESET
        //    )
        //);
    }
}
