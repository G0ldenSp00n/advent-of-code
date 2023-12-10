use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, none_of, not_line_ending, u64},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
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

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (input, time) = preceded(tag("Time:"), preceded(multispace1, not_line_ending))(input)?;
    let (input, record_distance) =
        preceded(tag("\nDistance:"), preceded(multispace1, not_line_ending))(input)?;

    Ok((
        input,
        Race {
            time: time.to_string().replace(" ", "").parse().unwrap(),
            record_distance: record_distance
                .to_string()
                .replace(" ", "")
                .parse()
                .unwrap(),
        },
    ))
}

fn main() {
    let input = include_str!("./input2.txt");
    let (input, race) = parse_race(input).expect("should parse races");
    println!("{:#?}", race.compute_number_of_ways_to_win());
}
