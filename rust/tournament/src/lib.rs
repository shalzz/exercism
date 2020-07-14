use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(Default, Debug)]
struct Score {
    won: u32,
    draw: u32,
    loss: u32,
}

impl Score {
    fn played(&self) -> u32 {
        self.won + self.draw + self.loss
    }

    fn points(&self) -> u32 {
        3 * self.won + self.draw
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|  {} |  {} |  {} |  {} |  {}",
            self.played(),
            self.won,
            self.draw,
            self.loss,
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table = HashMap::<String, Score>::new();
    for line in match_results.lines() {
        let array: Vec<&str> = line.split(';').collect();
        let mut first_team = table.remove(array[0]).unwrap_or_default();
        let mut second_team = table.remove(array[1]).unwrap_or_default();
        match array[2] {
            "win" => {
                first_team.won += 1;
                second_team.loss += 1;
            }
            "loss" => {
                first_team.loss += 1;
                second_team.won += 1;
            }
            "draw" => {
                first_team.draw += 1;
                second_team.draw += 1
            }
            _ => unreachable!(),
        }
        table.insert(array[0].to_string(), first_team);
        table.insert(array[1].to_string(), second_team);
    }

    let mut result = String::default();
    let mut vec: Vec<(String, Score)> = table.into_iter().collect();
    vec.sort_unstable_by(|a, b| b.1.points().cmp(&a.1.points()).then(a.0.cmp(&b.0)));

    result.push_str("Team                           | MP |  W |  D |  L |  P\n");
    for team in vec {
        result.push_str(&format!("{:31}{}\n", team.0, team.1))
    }
    result.trim_end().to_string()
}
