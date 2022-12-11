use color_eyre::eyre::{eyre, Result};

type CalorieValue = u64;
#[derive(Debug)]
struct Food {
    calorie_value: CalorieValue,
}

impl Food {
    pub fn new(calorie_value: CalorieValue) -> Self {
        Self { calorie_value }
    }
}

#[derive(Debug)]
struct Elf {
    id: u64,
    food_carried: Vec<Food>,
}

impl Elf {
    pub fn new(id: u64, food_carried: &[CalorieValue]) -> Self {
        Self {
            id,
            food_carried: food_carried
                .into_iter()
                .map(|calorie_value| Food::new(*calorie_value))
                .collect(),
        }
    }

    pub fn total_calories_carried(&self) -> CalorieValue {
        self.food_carried
            .iter()
            .fold(0u64, |acc, food: &Food| acc + food.calorie_value)
    }
}

#[derive(Debug, Default)]
struct ElfGroup(Vec<Elf>);

impl ElfGroup {
    pub fn add_elf(&mut self, food_carried: &[CalorieValue]) -> &mut Self {
        let new_id = self.0.len() + 1;
        self.0.push(Elf::new(new_id as u64, food_carried));
        self
    }

    pub fn elf_with_most_calories(&self) -> Option<&Elf> {
        if self.0.is_empty() {
            return None;
        }

        self.0.iter().max_by_key(|elf| elf.total_calories_carried())
    }

    pub fn top_3_elves_calories(&self) -> CalorieValue {
        let mut calories: Vec<CalorieValue> =
            self.0.iter().map(Elf::total_calories_carried).collect();
        calories.sort_by_key(|cal| std::cmp::Reverse(*cal));
        calories.into_iter().take(3).sum()
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    use std::io::BufRead as _;

    let file = std::fs::File::open("./src/elf_list.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let mut elves = ElfGroup::default();

    let mut cur_food_carried: Vec<CalorieValue> = vec![];

    for line in lines {
        if let Ok(calorie_value) = line {
            if calorie_value.is_empty() {
                elves.add_elf(cur_food_carried.as_slice());
                cur_food_carried.clear();
            } else {
                cur_food_carried.push(calorie_value.parse()?);
            }
        }
    }

    let chad_elf = elves
        .elf_with_most_calories()
        .ok_or_else(|| eyre!("Elves list is empty!"))?;

    println!(
        "Chad elf is elf #{} with {} calories carried",
        chad_elf.id,
        chad_elf.total_calories_carried()
    );

    println!(
        "Top 3 elves sum of calories: {}",
        elves.top_3_elves_calories()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conforms_to_brief_step1() {
        let mut elves = ElfGroup::default();
        elves
            .add_elf(&[1000, 2000, 3000])
            .add_elf(&[4000])
            .add_elf(&[5000, 6000])
            .add_elf(&[7000, 8000, 9000])
            .add_elf(&[10000]);

        let chad_elf = elves.elf_with_most_calories().unwrap();
        assert_eq!(chad_elf.id, 4);
        assert_eq!(chad_elf.total_calories_carried(), 24000);
    }

    #[test]
    fn conforms_to_brief_step2() {
        let mut elves = ElfGroup::default();
        elves
            .add_elf(&[1000, 2000, 3000])
            .add_elf(&[4000])
            .add_elf(&[5000, 6000])
            .add_elf(&[7000, 8000, 9000])
            .add_elf(&[10000]);

        assert_eq!(elves.top_3_elves_calories(), 45000);
    }
}
