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
    Number(u8),
    Joker,
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
            Self::Number(number) => match *other {
                Self::Ace | Self::King | Self::Queen => Ordering::Less,
                Self::Number(other_number) => {
                    if number == other_number {
                        Ordering::Equal
                    } else if number < other_number {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                Self::Joker => Ordering::Greater,
            },
            Self::Joker => match *other {
                Self::Joker => Ordering::Equal,
                _ => Ordering::Less,
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
            'T' => Self::Number(10),
            'J' => Self::Joker,
            c => Self::Number(c.to_string().parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind([Card; 5]),
    FourOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    ThreeOfAKind([Card; 5]),
    TwoPair([Card; 5]),
    OnePair([Card; 5]),
    HighCard([Card; 5]),
}

impl HandType {
    fn from(hand: &Hand) -> Self {
        let mut joker = 0;
        let mut numbers = [0; 14];
        hand.cards.iter().for_each(|card| match card {
            Card::Ace => numbers[13] += 1,
            Card::King => numbers[12] += 1,
            Card::Queen => numbers[11] += 1,
            Card::Number(n) => numbers[usize::try_from(n - 1).unwrap()] += 1,
            Card::Joker => joker += 1,
        });
        let mut one_pair_index = 0;
        let mut three_of_a_kind_index = 0;
        let mut result = HandType::HighCard(hand.cards);
        numbers.iter().enumerate().for_each(|(index, count)| {
            if *count == 5 || count + joker == 5 {
                result = HandType::FiveOfAKind(hand.cards);
            }

            if *count == 4 || count + joker == 4 {
                result = HandType::FourOfAKind(hand.cards);
            }
            if *count == 3 || count + joker == 3 {
                match result {
                    Self::OnePair(_) => {
                        if numbers[one_pair_index] + joker + count == 5 {
                            result = HandType::FullHouse(hand.cards)
                        } else {
                            result = HandType::ThreeOfAKind(hand.cards)
                        }
                    }
                    Self::HighCard(_) | Self::TwoPair(_) => {
                        three_of_a_kind_index = index;
                        result = HandType::ThreeOfAKind(hand.cards)
                    }
                    _ => (),
                }
            }
            if (*count == 2 || count + joker == 2) && three_of_a_kind_index != index {
                match result {
                    Self::ThreeOfAKind(_) => {
                        // println!(
                        //     "{:?} - {:?}, {joker} {count}",
                        //     numbers, three_of_a_kind_index
                        // );
                        if numbers[three_of_a_kind_index] + joker + count == 5 {
                            result = HandType::FullHouse(hand.cards)
                        }
                    }
                    Self::OnePair(_) => {
                        if numbers[one_pair_index] == 1 {
                            one_pair_index = index;
                        } else {
                            result = HandType::TwoPair(hand.cards)
                        }
                    }
                    Self::HighCard(_) => {
                        one_pair_index = index;
                        result = Self::OnePair(hand.cards)
                    }
                    _ => (),
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
            HandType::FiveOfAKind(cards) => match HandType::from(other) {
                HandType::FiveOfAKind(other_cards) => order_card_list(cards, other_cards),
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
    let input = include_str!("input2.txt");
    let (input, mut hands) = parse_hands(input).expect("Should parse hands");
    hands.sort_by(|a, b| {
        // println!(
        //     "{:?} < {:?} = {:?}",
        //     HandType::from(a),
        //     HandType::from(b),
        //     a.cmp(b)
        // );
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
