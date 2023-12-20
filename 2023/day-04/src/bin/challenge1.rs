use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0, multispace1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct LotteryCard {
    id: u32,
    winning_numbers: Vec<u32>,
    found_numbers: Vec<u32>,
}

impl LotteryCard {
    fn score(&self) -> u32 {
        self.found_numbers.iter().fold(0, |mut acc, number| {
            if self.winning_numbers.contains(number) {
                if acc == 0 {
                    acc = 1
                } else {
                    acc *= 2;
                }
            }
            acc
        })
    }
}

fn card(input: &str) -> IResult<&str, LotteryCard> {
    let (input, id) = preceded(tag("Card"), preceded(multispace0, complete::u32))(input)?;
    let (input, winning_numbers) = preceded(
        tag(":"),
        preceded(multispace0, separated_list1(multispace0, complete::u32)),
    )(input)?;
    let (input, found_numbers) = preceded(
        tag(" |"),
        preceded(multispace0, separated_list1(multispace0, complete::u32)),
    )(input)?;
    Ok((
        input,
        LotteryCard {
            id,
            winning_numbers,
            found_numbers,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<LotteryCard>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;
    Ok((input, cards))
}

fn main() {
    let input = include_str!("./input1.txt");
    let (_input, cards) = parse_cards(input).expect("should parse");
    let result: u32 = cards
        .iter()
        .map(|card| {
            let score = card.score();
            println!("Card {} - Score {}", card.id, score);
            score
        })
        .sum();
    println!("{}", result);
}
