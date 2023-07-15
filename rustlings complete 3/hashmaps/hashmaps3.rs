// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.



use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        {
            let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
                name: team_1_name.clone(),
                goals_scored: 0,
                goals_conceded: 0,
            });

            team_1.goals_scored += team_1_score;
            let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
                name: team_2_name.clone(),
                goals_scored: 0,
                goals_conceded: 0,
            });

            team_2.goals_conceded += team_1_score;
        }

        {
            let team_2 = scores.get_mut(&team_2_name).unwrap();
            team_2.goals_scored += team_2_score;
            let team_1 = scores.get_mut(&team_1_name).unwrap();
            team_1.goals_conceded += team_2_score;
        }
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rest of the code...
}
