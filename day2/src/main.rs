use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::EnumString, strum::AsRefStr)]
#[repr(u64)]
enum Choice {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

impl Choice {
    pub fn wins_against(&self, other: Choice) -> bool {
        match self {
            Choice::Rock => other == Choice::Scissors,
            Choice::Paper => other == Choice::Rock,
            Choice::Scissors => other == Choice::Paper,
        }
    }

    pub fn points(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u64)]
enum ChoiceFightOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl ChoiceFightOutcome {
    pub fn points(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug)]
struct ChoiceFight {
    opponent: Choice,
    me: Choice,
}

impl ChoiceFight {
    pub fn outcome(&self) -> ChoiceFightOutcome {
        if self.opponent == self.me {
            ChoiceFightOutcome::Draw
        } else if self.me.wins_against(self.opponent) {
            ChoiceFightOutcome::Win
        } else {
            ChoiceFightOutcome::Loss
        }
    }
}

#[derive(Debug, Default)]
struct MatchResult {
    opponent: u64,
    me: u64,
}

#[derive(Debug, Default)]
struct StrategyGuide(Vec<ChoiceFight>);

impl StrategyGuide {
    pub fn points_scored(&self) -> MatchResult {
        self.0
            .iter()
            .fold(MatchResult::default(), |mut result, fight| {
                let mut points_to_add_me = fight.me.points();
                let mut points_to_add_opponent = fight.opponent.points();
                match fight.outcome() {
                    ChoiceFightOutcome::Loss => {
                        points_to_add_opponent += ChoiceFightOutcome::Win.points()
                    }
                    o @ ChoiceFightOutcome::Draw => {
                        points_to_add_opponent += o.points();
                        points_to_add_me += o.points();
                    }
                    o @ ChoiceFightOutcome::Win => {
                        points_to_add_me += o.points();
                    }
                }

                result.me += points_to_add_me;
                result.opponent += points_to_add_opponent;

                result
            })
    }
}

fn main() -> Result<()> {
    use std::io::BufRead as _;
    use std::str::FromStr as _;

    let mut guide = StrategyGuide::default();

    let file = std::fs::File::open("./src/rps_strategy_guide.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(strategy_line) = line {
            if strategy_line.is_empty() {
                continue;
            }

            let choices = strategy_line
                .split(" ")
                .map(|choice| Ok(Choice::from_str(choice)?))
                .collect::<Result<Vec<Choice>>>()?;

            assert_eq!(choices.len(), 2);
            let opponent = choices[0];
            let me = choices[1];

            let fight = ChoiceFight { me, opponent };

            guide.0.push(fight);
        }
    }

    let MatchResult { me, opponent } = guide.points_scored();

    println!("Match result: me [{me}] vs opponent [{opponent}]");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn conforms_to_brief_step1() {
        let strategy_guide = StrategyGuide(vec![
            ChoiceFight {
                opponent: Choice::from_str("A").unwrap(),
                me: Choice::from_str("Y").unwrap(),
            },
            ChoiceFight {
                opponent: Choice::from_str("B").unwrap(),
                me: Choice::from_str("X").unwrap(),
            },
            ChoiceFight {
                opponent: Choice::from_str("C").unwrap(),
                me: Choice::from_str("Z").unwrap(),
            },
        ]);

        assert_eq!(strategy_guide.0[0].opponent, Choice::Rock);
        assert_eq!(strategy_guide.0[0].me, Choice::Paper);
        assert_eq!(strategy_guide.0[1].opponent, Choice::Paper);
        assert_eq!(strategy_guide.0[1].me, Choice::Rock);
        assert_eq!(strategy_guide.0[2].opponent, Choice::Scissors);
        assert_eq!(strategy_guide.0[2].me, Choice::Scissors);

        let MatchResult { me, opponent } = strategy_guide.points_scored();
        assert_eq!(me, 15);
        assert_eq!(opponent, 15);
    }

    #[test]
    fn conforms_to_brief_step2() {}
}
