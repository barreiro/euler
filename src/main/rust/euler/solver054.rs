// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::read_to_string;
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

use euler::Solver;

// In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:
//
//       High Card: Highest value card.
//        One Pair: Two cards of the same value.
//       Two Pairs: Two different pairs.
// Three of a Kind: Three cards of the same value.
//        Straight: All cards are consecutive values.
//           Flush: All cards of the same suit.
//      Full House: Three of a kind and a pair.
//  Four of a Kind: Four cards of the same value.
//  Straight Flush: All cards are consecutive values of same suit.
//     Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
//
// The cards are valued in the order: 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//
// If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens, then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.
//
// Consider the following five hands dealt to two players:
//
// Hand Player 1           Player 2            Winner
//
// 1    5H 5C 6S 7S KD     2C 3S 8S 8D TD      Player 2
//      Pair of Fives      Pair of Eights
//
// 2    5D 8C 9S JS AC     2C 5C 7D 8S QH      Player 1
//      Highest card Ace   Highest card Queen
//
// 3    2D 9C AS AH AC     3D 6D 7D TD QD      Player 2
//      Three Aces         Flush with Diamonds
//
// 4    4D 6S 9H QH QC     3D 6D 7H QD QS      Player 1
//      Pair of Queens     Pair of Queens
//      Highest card Nine  Highest card Seven
//
// 5    2H 2D 4C 4D 4S     3C 3D 3S 9S 9D      Player 1
//      Full House         Full House
//      With Three Fours   With Three Threes
//
// The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.
//
// How many hands does Player 1 win?

const INPUT_FILE: &str = "src/main/resources/net/projecteuler/barreiro/problem/problem054-data.txt";

pub struct Solver054 {
    pub n: isize,
    pub input: String,
}

impl Default for Solver054 {
    fn default() -> Self {
        Solver054 { n: 1000, input: read_to_string(Path::new(INPUT_FILE)).expect("Unable to read file") }
    }
}

impl Solver for Solver054 {
    fn solve(&self) -> isize {
        self.input.lines().take(self.n as _).map(|s| {
            let cards = s.split_whitespace().map(|c| c.parse().unwrap()).collect::<Vec<_>>();
            (Hand::from(&cards[0..cards.len() >> 1]), Hand::from(&cards[cards.len() >> 1..cards.len()]))
        }).filter(|(h1, h2)| h1 > h2).count() as _
    }
}

// --- //

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl FromStr for Rank {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err("Unknown Rank"),
        }
    }
}

impl Sub for Rank {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        self as isize - rhs as isize
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl FromStr for Suite {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Suite::Clubs),
            "D" => Ok(Suite::Diamonds),
            "H" => Ok(Suite::Hearts),
            "S" => Ok(Suite::Spades),
            _ => Err("Unknown Suite"),
        }
    }
}

#[derive(Copy, Clone)]
struct Card {
    rank: Rank,
    suite: Suite,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rank, suite) = s.split_at(1);
        Ok(Card { rank: rank.parse().unwrap(), suite: suite.parse().unwrap() })
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank.eq(&other.rank)
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    fn is_four_of_a_kind(&self) -> bool {
        for i in 1..4 {
            if self.cards[i] != self.cards[i - 1] {
                return false;
            }
        }
        self.cards[0] == self.cards[1] || self.cards[3] == self.cards[4]
    }

    fn is_full_house(&self) -> bool {
        self.cards[0] == self.cards[1] && self.cards[3] == self.cards[4] && (self.cards[1] == self.cards[2] || self.cards[2] == self.cards[3])
    }

    fn is_flush(&self) -> bool {
        self.cards.iter().all(|c| c.suite == self.cards[0].suite)
    }

    fn is_straight(&self) -> bool {
        if self.cards[4].rank - self.cards[0].rank != 4 {
            return false;
        }
        for i in 1..5 {
            if self.cards[i].rank - self.cards[i - 1].rank != 1 {
                return false;
            }
        }
        true
    }

    fn is_three_of_a_kind(&self) -> Option<Rank> {
        for i in 2..5 {
            if self.cards[i] == self.cards[i - 1] && self.cards[i] == self.cards[i - 2] {
                return Some(self.cards[i].rank);
            }
        }
        None
    }

    fn is_two_pairs(&self) -> bool {
        let (low_pair, high_pair) = (self.cards[0] == self.cards[1], self.cards[3] == self.cards[4]);
        low_pair && (self.cards[2] == self.cards[3] || high_pair) || high_pair && self.cards[1] == self.cards[2]
    }

    fn is_pair(&self) -> Option<Rank> {
        for i in (2..5).rev() {
            if self.cards[i] == self.cards[i - 1] && self.cards[i] != self.cards[i - 2] {
                return Some(self.cards[i].rank);
            }
        }
        if self.cards[0] == self.cards[1] { Some(self.cards[0].rank) } else { None }
    }
}

impl From<&[Card]> for Hand {
    fn from(c: &[Card]) -> Self {
        let mut cards = [c[0], c[1], c[2], c[3], c[4]];
        cards.sort_unstable();
        Hand { cards }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Equal)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let highest = || self.cards[4].cmp(&other.cards[4]).then(self.cards[3].cmp(&other.cards[3])).then(self.cards[2].cmp(&other.cards[2])).then(self.cards[1].cmp(&other.cards[1])).then(self.cards[0].cmp(&other.cards[0]));
        let (trio, pair) = (self.is_three_of_a_kind(), self.is_pair());

        if self.is_straight_flush() {
            if other.is_straight_flush() { highest() } else { Greater }
        } else if self.is_four_of_a_kind() {
            if other.is_straight_flush() { Less } else if other.is_four_of_a_kind() { highest() } else { Greater }
        } else if self.is_full_house() {
            if other.is_straight_flush() || other.is_four_of_a_kind() { Less } else if other.is_full_house() { trio.cmp(&other.is_three_of_a_kind()).then(pair.cmp(&other.is_pair())) } else { Greater }
        } else if self.is_flush() {
            if other.is_straight_flush() || other.is_four_of_a_kind() || other.is_full_house() { Less } else if other.is_flush() { highest() } else { Greater }
        } else if self.is_straight() {
            if other.is_four_of_a_kind() || other.is_full_house() || other.is_flush() { Less } else if other.is_straight() { highest() } else { Greater }
        } else if trio.is_some() {
            if other.is_four_of_a_kind() || other.is_full_house() || other.is_flush() || other.is_straight() { Less } else { other.is_three_of_a_kind().map_or(Greater, |t| trio.unwrap().cmp(&t)).then(highest()) }
        } else if self.is_two_pairs() {
            if other.is_four_of_a_kind() || other.is_full_house() || other.is_flush() || other.is_straight() || other.is_three_of_a_kind().is_some() { Less } else if other.is_two_pairs() { other.is_pair().map_or(Greater, |t| pair.unwrap().cmp(&t)).then(highest()) } else { Greater }
        } else if pair.is_some() {
            if other.is_four_of_a_kind() || other.is_full_house() || other.is_flush() || other.is_straight() || other.is_three_of_a_kind().is_some() || other.is_two_pairs() { Less } else { other.is_pair().map_or(Greater, |t| pair.unwrap().cmp(&t)).then(highest()) }
        } else if other.is_four_of_a_kind() || other.is_full_house() || other.is_flush() || other.is_straight() || other.is_three_of_a_kind().is_some() || other.is_two_pairs() || other.is_pair().is_some() { Less } else { highest() }
    }
}
