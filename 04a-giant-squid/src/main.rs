use anyhow::{anyhow, Result};
use std::{convert::TryInto, fs, path::Path};
type Number = isize;

#[derive(Debug)]
struct Bingo {
    numbers: Vec<Number>,
    grids: Vec<Grid>,
}

#[derive(Debug)]
struct Field {
    number: Number,
    drawn: bool,
}

impl From<Number> for Field {
    fn from(n: Number) -> Self {
        Field {
            number: n,
            drawn: false,
        }
    }
}

type Row = [Field; 5];
type Grid = [Row; 5];

fn parse_field(value: &str) -> Result<Field> {
    let int_value: Number = value.parse()?;
    Ok(int_value.into())
}

fn read_single_row(line: &str) -> Result<[Field; 5]> {
    Ok(line
        .split_ascii_whitespace()
        .map(parse_field)
        .collect::<Result<Vec<Field>, _>>()?
        .try_into()
        .map_err(|from| anyhow!("Cannot transform {:?} to Row", from))?)
}

fn parse_grid<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Result<Grid> {
    let grid_simple = lines
        .take(5)
        .map(read_single_row)
        .collect::<Result<Vec<_>, _>>()?;
    let p: Grid = grid_simple
        .try_into()
        .map_err(|from: Vec<Row>| anyhow!("Invalid number of rows; {}.", from.len()))?;
    Ok(p)
}

fn read_input<T>(path: T) -> Result<Bingo>
where
    T: AsRef<Path>,
{
    let content = fs::read_to_string(path.as_ref())?;
    let mut lines = content.lines();
    let numbers = lines
        .next()
        .ok_or(anyhow!("Invalid input"))?
        .split(',')
        .map(|x| x.parse::<Number>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut grids = vec![];
    while lines.next().is_some() {
        grids.push(parse_grid(&mut lines)?);
    }
    Ok(Bingo { numbers, grids })
}

fn find_number_in_grid(grid: &Grid, number: Number) -> Option<(usize, usize)> {
    for (row, values) in grid.iter().enumerate() {
        for (column, field) in values.iter().enumerate() {
            if field.number == number {
                return Some((row, column));
            }
        }
    }
    None
}

fn check_if_win(grid: &Grid, row: usize, col: usize) -> bool {
    let row_win = grid[row].iter().all(|x| x.drawn);
    let col_win: bool = (0..5).map(|x| &grid[x][col]).all(|x| x.drawn);
    return row_win || col_win;
}

fn sum_grid_unmarked(grid: &Grid) -> Number {
    grid.iter()
        .flat_map(|x| x.iter())
        .filter_map(|x| if !x.drawn { Some(x.number) } else { None })
        .sum()
}

fn main() -> Result<()> {
    let mut input = read_input("inputs/04a-giant-squid.input")?;
    let mut last_grid = 0;
    for number in input.numbers {
        for grid in &mut input.grids {
            if let Some((row, col)) = find_number_in_grid(&grid, number) {
                grid[row][col].drawn = true;
                if check_if_win(&grid, row, col) {
                    last_grid = sum_grid_unmarked(&grid) * number;
                };
            }
        }
    }
    println!("{}", last_grid);
    Ok(())
}
