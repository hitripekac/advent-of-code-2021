use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fs,
    path::Path,
};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Eq for Point {}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn horisontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values = value
            .split(',')
            .map(|x| x.trim().parse().map_err(|_| anyhow!("Invalid coordinate")))
            .collect::<Result<Vec<isize>>>()?;
        if values.len() != 2 {
            Err(anyhow!("Invalid point format"))
        } else {
            Ok(Point {
                x: values[0],
                y: values[1],
            })
        }
    }
}

fn parse_line(line: &str) -> Result<Line> {
    let mut points = line.split("->").map(|x| x.try_into());

    Ok(Line {
        start: points.next().ok_or(anyhow!("Invalid input"))??,
        end: points.next().ok_or(anyhow!("Invalid input"))??,
    })
}

fn read_input<T>(path: T) -> Result<Vec<Line>>
where
    T: AsRef<Path>,
{
    let content = fs::read_to_string(path.as_ref())?;
    content.lines().map(parse_line).collect()
}

fn points_on_line(mut start: Point, end: Point) -> Vec<Point> {
    let mut points = vec![];
    let dx = (start.x - end.x).abs();
    let sx = if start.x < end.x { 1isize } else { -1isize };
    let dy = -(end.y - start.y).abs();
    let sy = if start.y < end.y { 1isize } else { -1isize };
    let mut err = dx + dy; /* error value e_xy */
    loop {
        /* loop */
        points.push(start.clone());
        if start.x == end.x && start.y == end.y {
            break;
        };
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            start.x += sx;
        }
        if e2 <= dx {
            err += dx;
            start.y += sy;
        }
    }
    points
}

fn main() -> Result<()> {
    let input = read_input("inputs/05a-hydrothermal-venture.input")?;
    let mut points = HashMap::new();
    for line in input.into_iter().filter(|x| x.horisontal_or_vertical()) {
        points_on_line(line.start.clone(), line.end.clone())
            .into_iter()
            .for_each(|point| *points.entry(point).or_insert(0usize) += 1);
    }
    println!("{:?}", points.values().filter(|&&x| x > 1usize).count());
    Ok(())
}
