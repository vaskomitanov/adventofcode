use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn calc_calories(file_path: &Path) -> std::io::Result<u32> {
    Ok(BufReader::new(File::open(file_path)?)
        .lines()
        .group_by(|line| line.as_ref().map_or(false, |v| !v.is_empty()))
        .into_iter()
        .map(|(_, g)| {
            g.map(|v| v.map_or_else(|_| 0, |s| s.parse::<u32>().unwrap_or_else(|_| 0)))
                .sum()
        })
        .max()
        .unwrap_or(0))
}

fn main() {}

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
    fn test_calc_calories() -> io::Result<()> {
        assert_eq!(calc_calories(&fixtures_dir().join("input1.txt"))?, 24000);
        assert_eq!(calc_calories(&fixtures_dir().join("input2.txt"))?, 70613);
        Ok(())
    }
}
