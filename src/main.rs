use std::collections::HashSet;

use itertools::Itertools;

/// Part 1: find two numbers that add up to 2020, then multiply.
/// Part 2: find three numbers that add up to 2020, then multiply.

fn read_reports() -> Vec<u32> {
    // Thanks to Pierre for learning this macro + split_terminator()!
    let data = include_str!("input.txt");
    data.split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

/// O(n), thanks to the HashSet.
fn find_pair(reports: &[u32]) -> Option<(u32, u32)> {
    let reports: HashSet<&u32> = reports.iter().collect();
    for report in &reports {
        let complement = 2020u32.checked_sub(**report);
        match complement {
            None => continue,
            Some(complement) => match reports.get(&complement) {
                None => continue,
                Some(second) => {
                    let mut result = [**report, **second];
                    // NB: We sort for determinism, given that we use a hash set.
                    result.sort_unstable();
                    return Some((result[0], result[1]));
                }
            },
        }
    }
    None
}

/// O(n^3).
fn find_triplet(reports: &[u32]) -> Option<(u32, u32, u32)> {
    reports.iter().combinations(3).find_map(|comb| {
        let (v0, v1, v2) = (*comb[0], *comb[1], *comb[2]);
        if v0 + v1 + v2 == 2020 {
            Some((v0, v1, v2))
        } else {
            None
        }
    })
}

fn main() {
    let reports = read_reports();
    match find_pair(&reports) {
        None => {
            eprintln!("No two reports found that add up to 2020.");
            std::process::exit(1);
        }
        Some((first, second)) => {
            let result = first * second;
            println!("Pair result: {}", result)
        }
    }
    match find_triplet(&reports) {
        None => {
            eprintln!("No three reports found that add up to 2020.");
            std::process::exit(1);
        }
        Some((first, second, third)) => {
            let result = first * second * third;
            println!("Triplet result: {}", result)
        }
    }
}

#[test]
fn test_find_pair() {
    fn assert_pair(input: &[u32], expected: Option<(u32, u32)>) {
        let input: HashSet<u32> = input.into_iter().map(|x| *x).collect();
        assert_eq!(find_pair(&input), expected);
    }
    assert_pair(&vec![2020, 10, 0], Some((0, 2020)));
    assert_pair(&vec![100, 10, 2010], Some((10, 2010)));
    // Can handle #s > 2020
    assert_pair(&vec![3000, 1000, 1020], Some((1000, 1020)));
    assert_pair(&vec![10, 20], None);
}
