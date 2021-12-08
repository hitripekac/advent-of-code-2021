use std::{fs, path::Path};

fn read_input<T>(path: T) -> Result<Vec<Vec<u8>>, String>
where
    T: AsRef<Path>,
{
    let content = fs::read_to_string(path.as_ref()).map_err(|_| "Could not read file")?;
    Ok(content.lines().map(parse_number).collect())
}

fn parse_number(line: &str) -> Vec<u8> {
    line.bytes().map(|x| x - 48).collect()
}

fn main() -> Result<(), &'static str> {
    let input =
        read_input("inputs/03a-binary-diagnostic.input").map_err(|_| "Could not read file")?;
    let word_len = input[0].len();
    let sums = input.iter().fold(vec![0isize; word_len], |acc, binary| {
        acc.iter()
            .zip(binary)
            .map(|(&a, &b)| a + b as isize)
            .collect()
    });
    let gamma = isize::from_str_radix(
        &sums
            .iter()
            .map(|&x| {
                if x > input.len() as isize / 2 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>(),
        2,
    )
    .map_err(|_| "Could not parse gamma")?;

    let epsilon = isize::from_str_radix(
        &sums
            .iter()
            .map(|&x| {
                if x <= input.len() as isize / 2 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>(),
        2,
    )
    .map_err(|_| "Could not parse epsilon")?;
    println!("{}", gamma * epsilon);
    Ok(())
}
