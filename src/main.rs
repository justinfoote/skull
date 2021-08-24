#[derive(Copy, Clone, Debug)]
enum Card {
    Skull,
    Flower,
}

#[derive(Debug)]
struct Player {
    id: String,
    hand: [Card; 4],
}

struct Game {
    players: Vec<Player>
}


fn new_player(id: String) -> Player {
    Player{id, hand: [Card::Skull, Card::Flower, Card::Flower, Card::Flower]}
}


fn main() {
    let id: String = "abc".to_string();
    let player = new_player(id);
    println!("Created player {:?}", player)
}