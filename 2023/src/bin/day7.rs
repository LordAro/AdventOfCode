use std::convert::TryInto;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate itertools;
use itertools::iproduct;
use itertools::Itertools;

#[derive(Ord, Eq, PartialEq, PartialOrd)]
enum Rank {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card(u8);

impl From<char> for Card {
    fn from(c: char) -> Self {
        Card(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => char::to_digit(c, 10).unwrap() as u8,
        })
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_form = match self.0 {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            _ => char::from_digit(self.0 as u32, 10).unwrap(),
        };
        write!(f, "{}", char_form)
    }
}

struct Hand {
    orig: [Card; 5],
    sorted: [Card; 5],
}

impl Hand {
    fn is_five_of_kind(&self) -> bool {
        self.sorted[0] == self.sorted[1]
            && self.sorted[1] == self.sorted[2]
            && self.sorted[2] == self.sorted[3]
            && self.sorted[3] == self.sorted[4]
    }

    fn is_four_of_kind(&self) -> bool {
        let t1 = self.sorted[0] == self.sorted[1]
            && self.sorted[1] == self.sorted[2]
            && self.sorted[2] == self.sorted[3];
        let t2 = self.sorted[1] == self.sorted[2]
            && self.sorted[2] == self.sorted[3]
            && self.sorted[3] == self.sorted[4];
        t1 || t2
    }

    fn is_full_house(&self) -> bool {
        let t1 = self.sorted[0] == self.sorted[1]
            && self.sorted[1] == self.sorted[2]
            && self.sorted[3] == self.sorted[4]; // xxxyy
        let t2 = self.sorted[0] == self.sorted[1]
            && self.sorted[2] == self.sorted[3]
            && self.sorted[3] == self.sorted[4]; // yyxxx
        t1 || t2
    }

    fn is_three_of_kind(&self) -> bool {
        let t1 = self.sorted[0] == self.sorted[1] && self.sorted[1] == self.sorted[2]; // xxxab
        let t2 = self.sorted[1] == self.sorted[2] && self.sorted[2] == self.sorted[3]; // axxxb
        let t3 = self.sorted[2] == self.sorted[3] && self.sorted[3] == self.sorted[4]; // abxxx
        t1 || t2 || t3
    }

    fn is_two_pair(&self) -> bool {
        let t1 = self.sorted[0] == self.sorted[1] && self.sorted[2] == self.sorted[3]; // aabbx
        let t2 = self.sorted[0] == self.sorted[1] && self.sorted[3] == self.sorted[4]; // aaxbb
        let t3 = self.sorted[1] == self.sorted[2] && self.sorted[3] == self.sorted[4]; // xaabb
        t1 || t2 || t3
    }

    fn is_one_pair(&self) -> bool {
        let t1 = self.sorted[0] == self.sorted[1];
        let t2 = self.sorted[1] == self.sorted[2];
        let t3 = self.sorted[2] == self.sorted[3];
        let t4 = self.sorted[3] == self.sorted[4];
        t1 || t2 || t3 || t4
    }

    fn rank(&self) -> Rank {
        if self.is_five_of_kind() {
            Rank::FiveOfKind
        } else if self.is_four_of_kind() {
            Rank::FourOfKind
        } else if self.is_full_house() {
            Rank::FullHouse
        } else if self.is_three_of_kind() {
            Rank::ThreeOfKind
        } else if self.is_two_pair() {
            Rank::TwoPair
        } else if self.is_one_pair() {
            Rank::OnePair
        } else {
            Rank::HighCard
        }
    }

    fn joker_rank(&self) -> Rank {
        // Construct every possible card that could be formed from a joker, and get the rank from that
        let card_possibilities: Vec<_> = self
            .orig
            .iter()
            .map(|&c| {
                if c == Card(11) {
                    // Only way jokers can improve the rank is by taking the value of one of the
                    // existing cards
                    self.sorted.iter().dedup().copied().collect()
                } else {
                    vec![c]
                }
            })
            .collect();
        iproduct!(
            &card_possibilities[0],
            &card_possibilities[1],
            &card_possibilities[2],
            &card_possibilities[3],
            &card_possibilities[4]
        )
        .map(|(&a, &b, &c, &d, &e)| Hand::from(vec![a, b, c, d, e]))
        .map(|h| h.rank())
        .max()
        .unwrap()
    }
}

impl From<Vec<Card>> for Hand {
    fn from(v: Vec<Card>) -> Self {
        let mut s = v.clone();
        s.sort();
        Hand {
            orig: v.try_into().unwrap(),
            sorted: s.try_into().unwrap(),
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let v: Vec<_> = s.chars().map(Card::from).collect();
        Hand::from(v)
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.orig)
    }
}

fn standard_sort(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    a.rank().cmp(&b.rank()).then(a.orig.cmp(&b.orig))
}

fn joker_sort(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    a.joker_rank().cmp(&b.joker_rank()).then(
        // jokers now worth less than anything else
        a.orig
            .iter()
            .map(|x| if x.0 == 11 { 1 } else { x.0 })
            .cmp(b.orig.iter().map(|x| if x.0 == 11 { 1 } else { x.0 })),
    )
}

fn main() -> io::Result<()> {
    let input_data: Vec<String> = BufReader::new(
        File::open(
            env::args()
                .nth(1)
                .expect("Incorrect number of arguments provided"),
        )
        .expect("Could not open input file"),
    )
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let mut hand_bids: Vec<_> = input_data
        .iter()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            (
                Hand::from(it.next().unwrap()),
                it.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    hand_bids.sort_by(|a, b| standard_sort(&a.0, &b.0));

    let winnings: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(idx, &(_, bid))| (idx + 1) * bid as usize)
        .sum();

    println!("Total winnings: {}", winnings);

    hand_bids.sort_by(|a, b| joker_sort(&a.0, &b.0));

    let joker_winnings: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(idx, &(_, bid))| (idx + 1) * bid as usize)
        .sum();

    println!("Total winnings with jokers: {}", joker_winnings);
    Ok(())
}
