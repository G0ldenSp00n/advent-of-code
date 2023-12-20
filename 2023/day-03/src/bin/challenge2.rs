struct EngineScheme {
    scheme: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Region {
    row: usize,
    lower: usize,
    upper: usize,
}

impl Region {
    fn lower_row(&self) -> usize {
        if self.row == 0 {
            0
        } else {
            self.row - 1
        }
    }

    fn upper_row(&self, max_row: usize) -> usize {
        if self.row + 2 < max_row {
            self.row + 1
        } else {
            self.row
        }
    }

    fn lower_char_index(&self) -> usize {
        if self.lower == 0 {
            0
        } else {
            self.lower - 1
        }
    }

    fn upper_char_index(&self, max_char_index: usize) -> usize {
        if self.upper + 1 < max_char_index {
            self.upper + 1
        } else {
            self.upper
        }
    }
}

impl EngineScheme {
    fn fill_scheme(data: &str) -> Self {
        let rows = data.split('\n');
        Self {
            scheme: rows.map(|row| row.chars().collect()).collect(),
        }
    }

    fn find_regions(&self) -> Vec<Region> {
        let mut found_regions: Vec<Region> = vec![];
        for (row_index, row) in self.scheme.iter().enumerate() {
            let mut regions = row.iter().enumerate().filter(|(_, char)| char.is_digit(10));
            let first_digit = regions.next();
            if let Some(first_digit) = first_digit {
                found_regions.push(Region {
                    row: row_index,
                    lower: first_digit.0,
                    upper: first_digit.0,
                });
                regions.for_each(|digit| {
                    let current_region = found_regions.last_mut().unwrap();
                    if current_region.upper + 1 == digit.0 {
                        current_region.upper = digit.0;
                    } else {
                        found_regions.push(Region {
                            row: row_index,
                            lower: digit.0,
                            upper: digit.0,
                        });
                    }
                })
            }
        }
        found_regions
    }

    fn read_region(&self, region: &Region) -> u32 {
        let mut read_val: String = "".to_owned();
        for char_index in region.lower..=region.upper {
            read_val += &self.scheme[region.row][char_index].to_string()[..];
        }
        read_val.parse().unwrap()
    }

    fn read_region_string(&self, region: &Region) -> String {
        let mut read_val: String = "".to_owned();
        for char_index in region.lower_char_index()..=region.upper_char_index(self.scheme[0].len())
        {
            read_val += &self.scheme[region.row][char_index].to_string()[..];
        }
        read_val
    }

    fn find_gear_piece(&self, region: &Region) -> Option<Region> {
        let mut found_symbol = false;
        println!("Testing Region {} - {:?}", self.read_region(region), region);
        for row_index in region.lower_row()..=region.upper_row(self.scheme.len()) {
            println!(
                "{}",
                self.read_region_string(&Region {
                    row: row_index,
                    lower: region.lower_char_index(),
                    upper: region.upper_char_index(self.scheme[0].len())
                })
            );
            for char_index in
                region.lower_char_index()..=region.upper_char_index(self.scheme[0].len())
            {
                let test_char = self.scheme[row_index][char_index];
                if test_char == '*' {
                    return Some(Region {
                        row: row_index,
                        lower: char_index,
                        upper: char_index,
                    });
                }
            }
        }
        None
    }

    fn find_complete_number_region(&self, starting_digit: &Region) -> Option<Region> {
        if self.scheme[starting_digit.row][starting_digit.lower].is_numeric() {
            let lower = self.scheme[starting_digit.row]
                .iter()
                .enumerate()
                .rev()
                .skip_while(|(index, _char)| *index > starting_digit.lower)
                .take_while(|(_index, char)| char.is_numeric())
                .last()
                .unwrap()
                .0;
            let upper = self.scheme[starting_digit.row]
                .iter()
                .enumerate()
                .skip_while(|(index, _char)| *index < starting_digit.upper)
                .take_while(|(_index, char)| char.is_numeric())
                .last()
                .unwrap()
                .0;
            return Some(Region {
                row: starting_digit.row,
                lower,
                upper,
            });
        }
        None
    }

    fn find_gear_piece_pairs(&self, gear_piece_region: &Region) -> Option<(Region, Region)> {
        let mut first_region: Option<Region> = None;
        for row_index in
            gear_piece_region.lower_row()..=gear_piece_region.upper_row(self.scheme.len())
        {
            for char_index in gear_piece_region.lower_char_index()
                ..=gear_piece_region.upper_char_index(self.scheme[0].len())
            {
                if self.scheme[row_index][char_index].is_numeric() {
                    if let Some(number_region) = self.find_complete_number_region(&Region {
                        row: row_index,
                        lower: char_index,
                        upper: char_index,
                    }) {
                        if let Some(first_region) = first_region {
                            if first_region != number_region {
                                return Some((first_region, number_region));
                            }
                        } else {
                            first_region = Some(number_region);
                        }
                    }
                }
            }
        }
        None
    }

    fn calculate_great_ratio(&self, region: &Region) -> u32 {
        if let Some(gear_piece) = self.find_gear_piece(region) {
            if let Some((number_region_one, number_region_two)) =
                self.find_gear_piece_pairs(&gear_piece)
            {
                if number_region_one == *region {
                    return self.read_region(&number_region_one)
                        * self.read_region(&number_region_two);
                }
            }
        }
        0
    }
}

fn main() {
    let data = include_str!("./input2.txt");
    let scheme = EngineScheme::fill_scheme(data);
    let found_regions: u32 = scheme
        .find_regions()
        .iter()
        .map(|region| scheme.calculate_great_ratio(region))
        .sum();
    println!("{:?}", found_regions)
}
