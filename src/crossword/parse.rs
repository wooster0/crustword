use super::{Cell, Crossword, Word};
use crate::Error;

impl<'a> TryFrom<&'a str> for Crossword<'a> {
    type Error = Error;

    fn try_from(crossword_str: &'a str) -> Result<Self, Self::Error> {
        let mut lines = crossword_str.lines().peekable();

        // Get the length of the first line to use as the initial capacity of `cells`
        // and to check for line width inconsistencies
        let width = if let Some(first_line) = lines.peek() {
            first_line
                .chars()
                .filter(|char| !char.is_whitespace())
                .count()
        } else {
            return Err("empty grid");
        };

        // Together with `with_capacity` and `reserve_exact` (see below) we can make sure that
        // we end up with the perfect capacity
        let mut cells = Vec::<Cell>::with_capacity(width);
        let mut height = None;
        let mut words = Vec::<Word>::new();

        while let Some(line) = lines.next() {
            // Is this the last line?
            if lines.peek().is_none() {
                // Parse the word list
                for word in line.split_whitespace() {
                    words.push(Word::new(word.into()));
                }
                if words.is_empty() {
                    return Err("empty grid");
                }
            } else {
                // Ignore lines only containing whitespace
                if line.trim().is_empty() {
                    continue;
                }

                // In the common case, we know exactly how much more we need to allocate
                cells.reserve_exact(width);

                for char in line.chars().filter(|char| !char.is_whitespace()) {
                    cells.push(Cell::new(char));
                }

                // If the length is not the same as what we allocated,
                // we know that this line has an inconsistent width
                if cells.len() != cells.capacity() {
                    return Err("inconsistent width");
                }

                *height.get_or_insert(0) += 1;
            }
        }

        if let Some(height) = height {
            let solved = false;

            Ok(Crossword {
                cells,
                width,
                height,
                words,
                solved,
            })
        } else {
            Err("empty grid")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Point;
    use indoc::indoc;

    #[test]
    fn test_try_from_empty_grid() {
        assert!(matches!(Crossword::try_from(""), Err("empty grid")));
        assert!(matches!(Crossword::try_from("a"), Err("empty grid")));
        assert!(matches!(Crossword::try_from("\n"), Err("empty grid")));
        assert!(matches!(Crossword::try_from("\na"), Err("empty grid")));
    }

    #[test]
    fn test_try_from_inconsistent_width() {
        assert!(matches!(
            Crossword::try_from(indoc! {
                "aaa
                 aaaaaa
                 aaaaaa"
            }),
            Err("inconsistent width")
        ));
        assert!(matches!(
            Crossword::try_from(indoc! {
                "aaaaaa
                 aaa
                 aaaaaa"
            }),
            Err("inconsistent width")
        ));
    }

    #[test]
    fn test_try_from_different_styles() {
        let crossword1 = Crossword::try_from(indoc! {
            "aaaaaa
             aaaaaa
             aaaaaa
             a"
        })
        .unwrap();
        let crossword2 = Crossword::try_from(indoc! {
            "a a a a a a
             a a a a a a
             a a a a a a
             a"
        })
        .unwrap();
        let crossword3 = Crossword::try_from(indoc! {
            "a  a  a       a  a   a

             a    a  a  a  a   a


             a  a    a    a  a        a

             a"
        })
        .unwrap();

        let crosswords = [crossword1, crossword2, crossword3];

        assert!(crosswords
            .iter()
            .all(|crossword| *crossword == crosswords[0]));
    }

    #[test]
    fn test_try_from() {
        //let crossword = Crossword::try_from(indoc! {
        //    "0K000000000000000000
        //     0o000000000000000000
        //     Crossword00000000000
        //     0s0000クロスワード00000000
        //     0o000000000000000000
        //     0r00填字遊戲000000000000
        //     0d000000000000000000
        //     Korsord Crossword クロスワード Kreuzworträtsel 填字遊戲"
        //})
        //.unwrap();

        //assert!(!crossword.solved);
        //assert_eq!(crossword.width, 20);
        //assert_eq!(crossword.height, 7);
        //assert_eq!(
        //    crossword.words,
        //    vec![
        //        Word::new("Crossword".into()),
        //        Word::new("Korsord".into()),
        //        Word::new("クロスワード".into()),
        //        Word::new("填字遊戲".into())
        //    ]
        //);
        //assert_eq!(crossword[Point { x: 0, y: 0 }], Cell::new('0'));
        //assert!(crossword.cells.iter().all(|cell| !cell.highlighted()));
    }
    }
