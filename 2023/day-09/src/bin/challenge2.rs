use nom::{
    character::complete::{i64, multispace1},
    multi::separated_list1,
    IResult,
};

fn find_diff(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_zero = true;
    let res: Vec<i64> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| {
            println!("a {a}, b {b}");
            let res = b - a;
            if res != 0 {
                all_zero = false;
            }
            res
        })
        .collect();

    let mut output: Vec<Vec<i64>> = vec![input.clone()];
    if !all_zero {
        let mut diff = find_diff(res);
        output.append(&mut diff);
    }
    output
}

fn calculate_next_seq(input: Vec<i64>) -> i64 {
    let diff = find_diff(input);
    diff.iter().rev().fold(0, |acc, curr| {
        let first = curr.first().unwrap();
        first - acc
    })
}

fn parse_vec(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, u64vecvec) = separated_list1(multispace1, i64)(input)?;
    Ok((input, u64vecvec))
}

fn main() {
    let input = include_str!("./input2.txt");
    let res: i64 = input
        .split("\n")
        .filter(|str| !str.is_empty())
        .map(|input| {
            let (_, data) = parse_vec(input).expect("Should parse correctly");
            println!("{data:?}");
            let o = calculate_next_seq(data);
            println!("{o:?}");
            o
        })
        .sum();
    println!("{res}");
}
