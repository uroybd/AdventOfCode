// Advent of Code 2021 - Day 21

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Player {
    position: usize,
    score: usize,
}

fn deterministic_dice_game(players: &mut Vec<Player>, till: usize, score_limit: usize) -> usize {
    let (mut total_roll, mut player_idx, mut dice) = (0, 0, 0);

    loop {
        total_roll += 3;

        let mut movement = dice + 1;
        for _ in 1..3 {
            dice = (dice + 1) % till;
            movement += dice + 1;
        }
        let cur_player = players.get_mut(player_idx).unwrap();

        let next_pos = (cur_player.position + movement) % 10;
        cur_player.position = next_pos;
        cur_player.score += next_pos + 1;
        if cur_player.score >= score_limit {
            if player_idx == 0 {
                return players[1].score * total_roll;
            } else {
                return players[0].score * total_roll;
            }
        } else {
            player_idx = match player_idx {
                0 => 1,
                _ => 0,
            };
        }
        dice = (dice + 1) % till;
        if dice >= till {
            dice = 0;
        }
    }
}

fn diracs_dice_game(
    p1: Player,
    p2: Player,
    limit: usize,
    cache: &mut HashMap<(Player, Player, usize), (usize, usize)>,
) -> (usize, usize) {
    if p2.score >= limit {
        (0, 1)
    } else {
        let dice_frequencies = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        for (movement, freq) in dice_frequencies {
            let next_pos = (p1.position + movement) % 10;
            let next_score = p1.score + next_pos + 1;
            let new_p = Player {
                position: next_pos,
                score: next_score,
            };
            let mut nested_games = (0, 0);
            if cache.contains_key(&(p2, new_p, limit)) {
                nested_games = *cache.get(&(p2, new_p, limit)).unwrap();
            } else {
                nested_games = diracs_dice_game(p2, new_p, limit, cache);
                cache.insert((p2, new_p, limit), nested_games);
            }
            let (p2_quantum, p1_quantum) = nested_games;
            p1_wins += p1_quantum * freq;
            p2_wins += p2_quantum * freq;
        }
        (p1_wins, p2_wins)
    }
}

pub fn solution_2021_21_01(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let positions = vec![8, 9];
    let players: Vec<Player> = positions
        .iter()
        .map(|&x| Player {
            position: x - 1,
            score: 0,
        })
        .collect();
    let res = deterministic_dice_game(&mut players.clone(), 100, 1000);
    Ok(res as i64)
}

pub fn solution_2021_21_02(_filepath: String) -> Result<i64, Box<dyn std::error::Error>> {
    let positions = vec![8, 9];
    let players: Vec<Player> = positions
        .iter()
        .map(|&x| Player {
            position: x - 1,
            score: 0,
        })
        .collect();
    let mut cache: HashMap<(Player, Player, usize), (usize, usize)> = HashMap::new();
    let res = diracs_dice_game(players[0], players[1], 21, &mut cache);
    Ok(res.0.max(res.1) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2021_21_01() {
        let result = solution_2021_21_01("inputs/2021/day21e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_21_01() {
        let result = solution_2021_21_01("inputs/2021/day21.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn test_2021_21_02() {
        let result = solution_2021_21_02("inputs/2021/day21e.txt".to_string()).unwrap();
        assert!(result > 0);
    }

    #[test]
    #[ignore]
    fn output_2021_21_02() {
        let result = solution_2021_21_02("inputs/2021/day21.txt".to_string()).unwrap();
        println!("{:?}", result);
        assert!(result > 0);
    }
}
