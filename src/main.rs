#[derive(Copy, Clone, Debug)]
enum Card {
    Skull,
    Flower,
}

#[derive(Debug)]
struct Player {
    id: String,
    hand: [Card; 4],
    score: u32,
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
}
impl Game {
    fn new() -> Self {
        Game{
            players: Vec::new(),
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

fn new_player(id: String) -> Player {
    Player {
        id,
        hand: [Card::Skull, Card::Flower, Card::Flower, Card::Flower],
        score: 0,
    }
}

fn main() {
    let mut game = Game::new();
    for i in 0..6 {
        game.add_player(new_player(i.to_string()))
    }
    println!("Created game: {:?}", game);
}
