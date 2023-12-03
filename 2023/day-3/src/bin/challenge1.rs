struct EngineScheme {
    scheme: Vec<Vec<char>>,
}

#[derive(Debug)]
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
        println!("{}", read_val);
        read_val.parse().unwrap()
    }

    fn test_region(&self, region: &Region) -> bool {
        let mut found_symbol = false;
        println!("Testing Region {} - {:?}", self.read_region(region), region);
        for row_index in region.lower_row()..=region.upper_row(self.scheme.len()) {
            for char_index in
                region.lower_char_index()..=region.upper_char_index(self.scheme[0].len())
            {
                println!(
                    "Testing Char [{}, {}]: {}",
                    row_index, char_index, self.scheme[row_index][char_index]
                );
                let test_char = self.scheme[row_index][char_index];
                if !test_char.is_digit(10) && test_char != '.' {
                    found_symbol = true;
                }
            }
        }
        println!(
            "Testing Region {} - {}",
            self.read_region(region),
            if found_symbol { "Succeeded" } else { "Failed" }
        );
        found_symbol
    }
}

fn main() {
    let data = include_str!("./input1.txt");
    let scheme = EngineScheme::fill_scheme(data);
    let found_regions: u32 = scheme
        .find_regions()
        .iter()
        .filter(|region| scheme.test_region(region))
        .map(|region| scheme.read_region(region))
        .sum();
    println!("{:?}", found_regions)
}
