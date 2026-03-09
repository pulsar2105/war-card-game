use rand::prelude::SliceRandom;
use std::collections::VecDeque;
use std::fmt::Debug;

// Card definitions

// different Faces
#[derive(Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
enum CardFaces {
    Jack,
    Queen,
    King,
}

// different value of cards
#[derive(Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
enum CardType {
    Value(usize),
    Faces(CardFaces),
    As,
    Joker,
}

// different color of cards
#[derive(Clone, Copy, PartialEq, Eq)]
enum CardColor {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
    Nop,
}

#[derive(Clone, Copy)]
struct Card(CardColor, CardType);

// Order Traits

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

// Debug Trait
impl Debug for CardFaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardFaces::King => write!(f, "K"),
            CardFaces::Queen => write!(f, "Q"),
            CardFaces::Jack => write!(f, "J"),
        }
    }
}
impl Debug for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardType::Value(v) => write!(f, "{}", v),
            CardType::As => write!(f, "As"),
            CardType::Faces(face) => write!(f, "{:?}", face),
            CardType::Joker => write!(f, "Joker"),
        }
    }
}

impl Debug for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardColor::Spades => write!(f, "♠️"),
            CardColor::Clubs => write!(f, "♣️"),
            CardColor::Hearts => write!(f, "♥️"),
            CardColor::Diamonds => write!(f, "♦️"),
            CardColor::Nop => write!(f, ""),
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            CardType::Joker => write!(f, "{:?}", self.1),
            _ => write!(f, "{:?}-{:?}", self.1, self.0),
        }
    }
}

// initialise the main deck of cards
fn new_deck(joker: bool) -> Vec<Card> {
    let mut deck = vec![];

    // Jokers
    if joker {
        deck.push(Card(CardColor::Nop, CardType::Joker));
        deck.push(Card(CardColor::Nop, CardType::Joker));
    }

    for color in [
        CardColor::Spades,
        CardColor::Clubs,
        CardColor::Hearts,
        CardColor::Diamonds,
    ] {
        // Values
        for i in 2..11 {
            deck.push(Card(color, CardType::Value(i)))
        }

        // Faces
        deck.push(Card(color, CardType::Faces(CardFaces::King)));
        deck.push(Card(color, CardType::Faces(CardFaces::Queen)));
        deck.push(Card(color, CardType::Faces(CardFaces::Jack)));

        // As
        deck.push(Card(color, CardType::As));
    }

    deck
}

fn shuffle_deck(deck: &mut [Card]) {
    let mut r = rand::rng();
    deck.shuffle(&mut r);
}

#[derive(Debug, Clone)]
struct Game {
    player1: VecDeque<Card>,
    player2: VecDeque<Card>,
}

impl Game {
    fn new() -> Game {
        let mut deck = new_deck(true);
        shuffle_deck(&mut deck);

        // create two players with empty hands of 52 cards
        let mut player1: VecDeque<Card> = VecDeque::with_capacity(deck.len());
        let mut player2: VecDeque<Card> = VecDeque::with_capacity(deck.len());

        // repartion
        for _ in 0..(deck.len() / 2) {
            player1.push_back(deck.pop().unwrap());
            player2.push_back(deck.pop().unwrap());
        }

        Game { player1, player2 }
    }
}

impl Iterator for Game {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // if one of the player can not continue
        if self.player1.is_empty() || self.player2.is_empty() {
            return None;
        }

        // so we can unwrap safely
        let card_a = self.player1.pop_front().unwrap();
        let card_b = self.player2.pop_front().unwrap();

        // when a player win he get his card THEN the other's player card
        if card_a < card_b {
            self.player2.push_back(card_b);
            self.player2.push_back(card_a);
        } else if card_a > card_b {
            self.player1.push_back(card_a);
            self.player1.push_back(card_b);
        } else {
            // case of a battle
            let mut tmp_player1: VecDeque<Card> = VecDeque::new();
            let mut tmp_player2: VecDeque<Card> = VecDeque::new();

            tmp_player1.push_back(card_a);
            tmp_player2.push_back(card_b);

            loop {
                if self.player1.len() < 2 || self.player2.len() < 2 {
                    return None;
                }

                // each player put one card face down
                tmp_player1.push_back(self.player1.pop_front().unwrap());
                tmp_player2.push_back(self.player2.pop_front().unwrap());

                // each player put one card face up
                let card_a = self.player1.pop_front().unwrap();
                let card_b = self.player2.pop_front().unwrap();

                tmp_player1.push_back(card_a);
                tmp_player2.push_back(card_b);

                if card_a < card_b {
                    self.player2.extend(tmp_player2);
                    self.player2.extend(tmp_player1);
                    break;
                } else if card_a > card_b {
                    self.player1.extend(tmp_player1);
                    self.player1.extend(tmp_player2);
                    break;
                }
            }
        }

        Some((self.player1.len(), self.player2.len()))
    }
}

fn main() {
    // initialise the deck
    let game = Game::new();

    println!("Init :\n{:?}", game);

    let mut n = 0;
    game.for_each(|(p1, p2)| {
        println!("p1 {}, p2 {}", p1, p2);
        n += 1
    });

    println!("Length of the game = {}", n);
}
