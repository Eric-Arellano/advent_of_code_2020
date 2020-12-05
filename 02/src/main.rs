use std::collections::HashMap;

/// Find the number of valid passwords.

struct PasswordPolicy {
    letter: char,
    num1: u8,
    num2: u8,
}

impl PasswordPolicy {
    fn new(letter: char, num1: u8, num2: u8) -> PasswordPolicy {
        PasswordPolicy { letter, num1, num2 }
    }

    /// The letter appears a # of times within the given range (inclusive).
    /// O(c), where c is # characters.
    fn is_valid_policy1(&self, s: &str) -> bool {
        let mut char_map = HashMap::with_capacity(s.len().min(26));
        for c in s.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        let count = *char_map.get(&self.letter).unwrap_or(&0);
        count >= self.num1 && count <= self.num2
    }

    /// The letter appears in exactly one of the positions (1-indexed).
    /// O(1), where c is # characters.
    fn is_valid_policy2(&self, s: &str) -> bool {
        let char_matches = |i: u8| {
            if i < 1 {
                return false
            }
            // Adjust from one-index to zero-index.
            let i = i - 1;
            match s.chars().nth(i as usize) {
                None => false,
                Some(c) => c == self.letter,
            }
        };
        char_matches(self.num1) ^ char_matches(self.num2)
    }
}

fn read_data() -> Vec<(PasswordPolicy, String)> {
    let data = include_str!("input.txt");
    data.split_terminator('\n')
        // Parse the format `min-max char: str`
        .map(|s| {
            let val_split: Vec<&str> = s.splitn(2, ": ").collect();
            let policy_split: Vec<&str> = val_split[0].splitn(2, ' ').collect();
            let letter = policy_split[1].chars().nth(0).unwrap();
            let range_split: Vec<&str> = policy_split[0].splitn(2, "-").collect();
            let num1: u8 = range_split[0].parse().unwrap();
            let num2: u8 = range_split[1].parse().unwrap();
            let policy = PasswordPolicy::new(letter, num1, num2);
            (policy, val_split[1].to_string())
        })
        .collect()
}

fn main() {
    let data = read_data();
    let num_valid_pt1 = data
        .iter()
        .filter(|(policy, s)| policy.is_valid_policy1(s))
        .count();
    let num_valid_pt2 = data
        .iter()
        .filter(|(policy, s)| policy.is_valid_policy2(s))
        .count();
    println!("Num valid passwords pt 1: {}", num_valid_pt1);
    println!("Num valid passwords pt 2: {}", num_valid_pt2);
}

#[test]
fn test_policy_1() {
    fn assert_is_valid(min: u8, max: u8, input: &str, expected: bool) {
        let policy = PasswordPolicy::new('a', min, max);
        assert_eq!(policy.is_valid_policy1(input), expected);
    }
    assert_is_valid(0, 1, "", true);
    assert_is_valid(0, 1, "a", true);
    assert_is_valid(0, 1, "aa", false);
    assert_is_valid(1, 2, "bca", true);
}

#[test]
fn test_policy_2() {
    fn assert_is_valid(pos1: u8, pos2: u8, input: &str, expected: bool) {
        let policy = PasswordPolicy::new('a', pos1, pos2);
        assert_eq!(policy.is_valid_policy2(input), expected);
    }
    assert_is_valid(1, 2, "", false);
    assert_is_valid(1, 2, "a", true);
    assert_is_valid(1, 2, "ab", true);
    assert_is_valid(1, 2, "ba", true);
    assert_is_valid(1, 2, "bca", false);
    assert_is_valid(1, 2, "aa", false);
    // O is invalid because it's zero-indexed, but we should handle it.
    assert_is_valid(0, 1, "ba", false);
}
