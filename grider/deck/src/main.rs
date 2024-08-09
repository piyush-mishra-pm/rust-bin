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

    fn shuffle(&self) {
        
    }
}

fn main() {
    let deck = Deck::new();
    deck.shuffle();
    println!("Deck: {:#?}", deck);
}
