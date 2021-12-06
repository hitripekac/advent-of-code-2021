use std::{fs, path::Path};

fn read_input<T>(path: T) -> Result<Vec<usize>, String>
where
    T: AsRef<Path>,
{
    let content = fs::read_to_string(path.as_ref()).map_err(|_| "Could not read file")?;
    content
        .lines()
        .map(|x| {
            x.parse::<usize>()
                .map_err(|_| "Could not parse to int".into())
        })
        .collect()
}

fn main() -> Result<(), String> {
    let input = read_input("inputs/01b-sonar-sweep.input")?;
    let sums_of_heights: Vec<usize> = input
        .as_slice()
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let increments = sums_of_heights
        .as_slice()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{}", increments);
    Ok(())
}
