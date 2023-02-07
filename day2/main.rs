// A Y B X C Z

#[derive(PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors
}

struct Move { sign: Sign, beats: Sign, points: i8 }

impl Move {
    fn new(sign: Sign, beats: Sign, points: i8) -> Move {
        Move { sign: sign, beats: beats, points: points }
    }

    fn eval_round(&self, opp_move: &Move) -> i8 {
        if self.sign == opp_move.beats {
            return self.points;
        }
        if self.sign == opp_move.sign {
            return self.points + 3;
        }
        return self.points + 6;
    }

    fn make(sign: Sign) -> Move
    {
        match sign {
            Sign::Rock => Move::new(Sign::Rock, Sign::Scissors, 1),
            Sign::Paper => Move::new(Sign::Paper, Sign::Rock, 2),
            Sign::Scissors => Move::new(Sign::Scissors, Sign::Rock, 3)
        }
    }

    fn strategy(&self) -> Move {
        match self.sign {
            Sign::Rock => Move::make(Sign::Paper),
            Sign::Paper => Move::make(Sign::Rock),
            Sign::Scissors => Move::make(Sign::Scissors)
        }
    }
}

fn main() {
    let rock: Move = Move::new(Sign::Rock, Sign::Scissors, 1);
    let paper: Move = Move::new(Sign::Paper, Sign::Rock, 2);
    let scissors: Move = Move::new(Sign::Scissors, Sign::Paper, 3);

    let mut my_points: i8 = 0;
    let mut their_points: i8 = 0;

    let their_moves = vec![rock, paper, scissors];
    let their_moves_iter = their_moves.iter();
    for their_move in their_moves_iter {
        let my_move = their_move.strategy();
        their_points += their_move.eval_round(&my_move);
        my_points += my_move.eval_round(&their_move);
        println!("Points:\nMine: {my_points}, Theirs: {their_points}");
    }
}
