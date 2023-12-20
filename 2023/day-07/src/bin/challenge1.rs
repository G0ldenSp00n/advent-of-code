use std::cmp::Ordering;

use nom::{
    character::complete::{anychar, multispace1, newline, u64},
    multi::{many_m_n, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Self::Ace => match *other {
                Self::Ace => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::King => match *other {
                Self::Ace => Ordering::Less,
                Self::King => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Queen => match *other {
                Self::Ace | Self::King => Ordering::Less,
                Self::Queen => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Jack => match *other {
                Self::Ace | Self::King | Self::Queen => Ordering::Less,
                Self::Jack => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Number(number) => match *other {
                Self::Ace | Self::King | Self::Queen | Self::Jack => Ordering::Less,
                Self::Number(other_number) => {
                    if number == other_number {
                        Ordering::Equal
                    } else if number < other_number {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            },
        }
    }
}

impl Card {
    fn from(c: &char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            c => Self::Number(c.to_string().parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(Card),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card; 5]),
}

impl HandType {
    fn from(hand: &Hand) -> Self {
        let mut numbers = [0; 14];
        hand.cards.iter().for_each(|card| match card {
            Card::Ace => numbers[13] += 1,
            Card::King => numbers[12] += 1,
            Card::Queen => numbers[11] += 1,
            Card::Jack => numbers[10] += 1,
            Card::Number(n) => numbers[usize::try_from(n - 1).unwrap()] += 1,
        });
        let mut result = HandType::HighCard(hand.cards);
        numbers.iter().for_each(|count| {
            if *count == 5 {
                result = HandType::FiveOfAKind(hand.cards[0]);
            }

            if *count == 4 {
                result = HandType::FourOfAKind(hand.cards);
            }
            if *count == 3 {
                match result {
                    Self::OnePair(_) => result = HandType::FullHouse(hand.cards),
                    _ => result = HandType::ThreeOfAKind(hand.cards),
                }
            }
            if *count == 2 {
                match result {
                    Self::ThreeOfAKind(_) => result = HandType::FullHouse(hand.cards),
                    Self::OnePair(_) => result = HandType::TwoPair(hand.cards),
                    _ => result = Self::OnePair(hand.cards),
                }
            }
        });
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

fn order_card_list(cards: [Card; 5], other_cards: [Card; 5]) -> Ordering {
    let mut result = Ordering::Equal;
    cards
        .iter()
        .zip(other_cards.iter())
        .for_each(|(card, other_card)| {
            // println!("{:?}, {:?} - {:?}", card, other_card, card.cmp(other_card));
            if card.cmp(other_card) == Ordering::Less && result == Ordering::Equal {
                result = Ordering::Less;
            } else if card.cmp(other_card) == Ordering::Greater && result == Ordering::Equal {
                result = Ordering::Greater;
            }
        });
    result
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match HandType::from(self) {
            HandType::FiveOfAKind(card) => match HandType::from(other) {
                HandType::FiveOfAKind(other_card) => card.cmp(&other_card),
                _ => Ordering::Greater,
            },
            HandType::FourOfAKind(cards) => match HandType::from(other) {
                HandType::FiveOfAKind(_) => Ordering::Less,
                HandType::FourOfAKind(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Greater,
            },
            HandType::FullHouse(cards) => match HandType::from(other) {
                HandType::FiveOfAKind(_) => Ordering::Less,
                HandType::FourOfAKind(_) => Ordering::Less,
                HandType::FullHouse(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Greater,
            },
            HandType::ThreeOfAKind(cards) => match HandType::from(other) {
                HandType::FiveOfAKind(_) => Ordering::Less,
                HandType::FourOfAKind(_) => Ordering::Less,
                HandType::FullHouse(_) => Ordering::Less,
                HandType::ThreeOfAKind(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Greater,
            },
            HandType::TwoPair(cards) => match HandType::from(other) {
                HandType::OnePair(_) => Ordering::Greater,
                HandType::HighCard(_) => Ordering::Greater,
                HandType::TwoPair(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Less,
            },
            HandType::OnePair(cards) => match HandType::from(other) {
                HandType::HighCard(_) => Ordering::Greater,
                HandType::OnePair(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Less,
            },
            HandType::HighCard(cards) => match HandType::from(other) {
                HandType::HighCard(other_cards) => order_card_list(cards, other_cards),
                _ => Ordering::Less,
            },
        }
    }
}

fn parse_cards(input: &str) -> IResult<&str, [Card; 5]> {
    let (input, cards) = many_m_n(5, 5, anychar)(input)?;
    let cards: Vec<Card> = cards.iter().map(|c| Card::from(c)).take(5).collect();
    Ok((input, cards.try_into().expect("Should become array")))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand) = separated_pair(parse_cards, multispace1, u64)(input)?;
    Ok((input, {
        Hand {
            cards: hand.0,
            bid: hand.1,
        }
    }))
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(newline, parse_hand)(input)?;
    Ok((input, hands))
}

fn main() {
    let input = include_str!("input1.txt");
    let (input, mut hands) = parse_hands(input).expect("Should parse hands");
    hands.sort_by(|a, b| {
        println!(
            "{:?} < {:?} = {:?}",
            HandType::from(a),
            HandType::from(b),
            a.cmp(b)
        );
        a.cmp(b)
    });
    let result: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            println!("{:?}", HandType::from(hand));
            let res = hand.bid * u64::try_from(index + 1).unwrap();
            res
        })
        .sum();
    println!("{:#?}", result);
}
