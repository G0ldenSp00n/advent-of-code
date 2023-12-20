use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u64},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn convert_to_races(times: Vec<u64>, record_distances: Vec<u64>) -> Vec<Race> {
        times
            .iter()
            .zip(record_distances)
            .fold(vec![], |mut acc, (time, record_distance)| {
                acc.push(Race {
                    time: *time,
                    record_distance,
                });
                acc
            })
    }

    fn compute_distance_from_button_hold(&self, button_held_for: u64) -> u64 {
        let speed = button_held_for;
        let remaining_time = self.time - button_held_for;

        speed * remaining_time
    }

    fn compute_number_of_ways_to_win(&self) -> u64 {
        let mut ways_to_win = 0;
        for button_held_for in 0..self.time {
            if self.compute_distance_from_button_hold(button_held_for) > self.record_distance {
                ways_to_win += 1;
            }
        }
        ways_to_win
    }
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        tag("Time:"),
        preceded(multispace1, separated_list1(multispace1, u64)),
    )(input)?;
    let (input, distances) = preceded(
        tag("\nDistance:"),
        preceded(multispace1, separated_list1(multispace1, u64)),
    )(input)?;

    Ok((input, Race::convert_to_races(times, distances)))
}

fn main() {
    let input = include_str!("./input1.txt");
    let (input, races) = parse_races(input).expect("should parse races");
    let result: u64 = races
        .iter()
        .map(|race| race.compute_number_of_ways_to_win())
        .product();

    println!("{result}");
}
