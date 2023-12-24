use glam::IVec2;

#[derive(Debug)]
enum MapPoint {
    Galaxy,
    EmptySpace,
}

#[derive(Debug)]
struct StarMap {
    map: IVec2,
}

impl StarMap {
    fn new(input: IVec2) -> Self {}
}

fn parse_star_map(input: &str) -> StarMap {
    let map = input
        .split('\n')
        .map(|rows| {
            rows.chars()
                .map(|char| match char {
                    '#' => MapPoint::Galaxy,
                    '.' => MapPoint::EmptySpace,
                    _ => panic!("Should only include valid chars"),
                })
                .collect()
        })
        .collect();
    StarMap::new(map)
}

fn main() {
    let input = include_str!("input1.txt");
    let star_map = parse_star_map(input);
    println!("{star_map:?}");
}
