use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["♥","♠","♦"];
        let numbers = ["K", "Q", "J"];

        let mut cards = vec![];
        
        for suit in suits {
            for number in numbers {
                let card = format!("{} {}", suit, number);
                cards.push(card);
            }
        }

        Deck{cards}
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Deck: {:#?}", deck);
}
