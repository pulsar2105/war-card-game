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
fn new_deck() -> Vec<Card> {
    let mut deck = vec![
        Card(CardColor::Nop, CardType::Joker),
        Card(CardColor::Nop, CardType::Joker),
    ];

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

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut r = rand::rng();
    deck.shuffle(&mut r);
}

fn main() {
    // initialise the deck
    let mut main_deck = new_deck();
    shuffle_deck(&mut main_deck);

    println!("{:?} {}", main_deck, main_deck.len());

    // create two players with empty hands of 52 cards
    let mut player1: VecDeque<Card> = VecDeque::new();
    let mut player2: VecDeque<Card> = VecDeque::new();
}
