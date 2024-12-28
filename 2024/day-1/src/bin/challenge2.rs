use nom::{
    character::complete::{i64, multispace1},
    sequence::separated_pair,
    IResult,
};

fn parse_row(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, pair) = separated_pair(i64, multispace1, i64)(input)?;
    Ok((input, pair))
}

fn main() {
    let data = include_str!("./input1.txt");
    let mut location_id_list_a = Vec::new();
    let mut location_id_list_b = Vec::new();
    data.split('\n').for_each(|row| {
        if !row.is_empty() {
            let parsed_location_id =
                parse_row(row).expect("Expect Row to be formatted Number Space Number");
            location_id_list_a.push(parsed_location_id.1 .0);
            location_id_list_b.push(parsed_location_id.1 .1);
        }
    });

    location_id_list_a.sort();
    location_id_list_b.sort();

    let mut pointer = 0;
    let result = location_id_list_a
        .iter()
        .map(|location_id_a| {
            let mut matches = 0;
            while pointer < location_id_list_b.len()
                && location_id_a >= location_id_list_b.get(pointer).unwrap()
            {
                if location_id_a == location_id_list_b.get(pointer).unwrap() {
                    matches += 1;
                }
                pointer += 1;
            }
            location_id_a * matches
        })
        .sum::<i64>();
    println!("Result: {}", result);
}
