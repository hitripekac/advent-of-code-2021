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
    let input = read_input("inputs/01a-sonar-sweep.input")?;
    let increments = input
        .as_slice()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count();
    println!("{:?}", increments);
    Ok(())
}
