use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn get_bounds(line: &str) -> std::io::Result<(i32, i32)> {
    let mut pair = line.split('-');
    if let (Some(lbound), Some(ubound), None) = (pair.next(), pair.next(), pair.next()) {
        return Ok((lbound.parse().unwrap(), ubound.parse().unwrap()));
    }
    Err(Error::new(ErrorKind::InvalidData, ""))
}

fn calculate_overlaps(file_path: &Path) -> std::io::Result<usize> {
    Ok(BufReader::new(File::open(file_path)?)
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?
        .iter()
        .filter_map(|line| {
            let mut line_iter = line.split(',');
            if let (Some(pair_1), Some(pair_2), None) =
                (line_iter.next(), line_iter.next(), line_iter.next())
            {
                let pair_1_bounds = get_bounds(pair_1).unwrap();
                let pair_2_bounds = get_bounds(pair_2).unwrap();
                if (pair_1_bounds.0 <= pair_2_bounds.0 && pair_1_bounds.1 >= pair_2_bounds.1)
                    || (pair_2_bounds.0 <= pair_1_bounds.0 && pair_2_bounds.1 >= pair_1_bounds.1)
                {
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io;
    use std::path::PathBuf;

    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or(String::new())).join("fixtures")
    }

    #[test]
    fn test_strategy_points() -> io::Result<()> {
        assert_eq!(calculate_overlaps(&fixtures_dir().join("input1.txt"))?, 2);
        assert_eq!(calculate_overlaps(&fixtures_dir().join("input2.txt"))?, 1);
        Ok(())
    }
}

fn main() {}
