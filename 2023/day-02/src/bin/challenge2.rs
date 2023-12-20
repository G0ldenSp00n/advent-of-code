use std::cmp;

fn main() {
    let data = include_str!("./input2.txt");
    let result: i32 = data
        .split('\n')
        .take_while(|row| row.to_string().len() > 0)
        .map(|game| {
            let rounds = game.split(':').last().unwrap().split(';');
            let game_max = rounds.fold((0, 0, 0), |acc, round| {
                let turns = round.split(',');
                let round_max = turns.fold((0, 0, 0), |acc, turn| {
                    let pieces = turn.split(' ');
                    let size: i32 = pieces.clone().take(2).last().unwrap().parse().unwrap();
                    let color = pieces.last().unwrap();

                    match color {
                        "red" => {
                            return (size, acc.1, acc.2);
                        }
                        "green" => {
                            return (acc.0, size, acc.2);
                        }
                        "blue" => {
                            return (acc.0, acc.1, size);
                        }
                        _ => acc,
                    }
                });
                (
                    cmp::max(round_max.0, acc.0),
                    cmp::max(round_max.1, acc.1),
                    cmp::max(round_max.2, acc.2),
                )
            });

            game_max.0 * game_max.1 * game_max.2
        })
        .sum();

    println!("{}", result)
}
