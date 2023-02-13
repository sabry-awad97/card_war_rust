use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Eq, PartialEq)]
struct Card {
    rank: u8,
    suit: u8,
}

impl Card {
    fn new(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank == other.rank {
            self.suit.cmp(&other.suit)
        } else {
            self.rank.cmp(&other.rank)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::new();
        for i in 2..15 {
            for j in 0..4 {
                cards.push(Card::new(i, j));
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Deck { cards }
    }

    fn remove_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Player {
    name: String,
    wins: i32,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            wins: 0,
        }
    }
}

struct Game {
    deck: Deck,
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new() -> Game {
        let deck = Deck::new();
        let player1 = Player::new("A");
        let player2 = Player::new("B");
        Game {
            deck,
            player1,
            player2,
        }
    }

    pub fn play_game(&mut self) {
        println!("Beginning War");
        let mut response = String::new();
        while self.deck.cards.len() >= 2 && response != "q" {
            response.clear();
            std::io::stdin().read_line(&mut response).unwrap();
            response = response.trim().to_string();
            let player1_card = self.deck.remove_card().unwrap();
            let player2_card = self.deck.remove_card().unwrap();
            println!(
                "{} drew {:?}, {} drew {:?}",
                self.player1.name, player1_card, self.player2.name, player2_card
            );
            if player1_card.rank > player2_card.rank {
                self.player1.wins += 1;
                println!("{} wins this round", self.player1.name);
            } else {
                self.player2.wins += 1;
                println!("{} wins this round", self.player2.name);
            }
        }
    }

    pub fn winner(&self) -> String {
        if self.player1.wins > self.player2.wins {
            format!("{} wins with {} wins", self.player1.name, self.player1.wins)
        } else if self.player1.wins < self.player2.wins {
            format!("{} wins with {} wins", self.player2.name, self.player2.wins)
        } else {
            format!("It was a tie! Both players had {} wins", self.player1.wins)
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play_game();

    println!("{}", game.winner());
}
