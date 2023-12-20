struct GameSettings {
    max_red: i32,
    max_blue: i32,
    max_green: i32,
}

impl GameSettings {
    fn check_color(self: &Self, color: &str, size: i32) -> bool {
        match color {
            "blue" => size <= self.max_blue,
            "red" => size <= self.max_red,
            "green" => size <= self.max_green,
            _ => false,
        }
    }
}

fn main() {
    let game_settings = GameSettings {
        max_red: 12,
        max_blue: 14,
        max_green: 13,
    };

    let data = include_str!("./input1.txt");
    let result: i32 = data
        .split('\n')
        .take_while(|row| row.to_string().len() > 0)
        .map(|game| {
            let rounds = game.split(':').last().unwrap().split(';');
            let valid_game = rounds.fold(true, |acc, round| {
                if !acc {
                    return acc;
                }
                let turns = round.split(',');
                let valid_round = turns.fold(true, |acc, turn| {
                    if !acc {
                        return acc;
                    }
                    let pieces = turn.split(' ');
                    let size: i32 = pieces.clone().take(2).last().unwrap().parse().unwrap();
                    let color = pieces.last().unwrap();

                    game_settings.check_color(color, size)
                });
                valid_round
            });

            if valid_game {
                return game
                    .split(':')
                    .next()
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            0
        })
        .sum();

    println!("{}", result)
}
