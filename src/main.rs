use rand::prelude::*;
use std::io;
struct OX([char; 9]);
struct Player;
struct DumbComputer;

impl OX {
    fn board_state(state: Vec<char>) {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "                               ==============================\n                                       O X  G A M E\n                               =============================="
        );
        println!("Player 1 (X)\n------------\n    vs\n------------\nPlayer 2 (O)\n");
        println!(
            "+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n",
            state[0],
            state[1],
            state[2],
            state[3],
            state[4],
            state[5],
            state[6],
            state[7],
            state[8]
        );
    }
    fn win_check(&self, letter: char) -> bool {
        let a = self.0[0];
        let b = self.0[1];
        let c = self.0[2];
        let d = self.0[3];
        let e = self.0[4];
        let f = self.0[5];
        let g = self.0[6];
        let h = self.0[7];
        let i = self.0[8];
        let mut win: bool = false;
        if (a == letter && b == letter && c == letter)
            || (d == letter && e == letter && f == letter)
            || (g == letter && h == letter && i == letter)
            || (a == letter && d == letter && g == letter)
            || (c == letter && f == letter && i == letter)
            || (a == letter && c == letter && i == letter)
            || (c == letter && e == letter && g == letter)
            || (b == letter && e == letter && h == letter)
        {
            win = true;
        }

        win
    }

    fn manuplicate_ox(&mut self, square: usize, letter: char) {
        if self.0[square - 1] == '_' {
            self.0[square - 1] = letter;
        }
    }
    fn available_moves(&self) -> Vec<usize> {
        let mut avail_mov: Vec<usize> = Vec::new();
        for i in 1..=9 {
            if self.0[i - 1] == '_' {
                avail_mov.push(i);
            }
        }
        avail_mov
    }

    fn state(&self) -> Vec<char> {
        let mut state: Vec<char> = Vec::new();
        for i in 0..9 {
            state.push(self.0[i]);
        }
        state
    }
    fn empty_squares(&self) -> usize {
        let mut square = 0;
        for i in 1..=9 {
            if self.0[i - 1] == '_' {
                square += 1;
            }
        }
        square
    }
}
impl Player {
    fn get_move(available_moves: Vec<usize>, state: Vec<char>, letter: char) -> usize {
        let mut valid_square = false;
        let mut value = 0;
        while !valid_square {
            eprint!("Enter your move(1-9): ");
            let mut pinput = String::new();

            io::stdin()
                .read_line(&mut pinput)
                .expect("failed to take input ");
            let pinput: usize = pinput
                .trim()
                .parse()
                .expect("Man please enter numbers only ");
            value = pinput;
            if available_moves.contains(&pinput) {
                valid_square = true;
            } else {
                OX::board_state(state.clone());
                println!("\nOX: Baka ! That's an Invalid Move \n");

                println!("Available Moves: {:?}\n", available_moves);

                println!("Turn: Player ( {} )\n", letter);
            }
        }

        value
    }
}

impl DumbComputer {
    fn get_move(available_moves: Vec<usize>) -> usize {
        let mut valid_square = false;
        let mut val = 0;
        while !valid_square {
            let mut rng = rand::rng();
            let square = rng.random_range(1..=9);
            val = square;

            if available_moves.contains(&square) {
                valid_square = true;
            }
        }
        val
    }
}
fn main_game() {
    let mut board = OX(['_'; 9]);

    OX::board_state(board.state());
    let mut letter = 'X';
    DumbComputer::get_move(board.available_moves());
    while board.empty_squares() != 0 {
        println!("Turn: Player ( {} )\n", letter);
        if letter == 'X' {
            board.manuplicate_ox(
                Player::get_move(board.available_moves(), board.state(), letter),
                letter,
            );
            OX::board_state(board.state());
        } else {
            board.manuplicate_ox(DumbComputer::get_move(board.available_moves()), letter);
            OX::board_state(board.state());
        }

        if board.win_check(letter) {
            println!("Yay, Player  ( {letter} ) won !");
            break;
        } else if board.empty_squares() == 0 {
            OX::board_state(board.state());
            println!("OX: it's a draw ");
            break;
        }

        if letter == 'X' {
            letter = 'O';
        } else {
            letter = 'X';
        }
    }
}
fn main() {
    main_game();
}
