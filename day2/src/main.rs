use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::Path;

enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" | "A" => Ok(Hand::Rock),
            "Y" | "B" => Ok(Hand::Paper),
            "Z" | "C" => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (other, self) {
            (&Hand::Rock, &Hand::Paper) => Some(Ordering::Greater),
            (&Hand::Scissors, &Hand::Rock) => Some(Ordering::Greater),
            (&Hand::Paper, &Hand::Scissors) => Some(Ordering::Greater),
            (&Hand::Rock, &Hand::Scissors) => Some(Ordering::Less),
            (&Hand::Scissors, &Hand::Paper) => Some(Ordering::Less),
            (&Hand::Paper, &Hand::Rock) => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

fn calculate_points(line: Option<(&str, &str)>) -> std::io::Result<u32> {
    let Some((other, mine)) = line else {
        return Err(std::io::Error::new(ErrorKind::InvalidData, "Line contains != 2 elements"));
    };
    let (Ok(other), Ok(mine)) = (Hand::try_from(other), Hand::try_from(mine)) else {
        return Err(std::io::Error::new(ErrorKind::InvalidData, "Error parsing hand info"));
    };
    Ok((if other < mine {
        6
    } else if other > mine {
        0
    } else {
        3
    }) + mine as u32)
}

fn strategy_points(file_path: &Path) -> std::io::Result<u32> {
    Ok(BufReader::new(File::open(file_path)?)
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?
        .iter()
        .map(|line| calculate_points(line.split(' ').collect_tuple()))
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
        assert_eq!(strategy_points(&fixtures_dir().join("input1.txt"))?, 15);
        assert_eq!(strategy_points(&fixtures_dir().join("input2.txt"))?, 12772);
        Ok(())
    }
}

fn main() {}
