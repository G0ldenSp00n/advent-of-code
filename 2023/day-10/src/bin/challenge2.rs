use std::char;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Self::VerticalPipe => '|',
            Self::HorizontalPipe => '-',
            Self::NorthToEastBend => 'L',
            Self::NorthToWestBend => 'J',
            Self::SouthToWestBend => '7',
            Self::SouthToEastBend => 'F',
            Self::Ground => '.',
            Self::StartingPosition => 'S',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row_index: usize,
    col_index: usize,
}

#[derive(Debug)]
struct PipeNetwork {
    zone: Vec<Vec<Tile>>,
    animal: Option<Position>,
}

enum PipeNetworkSettings<'a> {
    Sized {
        width: usize,
        height: usize,
        fill: Tile,
    },
    FromStr(&'a str),
}

impl PipeNetwork {
    fn new(pipe_network_settings: PipeNetworkSettings) -> Result<Self, &str> {
        match pipe_network_settings {
            PipeNetworkSettings::Sized {
                width,
                height,
                fill,
            } => Ok(PipeNetwork {
                zone: vec![vec![fill; width]; height],
                animal: None,
            }),
            PipeNetworkSettings::FromStr(input) => {
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
                    Ok(PipeNetwork {
                        zone,
                        animal: Some(animal),
                    })
                } else {
                    Err("no animal")
                }
            }
        }
    }

    fn draw(&self) {
        self.zone.iter().for_each(|tiles| {
            tiles
                .iter()
                .for_each(|tile| print!("{}", tile.to_char().to_string()));
            println!();
        })
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
        let animal = self.animal.unwrap();
        let mut prev_position = animal;
        let mut next_position = self.follow_pipe(animal, animal);
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

    fn get_tile(&self, position: Position) -> Tile {
        self.zone
            .get(position.row_index)
            .unwrap()
            .get(position.col_index)
            .unwrap()
            .clone()
    }

    fn set_tile(&mut self, position: Position, tile: Tile) {
        self.zone[position.row_index][position.col_index] = tile;
    }

    fn copy_tile(&self, pipe_network: &mut PipeNetwork, position: Position) {
        pipe_network.set_tile(position, self.get_tile(position));
    }

    fn flood_fill(&mut self, position: Position) {
        self.set_tile(position, Tile::StartingPosition);
        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if let Ok(col_index) =
                    usize::try_from(isize::try_from(position.col_index).unwrap() + col_offset)
                {
                    if let Ok(row_index) =
                        usize::try_from(isize::try_from(position.row_index).unwrap() + row_offset)
                    {
                        if let Some(row) = self.zone.get(row_index) {
                            if let Some(tile) = row.get(col_index) {
                                if tile == &Tile::Ground {
                                    self.flood_fill(Position {
                                        row_index,
                                        col_index,
                                    })
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn count_tile(&self, tile: Tile) -> u32 {
        let mut count = 0;
        self.zone.iter().for_each(|row| {
            row.iter().for_each(|zone_tile| {
                if zone_tile == &tile {
                    count += 1
                }
            })
        });
        count
    }

    fn trace_edges(&self) -> Self {
        let mut pipe_network = Self::new(PipeNetworkSettings::Sized {
            width: self.zone.get(0).unwrap().len(),
            height: self.zone.len(),
            fill: Tile::Ground,
        })
        .unwrap();

        let animal = self.animal.unwrap();
        let mut prev_position = animal;
        let mut next_position = self.follow_pipe(animal, animal);
        while self
            .zone
            .get(next_position.row_index)
            .unwrap()
            .get(next_position.col_index)
            .unwrap()
            != &Tile::StartingPosition
        {
            let new_position = self.follow_pipe(next_position, prev_position);
            self.copy_tile(&mut pipe_network, new_position);
            prev_position = next_position;
            next_position = new_position;
        }

        pipe_network
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let pipe_network =
        PipeNetwork::new(PipeNetworkSettings::FromStr(input)).expect("Should be a valid network");
    let mut new_pipe_network = pipe_network.trace_edges();
    new_pipe_network.flood_fill(Position {
        row_index: 0,
        col_index: 0,
    });
    new_pipe_network.draw();
    let result = new_pipe_network.count_tile(Tile::Ground);
    println!("{result}");
}
