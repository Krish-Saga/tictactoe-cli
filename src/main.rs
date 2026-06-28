use rand::prelude::*;
use std::{
    cmp::{max, min},
    io::{self},
};
struct OX([char; 9]);
struct Player;
struct DumbComputer;

impl OX {
    fn choose_move() -> bool {
        let mut select = String::new();

        print!("\x1B[2J\x1B[1;1H");
        println!(
            "                               ====================================\n                                     WELCOME TO THE O X  G A M E\n                               ===================================="
        );
        println!("Chopper : Hello Cutie 😉, Orewa Tony Tony Chopper!, \n");
        println!("Note: Enter (Capital or Small ) .\nDon't enter number or program will break\n ");
        eprint!("You will play as ( X or O ) : ");

        io::stdin()
            .read_line(&mut select)
            .expect("Enter X or O please");
        let select: char = select.trim().parse().expect("Enter X or O please");
        if select == 'x' || select == 'X' {
            return true;
        }

        false
    }
    fn board_state(state: Vec<char>) {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "                               ====================================\n                                     WELCOME TO THE O X  G A M E\n                               ===================================="
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
        if (a == letter && b == letter && c == letter)
            || (d == letter && e == letter && f == letter)
            || (g == letter && h == letter && i == letter)
            || (a == letter && d == letter && g == letter)
            || (c == letter && f == letter && i == letter)
            || (a == letter && e == letter && i == letter)
            || (c == letter && e == letter && g == letter)
            || (b == letter && e == letter && h == letter)
        {
            return true;
        }

        false
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
    fn empty_squares(&self) -> isize {
        let mut square = 0;
        for i in 1..=9 {
            if self.0[i - 1] == '_' {
                square += 1;
            }
        }
        square
    }
    fn minimax(
        &mut self,
        depth: usize,
        is_maximizing: bool,
        max_player: char,
        other_player: char,
    ) -> isize {
        if self.win_check(max_player) {
            return 1 * 1 + self.empty_squares();
        } else if self.win_check(other_player) {
            return -1 * (1 + self.empty_squares());
        } else if self.empty_squares() == 0 {
            return 0;
        }
        let undo_move = '_';
        if is_maximizing {
            let mut best_score = -100000;
            for moves in self.available_moves() {
                self.0[moves - 1] = max_player;
                let score = self.minimax(depth + 1, false, max_player, other_player);
                self.0[moves - 1] = undo_move;
                best_score = max(score, best_score);
            }
            return best_score;
        } else {
            let mut best_score = 100000;
            for moves in self.available_moves() {
                self.0[moves - 1] = other_player;
                self.manuplicate_ox(moves, other_player);
                let score = self.minimax(depth + 1, true, max_player, other_player);
                self.0[moves - 1] = undo_move;
                best_score = min(score, best_score);
            }
            return best_score;
        }
    }

    fn get_computer_move(&mut self, max_player: char, other_player: char) -> usize {
        let mut best_move = 0;
        let undo_move = '_';
        let mut best_score = -100;
        if self.available_moves().len() == 9 {
            let mut rng = rand::rng();
            best_move = rng.random_range(1..=9);
        } else {
            for moves in self.available_moves() {
                self.0[moves - 1] = max_player;
                let score = self.minimax(0, false, max_player, other_player);
                self.0[moves - 1] = undo_move;
                if score > best_score {
                    best_score = score;
                    best_move = moves;
                }
            }
        }
        best_move
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
// impl SmartComputer {
//     fn get_move(available_moves: Vec<usize>) -> usize {
//         let mut best_move;
//         let mut best_score = -10000;
//         if available_moves.len() == 9 {
//             best_move = DumbComputer::get_move(available_moves);
//         }
//         for move in available_moves {
//             self.
//             best_score =
//         }
//         best_move
//     }
// }
// impl SmartComputer {}
fn main_game() {
    let mut board = OX(['_'; 9]);
    let mut turn;

    let player;
    let other_player;
    if OX::choose_move() {
        player = 'X';
        other_player = 'O';
        turn = true;
    } else {
        player = 'O';
        other_player = 'X';
        turn = false;
    }

    OX::board_state(board.state());
    while board.empty_squares() != 0 {
        let computer_move = board.get_computer_move(other_player, player);
        let win;
        println!("Turn: Player: {} \n", player);
        if turn {
            board.manuplicate_ox(
                Player::get_move(board.available_moves(), board.state(), player),
                player,
            );
            win = board.win_check(player);

            OX::board_state(board.state());
            turn = false;
        } else {
            board.manuplicate_ox(computer_move, other_player);

            win = board.win_check(other_player);

            OX::board_state(board.state());
            turn = true;
        }

        if win {
            println!("Yay, Player  ( {player} ) won !");
            break;
        }
    }
    if board.empty_squares() == 0 {
        OX::board_state(board.state());
        println!("OX: it's a draw ");
    }
}
fn main() {
    main_game();
    // OX::start_screen();
}
