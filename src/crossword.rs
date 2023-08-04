mod fmt;
mod gen;
mod parse;
mod solver;

use crate::{args::ArgWords, util::Point, Error};
use std::{
    borrow::Cow,
    cmp, io,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    char: char,
    highlighting: u8,
}

impl Cell {
    fn new(char: char) -> Self {
        let highlighting = 255;

        Self { char, highlighting }
    }

    fn highlighted(self) -> bool {
        self.highlighting != 255
    }
}

/// A word as part of a crossword.
#[derive(Debug, PartialEq)]
pub struct Word<'a> {
    str: Cow<'a, str>,
    /// [`Self::str`]'s length in Unicode characters.
    ///
    /// This is far more accurate and language-independent than [`str::len`].
    len: usize,
    /// This property can have different meanings depending on the context.
    ///
    /// * In a solving context, this means the word has been found.
    /// * In a generation context, this means the word has been included in the grid.
    marked: bool,
}

impl<'a> Word<'a> {
    pub fn new(str: Cow<'a, str>) -> Self {
        let len = str.chars().count();
        let marked = false;

        Self { str, len, marked }
    }

    fn chars(&self) -> impl Iterator<Item = char> + '_ + DoubleEndedIterator {
        self.str.chars()
    }
}

#[derive(Debug, PartialEq)]
pub struct Crossword<'a> {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    words: Vec<Word<'a>>,
    solved: bool,
}

impl Crossword<'_> {
    fn highlight(&mut self, point: Point) {
        self[point].highlighting -= 2;
    }

    /// Gets an index to index [`cells`] and makes clamps the index if required to make sure the index is never out of bounds.
    fn get_index(&self, point: Point) -> usize {
        let point = Point {
            x: cmp::min(self.width - 1, point.x),
            y: cmp::min(self.height - 1, point.y),
        };
        point.x + self.width * point.y
    }

    fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks_exact(self.width)
    }
}

impl Index<Point> for Crossword<'_> {
    type Output = Cell;

    fn index(&self, point: Point) -> &Self::Output {
        &self.cells[self.get_index(point)]
    }
}

impl IndexMut<Point> for Crossword<'_> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let index = self.get_index(point);
        &mut self.cells[index]
    }
}

pub fn gen(watch: bool, arg_words: ArgWords, writer: &mut impl io::Write) -> Result<(), Error> {
    let mut words = Vec::<Word>::with_capacity(arg_words.len());

    for arg_word in arg_words {
        let word = Word::new(arg_word.into());
        words.push(word);
    }

    gen::gen(watch, &words, writer)
}
