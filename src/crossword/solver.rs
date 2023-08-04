use super::Crossword;
use crate::util::{escape_sequences, Point};
use std::{io, thread, time};

macro_rules! cardinal_direction_fn {
    ($direction:ident, $rev:literal, $size:ident, $coord:ident) => {
        #[doc = concat!("Looks for words ", stringify!($direction), " from this point.")]
        fn $direction(&mut self, point: Point) {
            let result = self
                .words
                .iter()
                .enumerate()
                .filter(|(_, word)| !word.marked)
                .find(|(_, word)| {
                    if $rev {
                        (point.$coord..point.$coord + word.len)
                            .rev()
                            .map(|$coord| self[Point { $coord, ..point }].char)
                            .eq(word.chars())
                    } else {
                        (point.$coord..point.$coord + word.len)
                            .map(|$coord| self[Point { $coord, ..point }].char)
                            .eq(word.chars())
                    }
                });

            if let Some((i, word)) = result {
                for $coord in point.$coord..point.$coord + word.len {
                    self.highlight(Point { $coord, ..point });
                }
                self.words[i].marked = true;
            }
        }
    };
}

impl Crossword<'_> {
    cardinal_direction_fn!(north, true, height, y);
    cardinal_direction_fn!(east, false, width, x);
    cardinal_direction_fn!(south, false, height, y);
    cardinal_direction_fn!(west, true, width, x);

    // TODO: refactor the following with a macro called `ordinal_direction_fn`
    fn northeast(&mut self, point: Point) {
        let result = self
            .words
            .iter()
            .enumerate()
            .filter(|(_, word)| !word.marked)
            .find(|(_, word)| {
                (point.y..point.y + word.len)
                    .rev()
                    .enumerate()
                    .map(|(x, y)| self[Point { x: point.x + x, y }].char)
                    .eq(word.chars())
            });

        if let Some((i, word)) = result {
            for (x, y) in (point.y..point.y + word.len).rev().enumerate() {
                self.highlight(Point { x: point.x + x, y });
            }
            self.words[i].marked = true;
        }
    }
    fn southeast(&mut self, point: Point) {
        let result = self
            .words
            .iter()
            .enumerate()
            .filter(|(_, word)| !word.marked)
            .find(|(_, word)| {
                (point.x..point.x + word.len)
                    .enumerate()
                    .map(|(y, x)| self[Point { x, y: point.y + y }].char)
                    .eq(word.chars())
            });

        if let Some((i, word)) = result {
            for (y, x) in (point.x..point.x + word.len).enumerate() {
                self.highlight(Point { x, y: point.y + y });
            }
            self.words[i].marked = true;
        }
    }
    fn southwest(&mut self, point: Point) {
        let result = self
            .words
            .iter()
            .enumerate()
            .filter(|(_, word)| !word.marked)
            .find(|(_, word)| {
                (point.y..point.y + word.len)
                    .enumerate()
                    .map(|(x, y)| {
                        self[Point {
                            x: point.x.saturating_sub(x),
                            y,
                        }]
                        .char
                    })
                    .eq(word.chars())
            });

        if let Some((i, word)) = result {
            for (x, y) in (point.y..point.y + word.len).enumerate() {
                self.highlight(Point { x: point.x - x, y });
            }
            self.words[i].marked = true;
        }
    }
    fn northwest(&mut self, point: Point) {
        let result = self
            .words
            .iter()
            .enumerate()
            .filter(|(_, word)| !word.marked)
            .find(|(_, word)| {
                (point.x..point.x + word.len)
                    .rev()
                    .enumerate()
                    .map(|(y, x)| {
                        self[Point {
                            x,
                            y: point.y.saturating_sub(y),
                        }]
                        .char
                    })
                    .eq(word.chars())
            });

        if let Some((i, word)) = result {
            for (y, x) in (point.x..point.x + word.len).rev().enumerate() {
                self.highlight(Point { x, y: point.y - y });
            }
            self.words[i].marked = true;
        }
    }

    pub fn solve(&mut self, watch: bool) {
        let mut out = io::stdout();

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };

                // Try to find words clockwise from this cell

                self.north(point);
                self.northeast(point);
                self.east(point);
                self.southeast(point);
                self.south(point);
                self.southwest(point);
                self.west(point);
                self.northwest(point);

                if watch {
                    print!("{}", self);
                    escape_sequences::cursor_up(&mut out, self.height + 4).unwrap();

                    thread::sleep(time::Duration::from_secs_f32(0.1));
                }
            }
        }

        self.solved = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_cardinal_directions() {
        let mut crossword = Crossword::try_from(indoc! {
            "ahaaaaaaaaaaaaaaaaa
             aoaaaaaaaatreeaaaaa
             auaaaaaaaaaaaaaaaaa
             asaaaaadaaaaaaaaaaa
             aeaaaaaraaaaaaaaaaa
             aaaaaaaoaaaaaaaaaaa
             aaaaaaawaaaaaaaaaaa
             aaaaaaaaaaaegaugnal
             house tree word language"
        })
        .unwrap();

        crossword.solve(false);

        assert!(crossword.words.iter().all(|word| word.marked));
        assert!(crossword.solved);
    }

    #[test]
    fn test_ordinal_directions() {
        let mut crossword = Crossword::try_from(indoc! {
            "haaaaaaaaaaeaaaaaat
             aoaaaaaaaaaagaaaara
             aauaaaaaaaaaaaaaeaa
             aaasaaaaaaaaaaueaaa
             aaadeaaaaaaaaaagaaa
             aaraaaaaaaaaaaaanaa
             aoaaaaaaaaaaaaaaaaa
             waaaaaaaaaaaaaaaaal
             house tree word language"
        })
        .unwrap();

        crossword.solve(false);

        assert!(crossword.words.iter().all(|word| word.marked));
        assert!(crossword.solved);
    }

    #[test]
    fn test_cardinal_and_ordinal_directions() {
        let mut crossword = Crossword::try_from(indoc! {
            "ahaamaaaasaaaaaaaaa
             aoaaaraaaatreeaaaaa
             auaaaaoaaaaoaaaaaaa
             asaaaaadaaaararaaaa
             aeaaaaaraaaaaeaaaaa
             aaaaaakoaaaataaaaaa
             aaaaaeawaaaaaaaaaaa
             aaaayaaaaawegaugnal
             house tree word language store water key dorm"
        })
        .unwrap();

        crossword.solve(false);

        assert!(crossword.words.iter().all(|word| word.marked));
        assert!(crossword.solved);
    }
}
