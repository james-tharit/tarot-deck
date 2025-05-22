use rand::{rng, seq::SliceRandom};

const MAJOR_ARCANA: [&str; 22] = [
    "The Fool",
    "The Magician",
    "The High Priestess",
    "The Empress",
    "The Emperor",
    "The Hierophant",
    "The Lovers",
    "The Chariot",
    "Strength",
    "The Hermit",
    "Wheel of Fortune",
    "Justice",
    "The Hanged Man",
    "Death",
    "Temperance",
    "The Devil",
    "The Tower",
    "The Star",
    "The Moon",
    "The Sun",
    "Judgement",
    "The World",
];

const MINOR_ARCANA: [&str; 14] = [
    "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Page",
    "Knight", "Queen", "King",
];

const SUITS: [&str; 4] = ["Cups", "Pentacles", "Swords", "Wands"];

#[derive(Debug)]
struct Deck {
    major: Vec<String>,
    minor: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut minor: Vec<String> = vec![];

        for s in SUITS {
            for m in MINOR_ARCANA {
                let card = format!("{} of {}", m, s);
                minor.push(card);
            }
        }

        Deck {
            major: MAJOR_ARCANA.map(str::to_string).to_vec(),
            minor,
        }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.major.shuffle(&mut rng);
        self.minor.shuffle(&mut rng);
    }

    fn deal_major(&mut self, num: usize) -> Vec<String> {
        self.major.drain(0..num).collect()
    }
}
fn main() {
    let mut deck = Deck::new();
    // println!("This is my deck {:#?}", deck);
    deck.shuffle();
    // println!("Shuffled {:#?}", deck);
    println!("Deal 2: {:#?}", deck.deal_major(2))
}
