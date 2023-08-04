use super::Word;
use crate::{
    util::{escape_sequences, Point},
    Error,
};
use rand::{
    distributions::{Bernoulli, Uniform},
    prelude::Distribution,
    rngs::SmallRng,
    Rng, SeedableRng,
};
use std::{
    io,
    ops::{ControlFlow, Index, IndexMut},
    thread, time,
};
use unicode_width::UnicodeWidthChar;

#[derive(Debug)]
struct Grid {
    cells: Vec<char>,
    width: usize,
    height: usize,
    width_range: Uniform<usize>,
    height_range: Uniform<usize>,
}

impl Grid {
    fn new(words: &[Word], rng: &mut impl Rng) -> Result<Self, Error> {
        if let Some(max_word_len) = words.iter().map(|word| word.len).max() {
            let (width, height) = (
                rng.gen_range(max_word_len..max_word_len * 2),
                rng.gen_range(max_word_len..max_word_len * 2),
            );
            let cells = vec![char::default(); width * height];

            let width_range = Uniform::from(0..width);
            let height_range = Uniform::from(0..height);

            Ok(Self {
                cells,
                width,
                height,
                width_range,
                height_range,
            })
        } else {
            Err("no words")
        }
    }

    fn get_rand_point(&self, rng: &mut impl Rng) -> Point {
        Point {
            x: self.width_range.sample(rng),
            y: self.height_range.sample(rng),
        }
    }

    fn get_index(&self, point: Point) -> usize {
        point.x + self.width * point.y
    }

    fn get(&self, point: Point) -> Option<&char> {
        if point.x >= self.width || point.y >= self.height {
            None
        } else {
            self.cells.get(self.get_index(point))
        }
    }
}

impl Index<Point> for Grid {
    type Output = char;

    fn index(&self, point: Point) -> &Self::Output {
        &self.cells[self.get_index(point)]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let index = self.get_index(point);
        &mut self.cells[index]
    }
}

pub fn gen(watch: bool, words: &[Word], writer: &mut impl io::Write) -> Result<(), Error> {
    let mut rng = SmallRng::from_entropy();
    gen_internal(watch, words, writer, &mut rng)
}

