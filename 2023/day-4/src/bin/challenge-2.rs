use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone)]
struct LotteryCard {
    id: u32,
    winning_numbers: Vec<u32>,
    found_numbers: Vec<u32>,
}

#[derive(Debug)]
struct LotteryCardStack {
    _lottery_card: LotteryCard,
    copies: u32,
}

impl LotteryCard {
    fn score(&self) -> u32 {
        self.found_numbers.iter().fold(0, |mut acc, number| {
            if self.winning_numbers.contains(number) {
                acc += 1;
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
    let input = include_str!("./input2.txt");
    let (_input, cards) = parse_cards(input).expect("should parse");
    let mut cards_copies = vec![1];
    let result: u32 = cards
        .iter()
        .map(|card| {
            let score = card.score();
            (score, card)
        })
        .map(|(score, card)| {
            let copies = cards_copies.pop().unwrap_or_else(|| 1);
            for _ in 0..copies {
                let mut curr_score = score;
                cards_copies.iter_mut().rev().for_each(|copy_amount| {
                    if curr_score > 0 {
                        *copy_amount += 1;
                        curr_score -= 1;
                    }
                });
                if curr_score > 0 {
                    let mut current = cards_copies.clone();
                    cards_copies = vec![2; usize::try_from(curr_score).ok().unwrap()];
                    cards_copies.append(&mut current);
                }
            }
            LotteryCardStack {
                lottery_card: card.clone(),
                copies,
            }
        })
        .map(|card_stacks| card_stacks.copies)
        .sum();
    println!("{}", result);
}
