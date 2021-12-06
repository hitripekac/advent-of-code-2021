use std::{fs, path::Path};

fn read_input<T>(path: T) -> Result<Vec<Direction>, String>
where
    T: AsRef<Path>,
{
    let content = fs::read_to_string(path.as_ref()).map_err(|_| "Could not read file")?;
    content.lines().map(parse_line).collect()
}

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse_number(line: &str) -> Result<isize, String> {
    line.parse().map_err(|_| "Invalid direction".into())
}

fn parse_line(line: &str) -> Result<Direction, String> {
    if line.starts_with("forward ") {
        Ok(Direction::Forward(parse_number(&line[8..])?))
    } else if line.starts_with("down ") {
        Ok(Direction::Down(parse_number(&line[5..])?))
    } else if line.starts_with("up ") {
        Ok(Direction::Up(parse_number(&line[3..])?))
    } else {
        Err("Could not parse input".into())
    }
}

fn main() -> Result<(), String> {
    let input = read_input("inputs/02a-dive.input")?;
    let result = input.iter().fold(
        (0, 0, 0isize),
        |(depth, position, aim), direction| match direction {
            &Direction::Forward(amount) => (depth + aim * amount, position + amount, aim),
            &Direction::Down(amount) => (depth, position, aim + amount),
            &Direction::Up(amount) => (depth, position, aim - amount),
        },
    );
    println!("{:?}, {}", result, result.0 * result.1);
    Ok(())
}
