use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

fn calculate_priorities(line: &str) -> std::io::Result<u32> {
    let chunks = line.chars().chunks(line.len() / 2);
    let mut iter = chunks.into_iter();
    let (Some(first), Some(mut second), None) = (iter.next(), iter.next(), iter.next()) else {
        return Err(std::io::Error::new(ErrorKind::InvalidData, "Error parsing line"));
    };
    let ht: HashSet<char> = HashSet::from_iter(first);
    let Some(dup) = second.find(|ch| ht.contains(ch)) else {
        return Err(std::io::Error::new(ErrorKind::InvalidData, "No duplicate found"));
    };
    Ok(if dup.is_lowercase() {
        ((dup as u32) - ('a' as u32)) + 1
    } else {
        ((dup as u32) - ('A' as u32)) + 27
    })
}

fn strategy_points(file_path: &Path) -> std::io::Result<u32> {
    Ok(BufReader::new(File::open(file_path)?)
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?
        .iter()
        .map(|line| calculate_priorities(line))
        .collect::<std::io::Result<Vec<_>>>()?
        .iter()
        .sum())
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
        assert_eq!(strategy_points(&fixtures_dir().join("input1.txt"))?, 157);
        assert_eq!(strategy_points(&fixtures_dir().join("input2.txt"))?, 7793);
        Ok(())
    }
}

fn main() {}
