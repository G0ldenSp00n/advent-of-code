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

                let mut neg = 0;
                let mut pos = 0;
                data.iter().zip(data.iter().skip(1)).for_each(|(a, b)| {
                    if a < b {
                        neg += 1;
                    } else {
                        pos += 1;
                    }
                });

                let mistakes = data
                    .iter()
                    .zip(data.iter().skip(1))
                    .map(|(a, b)| {
                        if neg > pos {
                            (*a - *b).abs() <= 3 && (*a - *b).abs() >= 1 && a < b
                        } else {
                            (*a - *b).abs() <= 3 && (*a - *b).abs() >= 1 && a > b
                        }
                    })
                    .filter(|is_valid| !*is_valid)
                    .count();

                if mistakes <= 2 {
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
