use color_eyre::eyre::{eyre, Result};

#[derive(Debug)]
struct Priorities([char; 52]);

impl Priorities {
    pub fn priority_for_char(&self, c: char) -> u8 {
        self.0
            .iter()
            .position(|p| *p == c)
            // SAFETY: Safe because we do not have more than 52 items so it cannot ever overflow u8::MAX
            .map(|pos| (pos + 1) as u8)
            .expect("Out of range priority")
    }
}

impl Default for Priorities {
    fn default() -> Self {
        let res = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();
        assert_eq!(res.len(), 52);
        let mut priorities = [char::default(); 52];
        priorities.copy_from_slice(res.as_slice());
        Self(priorities)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Container(String);

impl Container {
    pub fn cumulated_priorities(&self, priorities: &Priorities) -> u64 {
        self.0
            .chars()
            .map(|c| priorities.priority_for_char(c) as u64)
            .sum()
    }
}

impl From<&str> for Container {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Container {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rucksack {
    c1: Container,
    c2: Container,
}

impl Rucksack {
    pub fn new_from_str(s: String) -> Result<Self> {
        if s.len() % 2 != 0 {
            return Err(eyre!(
                "Rucksack contents are not even. Cannot split into compartments!"
            ));
        }

        let (c1, c2) = s.split_at(s.len() / 2);
        Ok(Self {
            c1: c1.into(),
            c2: c2.into(),
        })
    }

    pub fn to_string(&self) -> String {
        let mut res = self.c1.0.clone();
        res.push_str(&self.c2.0);
        res
    }

    pub fn common_items(&self) -> Container {
        let c2_chars: Vec<char> = self.c2.0.chars().collect();
        let mut common_chars: Vec<char> = self
            .c1
            .0
            .chars()
            .filter(|item| c2_chars.contains(item))
            .collect();

        common_chars.dedup();

        common_chars.into_iter().collect::<String>().into()
    }

    pub fn common_items_with_group(&self, two: &Self, three: &Self) -> Container {
        let one_str = self.to_string();
        let two_str = two.to_string();
        let three_str = three.to_string();

        let two_chars: Vec<char> = two_str.chars().collect();
        let three_chars: Vec<char> = three_str.chars().collect();

        let mut common = one_str
            .chars()
            .filter(|c| two_chars.contains(c) && three_chars.contains(c))
            .collect::<Vec<char>>();
        common.dedup();

        common.into_iter().collect::<String>().into()
    }
}

#[derive(Debug, Clone, Default)]
struct RucksackGroup(Vec<Rucksack>);

impl RucksackGroup {
    pub fn cumulated_priority_sum(&self, priorities: &Priorities) -> u64 {
        self.0
            .iter()
            .map(|rs| rs.common_items().cumulated_priorities(&priorities))
            .sum::<u64>()
    }

    pub fn group_badge_priority_sum(&self, priorities: &Priorities) -> u64 {
        self.0
            .chunks_exact(3)
            .map(|rucksacks| {
                let a = &rucksacks[0];
                let b = &rucksacks[1];
                let c = &rucksacks[2];

                a.common_items_with_group(b, c)
                    .cumulated_priorities(&priorities)
            })
            .sum()
    }
}

fn main() -> Result<()> {
    use std::io::BufRead as _;

    let file = std::fs::File::open("./src/rucksack_list.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let mut rucksack_group = RucksackGroup::default();

    for line in lines {
        if let Ok(rucksack_line) = line {
            if rucksack_line.is_empty() {
                continue;
            }

            rucksack_group
                .0
                .push(Rucksack::new_from_str(rucksack_line)?);
        }
    }

    let priorities = Priorities::default();

    println!(
        "Step1: Cumulated priorities: {}",
        rucksack_group.cumulated_priority_sum(&priorities)
    );

    println!(
        "Step2: Group badge cumulated priority sum: {}",
        rucksack_group.group_badge_priority_sum(&priorities)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conforms_to_brief_step1() {
        let priorities = Priorities::default();
        let expected_results = [
            ("p", 16u64),
            ("L", 38),
            ("P", 42),
            ("v", 22),
            ("t", 20),
            ("s", 19),
        ];

        let rucksacks = RucksackGroup(vec![
            Rucksack::new_from_str("vJrwpWtwJgWrhcsFMMfFFhFp".into()).unwrap(),
            Rucksack::new_from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into()).unwrap(),
            Rucksack::new_from_str("PmmdzqPrVvPwwTWBwg".into()).unwrap(),
            Rucksack::new_from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into()).unwrap(),
            Rucksack::new_from_str("ttgJtRGJQctTZtZT".into()).unwrap(),
            Rucksack::new_from_str("CrZsJsPPZsGzwwsLwLmpwMDw".into()).unwrap(),
        ]);

        for (rucksack, (expected_str, expected_priority)) in
            rucksacks.0.iter().zip(expected_results.iter())
        {
            let common = rucksack.common_items();
            assert_eq!(common.0, *expected_str);
            assert_eq!(common.cumulated_priorities(&priorities), *expected_priority);
        }

        assert_eq!(rucksacks.cumulated_priority_sum(&priorities), 157);
    }
}
