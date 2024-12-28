use nom::{
    character::complete::{i64, multispace1},
    multi::separated_list1,
    IResult,
};

fn parse_row(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, i64vecvec) = separated_list1(multispace1, i64)(input)?;
    Ok((input, i64vecvec))
}

fn main() {
    let input = include_str!("./input.txt");
    let res = input
        .split("\n")
        .map(|input| {
            if !input.is_empty() {
                let (_, data) = parse_row(input).expect("Row should correctly parse");

                let is_sorted = data.is_sorted() || data.iter().rev().is_sorted();

                let is_valid = data
                    .iter()
                    .zip(data.iter().skip(1))
                    .all(|(a, b)| (*a - *b).abs() <= 3 && (*a - *b).abs() >= 1);

                if is_valid && is_sorted {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<i64>();
    println!("Result: {}", res);
}
