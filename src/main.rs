use std::io;
struct Player {
    letter: char,
}
struct OX([char; 9]);

impl OX {
    fn board_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "                               ==============================\n                                       O X  G A M E\n                               =============================="
        );
        println!("Player 1 (X)\n------------\n    vs\n------------\nPlayer 2 (O)\n");
        println!(
            "+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n| {}  | {}  | {}  | \n+----+----+----+\n",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8]
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

    fn manuplicate_ox(&mut self, square: usize, letter: char) -> bool {
        if self.0[square - 1] == '_' {
            self.0[square - 1] = letter;
        }
        true
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
    fn empty_squares(&self) -> usize {
        let mut square = 0;
        for i in 1..=9 {
            if self.0[i - 1] == '_' {
                square += 1;
            }
        }
        square
    }
    fn move_check(&self) -> usize {
        println!("\n Available Moves: {:?}", self.available_moves());
        let val = Player::playerinput();
        if self.available_moves().contains(&val) {
        } else if val > 9 {
            println!("Baka ! Can't you see it is pre-occupied ?\n");
            OX::move_check(&self);
        } else {
            println!("Baka ! Can't you see it is pre-occupied ?\n");
            OX::move_check(&self);
        }
        val
    }
}
impl Player {
    // Player input just takes input
    fn playerinput() -> usize {
        let mut pinput = String::new();

        io::stdin()
            .read_line(&mut pinput)
            .expect("failed to take input ");
        let pinput: usize = pinput
            .trim()
            .parse()
            .expect("Man please enter numbers only ");
        pinput
    }
}
fn main_game() {
    let mut board = OX(['_'; 9]);
    let mut turn = 0;
    board.board_state();
    'outer: loop {
        'p1: loop {
            let letter = 'X';
            println!("Turn: Player ( {letter} ) \n");
            eprint!("Enter your move(1-9): ");
            if board.manuplicate_ox(board.move_check(), letter) {
                turn += 1;
                if board.win_check(letter) {
                    board.board_state();
                    println!("Yay, Player  ( {letter} ) won !");
                    break 'outer;
                } else if turn == 9 {
                    board.board_state();
                    println!("OX: it's a draw ");
                    break 'outer;
                }
                board.board_state();
            } else {
                continue 'p1;
            }

            break 'p1;
        }
        'p2: loop {
            let letter = 'O';
            println!("Turn: Player ( {letter} )\n");
            eprint!("Enter your move(1-9): ");
            if board.manuplicate_ox(board.move_check(), letter) {
                turn += 1;
                if board.win_check(letter) {
                    board.board_state();
                    println!("Yay, Player ( {letter} ) won !");
                    break 'outer;
                } else if turn == 9 {
                    board.board_state();
                    println!("OX: it's a draw ");
                    break 'outer;
                }
                board.board_state();
            } else {
                continue 'p2;
            }
            break 'p2;
        }
    }
}
fn main() {
    main_game();
}
