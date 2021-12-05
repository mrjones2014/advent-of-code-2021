use std::{collections::HashMap, error::Error, num::ParseIntError, str::FromStr};
use utils::input_parser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i64, i64);
#[derive(Debug, Clone, PartialEq, Eq)]
struct LineSegment(Point, Point);

#[derive(Debug)]
enum ParsePointError {
    NoDelimeter,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParsePointError {
    fn from(e: ParseIntError) -> Self {
        ParsePointError::ParseIntError(e)
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    /// Parses a string in the form "[0-9]+,[0-9]+"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(Self::Err::NoDelimeter)?;
        Ok(Point(x.parse()?, y.parse()?))
    }
}

impl Point {
    pub fn range(&self) -> Box<dyn Iterator<Item = i64>> {
        if self.0 <= self.1 {
            Box::new(self.0..=self.1)
        } else {
            Box::new((self.1..=self.0).rev())
        }
    }
}

#[derive(Debug)]
enum ParseLineSegmentError {
    NoDelimeter,
    ParsePointError(ParsePointError),
}

impl From<ParsePointError> for ParseLineSegmentError {
    fn from(e: ParsePointError) -> Self {
        ParseLineSegmentError::ParsePointError(e)
    }
}

impl FromStr for LineSegment {
    type Err = ParseLineSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (point1_str, point2_str) = s.split_once(" -> ").ok_or(Self::Err::NoDelimeter)?;
        Ok(LineSegment(point1_str.parse()?, point2_str.parse()?))
    }
}

fn compute(
    line_segments: &[LineSegment],
    include_diagonals: bool,
) -> Result<usize, Box<dyn Error>> {
    let grid = line_segments.iter().fold(
        HashMap::<(i64, i64), usize>::new(),
        |mut acc, line_segment| {
            if line_segment.0 .0 == line_segment.1 .0 {
                for y in Point(line_segment.0 .1, line_segment.1 .1).range() {
                    *acc.entry((line_segment.0 .0, y)).or_default() += 1;
                }
            } else if line_segment.0 .1 == line_segment.1 .1 {
                for x in Point(line_segment.0 .0, line_segment.1 .0).range() {
                    *acc.entry((x, line_segment.0 .1)).or_default() += 1;
                }
            } else if include_diagonals {
                for (x, y) in Point(line_segment.0 .0, line_segment.1 .0)
                    .range()
                    .zip(Point(line_segment.0 .1, line_segment.1 .1).range())
                {
                    *acc.entry((x, y)).or_default() += 1;
                }
            }

            acc
        },
    );

    Ok(grid.into_iter().filter(|(_, count)| *count > 1).count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = input_parser::parse("puzzle5");
    let line_segments: Vec<LineSegment> = inputs
        .iter()
        .map(|input| input.parse().expect("Unable to parse line"))
        .collect();

    println!("Part 1: {}", compute(&line_segments, false)?);
    println!("Part 2: {}", compute(&line_segments, true)?);
    Ok(())
}
