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

enum FilterCriteria {
    MostCommon,
    LeastCommon,
}

fn filter_by_criteria(
    lines: Vec<Vec<u8>>,
    position: usize,
    filter_criteria: FilterCriteria,
) -> Vec<Vec<u8>> {
    if lines.len() == 1 {
        return lines;
    }
    let len: usize = lines.len();
    let sum: usize = lines.iter().map(|x| x[position] as usize).sum();
    let filter_bit: u8 = match filter_criteria {
        FilterCriteria::MostCommon => {
            if 2 * sum >= len {
                1
            } else {
                0
            }
        }
        FilterCriteria::LeastCommon => {
            if 2 * sum >= len {
                0
            } else {
                1
            }
        }
    };

    let new = lines
        .into_iter()
        .filter(|x| x[position] == filter_bit)
        .collect::<Vec<_>>();
    filter_by_criteria(new, position + 1, filter_criteria)
}

fn find_gamma(lines: &Vec<Vec<u8>>) -> isize {
    bin_vec_to_number(&filter_by_criteria(lines.clone(), 0, FilterCriteria::MostCommon)[0])
}

fn find_epsilon(lines: &Vec<Vec<u8>>) -> isize {
    bin_vec_to_number(&filter_by_criteria(lines.clone(), 0, FilterCriteria::LeastCommon)[0])
}

fn bin_vec_to_number(bitvector: &[u8]) -> isize {
    bitvector.iter().fold(0, |acc, bit| acc * 2 + *bit as isize)
}

fn main() -> Result<(), &'static str> {
    let input =
        read_input("inputs/03b-binary-diagnostic.input").map_err(|_| "Could not read file")?;

    let gamma = find_gamma(&input);
    let epsilon = find_epsilon(&input);

    println!("{}", gamma * epsilon);
    Ok(())
}