fn gen_internal(
    mut watch: bool,
    words: &[Word],
    writer: &mut impl io::Write,
    rng: &mut impl Rng,
) -> Result<(), Error> {
    fn index(
        grid: &mut Grid,
        points: impl Iterator<Item = Point> + Clone,
        word: impl Iterator<Item = char>,
    ) -> ControlFlow<()> {
        if points.clone().all(|point| grid.get(point) == Some(&'\0')) {
            for (point, char) in points.zip(word) {
                grid[point] = char;
            }
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }

    let mut grid = Grid::new(words, rng)?;

    let direction_range = Uniform::from(0..8);

    for word in words {
        let mut point = grid.get_rand_point(rng);
        let mut direction = direction_range.sample(rng);

        loop {
            match direction {
                0 => {
                    // North
                    if let Some(result) = point.y.checked_sub(word.len) {
                        let points = (result..point.y).map(|y| Point { y, ..point });
                        if let ControlFlow::Break(()) = index(&mut grid, points, word.chars().rev())
                        {
                            break;
                        }
                    }
                }
                1 => {
                    // Northeast
                    if let Some(result) = point.y.checked_sub(word.len) {
                        let points = (result..point.y)
                            .rev()
                            .enumerate()
                            .map(|(x, y)| Point { y, x: point.x + x });
                        if let ControlFlow::Break(()) = index(&mut grid, points, word.chars()) {
                            break;
                        }
                    }
                }
                2 => {
                    // East
                    let points = (point.x..point.x + word.len).map(|x| Point { x, ..point });
                    if let ControlFlow::Break(()) = index(&mut grid, points, word.chars()) {
                        break;
                    }
                }
                3 => {
                    // Southeast
                    let points = (point.y..point.y + word.len)
                        .enumerate()
                        .map(|(x, y)| Point { y, x: point.x + x });
                    if let ControlFlow::Break(()) = index(&mut grid, points, word.chars()) {
                        break;
                    }
                }
                4 => {
                    // South
                    let points = (point.y..point.y + word.len).map(|y| Point { y, ..point });
                    if let ControlFlow::Break(()) = index(&mut grid, points, word.chars()) {
                        break;
                    }
                }
                5 => {
                    // Southwest
                    let points = (point.y..point.y + word.len)
                        .rev()
                        .enumerate()
                        .map(|(x, y)| Point { x, y });
                    if let ControlFlow::Break(()) = index(&mut grid, points, word.chars().rev()) {
                        break;
                    }
                }
                6 => {
                    // West
                    if let Some(result) = point.x.checked_sub(word.len) {
                        let points = (result..point.x).map(|x| Point { x, ..point });
                        if let ControlFlow::Break(()) = index(&mut grid, points, word.chars().rev())
                        {
                            break;
                        }
                    }
                }
                7 => {
                    // Northwest
                    if point.x.checked_sub(word.len).is_some() {
                        if let Some(result) = point.y.checked_sub(word.len) {
                            let points = (result..point.y)
                                .rev()
                                .enumerate()
                                .map(|(x, y)| Point { y, x: point.x - x });
                            if let ControlFlow::Break(()) = index(&mut grid, points, word.chars()) {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    point = grid.get_rand_point(rng);
                    direction = direction_range.sample(rng);
                    continue;
                }
            }
            direction += 1;
        }

        if watch {
            if write_grid(watch, &grid, writer, rng).is_err()
                || write_words(words, writer).is_err()
                || writeln!(writer).is_err()
                || escape_sequences::cursor_up(writer, grid.height + 2).is_err()
            {
                return Err("writing failed");
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    watch = false;

    if write_grid(watch, &grid, writer, rng).is_err() || write_words(words, writer).is_err() {
        Err("writing failed")
    } else {
        Ok(())
    }
}

fn write_grid(
    watch: bool,
    grid: &Grid,
    writer: &mut impl io::Write,
    rng: &mut impl Rng,
) -> io::Result<()> {
    let original_fill_characters = grid.cells.iter().filter(|&char| char != &'\0');
    let mut fill_characters = original_fill_characters.clone();
    let distribution = Bernoulli::new(0.5).unwrap();

    for row in grid.cells.chunks_exact(grid.width) {
        let mut row = row.iter().peekable();
        while let Some(char) = row.next() {
            let written_char = if char == &'\0' {
                if watch {
                    &' '
                } else {
                    if distribution.sample(rng) {
                        fill_characters.next();
                    }
                    let fill_character = fill_characters.next().unwrap_or_else(|| {
                        // Refill the fill characters
                        fill_characters = original_fill_characters.clone();
                        fill_characters.next().unwrap()
                    });
                    write!(writer, "{}", fill_character)?;
                    fill_character
                }
            } else {
                write!(writer, "{}", char)?;
                char
            };

            if written_char.width() <= Some(1) && row.peek().is_some() {
                write!(writer, " ")?;
            }
        }
        writeln!(writer)?;
    }
    writeln!(writer)?;

    Ok(())
}

fn write_words(words: &[Word], writer: &mut impl io::Write) -> io::Result<()> {
    let mut words = words.iter().peekable();

    // Determine whether to use half width or full width characters for separation
    // depending on whether there are overall more full-width characters than half-width characters
    let use_full_width = words
        .clone()
        .filter(|word| word.chars().filter(|char| char.width() == Some(2)).count() > word.len / 2)
        .count()
        > words.len() / 2;

    while let Some(word) = words.next() {
        write!(writer, "{}", word.str)?;
        if words.peek().is_some() {
            if use_full_width {
                write!(writer, "　")?;
            } else {
                write!(writer, " ")?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_gen() {
        let mut rng = SmallRng::seed_from_u64(0);
        let mut writer = Vec::new();

        gen_internal(
            false,
            &[Word::new("hello".into()), Word::new("world".into())],
            &mut writer,
            &mut rng,
        )
        .unwrap();
        assert_eq!(
            std::str::from_utf8(&writer).unwrap(),
            indoc!(
                "o l e h o l o
                 l l e w r l o
                 l l h w r l o
                 l e w r d o l
                 h o l l e h w
                 o l w o r l d
                 o l e w r d o

                 hello world"
            )
        );

        writer.clear();

        gen_internal(
            false,
            &[
                Word::new("blåhaj".into()),
                Word::new("hello".into()),
                Word::new("クロスワード".into()),
            ],
            &mut writer,
            &mut rng,
        )
        .unwrap();
        assert_eq!(
            std::str::from_utf8(&writer).unwrap(),
            indoc!(
                "クh b h ロe
                 ロe l l スl
                 スl å å l h
                 ワl h o a j
                 ーo a クh b
                 ドロj e l l

                 blåhaj hello クロスワード"
            )
        );
    }
}
