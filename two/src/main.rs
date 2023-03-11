#[derive(Debug, PartialEq)]
struct Player {
    score: u32,
    moves: Moves,
}

#[derive(Debug, PartialEq)]
enum Moves {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

impl Moves {
    fn play_move(&self) -> u32 {
        match self {
            Moves::Rock(v) => *v,
            Moves::Paper(v) => *v,
            Moves::Scissors(v) => *v,
        }
    }

    fn save_move(c: char) -> Moves {
        match c {
            'A' | 'X' => Moves::Rock(1),
            'B' | 'Y' => Moves::Paper(2),
            'C' | 'Z' => Moves::Scissors(3),
            _ => {
                unreachable!()
            }
        }
    }
}

fn main() {
    let f = std::fs::read_to_string("input.txt").unwrap();
    let mut moves = Vec::new();

    for c in f.chars() {
        if c.is_whitespace() {
            continue;
        }
        moves.push(Moves::save_move(c));
    }

    let mut player_1 = Player {
        score: 0,
        moves: Moves::Paper(0),
    };
    let mut player_2 = Player {
        score: 0,
        moves: Moves::Paper(0),
    };
    let mut player_1_turn = true;

    for play_move in moves {
        if player_1_turn {
            player_1.score += play_move.play_move();
            player_1.moves = play_move;
            player_1_turn = false;
            continue;
        }

        player_1_turn = true;
        player_2.score += play_move.play_move();

        if play_move == player_1.moves {
            player_1.score += 3;
            player_2.score += 3;
        } else if play_move == Moves::Scissors(3) && player_1.moves == Moves::Paper(2)
            || play_move == Moves::Rock(1) && player_1.moves == Moves::Scissors(3)
            || play_move == Moves::Paper(2) && player_1.moves == Moves::Rock(1)
        {
            player_2.score += 6;
        }
    }

    println!("Player 1 Score: {}", player_1.score);
    println!("Player 2 Score: {}", player_2.score);
}
