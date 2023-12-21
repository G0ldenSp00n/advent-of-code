use std::char;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthToEastBend,
    NorthToWestBend,
    SouthToWestBend,
    SouthToEastBend,
    Ground,
    StartingPosition,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row_index: usize,
    col_index: usize,
}

#[derive(Debug)]
struct PipeNetwork {
    zone: Vec<Vec<Tile>>,
    animal: Position,
}

impl PipeNetwork {
    fn new(input: &str) -> Result<Self, &str> {
        let mut position = None;
        let zone = input
            .split("\n")
            .filter(|row| !row.is_empty())
            .enumerate()
            .map(|(row_index, rows)| {
                rows.chars()
                    .enumerate()
                    .map(|(col_index, char)| {
                        if char == 'S' {
                            position = Some(Position {
                                row_index,
                                col_index,
                            });
                        }
                        match char {
                            '|' => Tile::VerticalPipe,
                            '-' => Tile::HorizontalPipe,
                            'L' => Tile::NorthToEastBend,
                            'J' => Tile::NorthToWestBend,
                            '7' => Tile::SouthToWestBend,
                            'F' => Tile::SouthToEastBend,
                            '.' => Tile::Ground,
                            'S' => Tile::StartingPosition,
                            _ => panic!(),
                        }
                    })
                    .collect()
            })
            .collect();

        if let Some(animal) = position {
            Ok(PipeNetwork { zone, animal })
        } else {
            Err("no animal")
        }
    }

    fn follow_pipe(&self, position: Position, prev_position: Position) -> Position {
        use Tile::*;
        let tile = self
            .zone
            .get(position.row_index)
            .unwrap()
            .get(position.col_index)
            .unwrap();
        match tile {
            VerticalPipe => {
                if position.row_index > prev_position.row_index {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index + 1,
                    }
                } else {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index - 1,
                    }
                }
            }
            HorizontalPipe => {
                if position.col_index > prev_position.col_index {
                    Position {
                        col_index: position.col_index + 1,
                        row_index: position.row_index,
                    }
                } else {
                    Position {
                        col_index: position.col_index - 1,
                        row_index: position.row_index,
                    }
                }
            }
            NorthToEastBend => {
                // .X..
                // .LX.
                // ....
                if position.col_index == prev_position.col_index {
                    Position {
                        col_index: position.col_index + 1,
                        row_index: position.row_index,
                    }
                } else {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index - 1,
                    }
                }
            }
            NorthToWestBend => {
                // ..X..
                // .XJ..
                // .....
                if position.col_index == prev_position.col_index {
                    Position {
                        col_index: position.col_index - 1,
                        row_index: position.row_index,
                    }
                } else {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index - 1,
                    }
                }
            }
            SouthToEastBend => {
                // ....
                // .FX.
                // .X..
                if position.col_index == prev_position.col_index {
                    Position {
                        col_index: position.col_index + 1,
                        row_index: position.row_index,
                    }
                } else {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index + 1,
                    }
                }
            }
            SouthToWestBend => {
                // ....
                // .X7.
                // ..X.
                if position.col_index == prev_position.col_index {
                    Position {
                        col_index: position.col_index - 1,
                        row_index: position.row_index,
                    }
                } else {
                    Position {
                        col_index: position.col_index,
                        row_index: position.row_index + 1,
                    }
                }
            }
            StartingPosition => {
                for row_offset in -1..=1 {
                    for col_offset in -1..=1 {
                        if let Ok(col_index) = usize::try_from(
                            isize::try_from(position.col_index).unwrap() + col_offset,
                        ) {
                            if let Ok(row_index) = usize::try_from(
                                isize::try_from(position.row_index).unwrap() + row_offset,
                            ) {
                                if let Some(row) = self.zone.get(row_index) {
                                    let tile = row.get(col_index).unwrap_or(&Ground);
                                    let position = Position {
                                        col_index: usize::try_from(
                                            isize::try_from(position.col_index).unwrap()
                                                + col_offset,
                                        )
                                        .unwrap_or_default(),
                                        row_index: usize::try_from(
                                            isize::try_from(position.row_index).unwrap()
                                                + row_offset,
                                        )
                                        .unwrap_or_default(),
                                    };

                                    match tile {
                                        VerticalPipe => {
                                            if col_offset == 0 && row_offset != 0 {
                                                return position;
                                            }
                                        }
                                        HorizontalPipe => {
                                            if row_offset == 0 && col_offset != 0 {
                                                return position;
                                            }
                                        }
                                        NorthToEastBend => {
                                            if row_offset == 1 && col_offset == 0 {
                                                return position;
                                            }

                                            if row_offset == 0 && col_offset == -1 {
                                                return position;
                                            }
                                        }
                                        NorthToWestBend => {
                                            if row_offset == 1 && col_offset == 0 {
                                                return position;
                                            }
                                            if row_offset == 0 && col_offset == 1 {
                                                return position;
                                            }
                                        }
                                        SouthToWestBend => {
                                            if row_offset == -1 && col_offset == 0 {
                                                return position;
                                            }
                                            if row_offset == 0 && col_offset == -1 {
                                                return position;
                                            }
                                        }
                                        SouthToEastBend => {
                                            if row_offset == -1 && col_offset == 0 {
                                                return position;
                                            }
                                            if row_offset == 0 && col_offset == 1 {
                                                return position;
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                }
                panic!()
            }
            Ground => position,
        }
    }

    fn find_furthest_point(&self) -> i32 {
        let mut prev_position = self.animal;
        let mut next_position = self.follow_pipe(self.animal, self.animal);
        let mut length = 1;
        while self
            .zone
            .get(next_position.row_index)
            .unwrap()
            .get(next_position.col_index)
            .unwrap()
            != &Tile::StartingPosition
        {
            length += 1;
            let new_position = self.follow_pipe(next_position, prev_position);
            prev_position = next_position;
            next_position = new_position;
        }
        length / 2
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let pipe_network = PipeNetwork::new(input).expect("Should be a valid network");
    let result = pipe_network.find_furthest_point();
    println!("{result}");
}
