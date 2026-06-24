use std::io;
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
    fn win_check(&self, p: char) -> bool {
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
        if (a == p && b == p && c == p)
            || (d == p && e == p && f == p)
            || (g == p && h == p && i == p)
            || (a == p && d == p && g == p)
            || (c == p && f == p && i == p)
            || (a == p && c == p && i == p)
            || (c == p && e == p && g == p)
            || (b == p && e == p && h == p)
        {
            win = true;
        }

        win
    }

    fn manuplicate_ox(&mut self, n: usize, p: char) -> bool {
        let mut noerr = true;
        if self.0[n] == '_' {
            self.0[n] = p;
        } else if n > 9 {
            println!("Baka ! Enter number in between 1 to 9\n");
            noerr = false;
        } else {
            println!("Baka ! Can't you see it is pre-occupied ?\n");
            noerr = false;
        }
        noerr
    }
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
fn main() {
    let mut players = OX(['_', '_', '_', '_', '_', '_', '_', '_', '_']);
    let mut turn = 0;
    players.board_state();
    'outer: loop {
        'p1: loop {
            let p1 = 'X';
            println!("Turn: Player 1 (X)\n");
            eprint!("Enter your move(1-9): ");
            if players.manuplicate_ox(OX::playerinput(), p1) {
                turn += 1;
                if players.win_check(p1) {
                    players.board_state();
                    println!("Yay, Player 1 ( X ) won !");
                    break 'outer;
                } else if turn == 9 {
                    players.board_state();
                    println!("OX: it's a draw ");
                    break 'outer;
                }
                players.board_state();
            } else {
                continue 'p1;
            }

            break 'p1;
        }
        'p2: loop {
            let p2 = 'O';
            println!("Turn: Player 2 (O)\n");
            eprint!("Enter your move(1-9): ");
            if players.manuplicate_ox(OX::playerinput(), p2) {
                turn += 1;
                if players.win_check(p2) {
                    players.board_state();
                    println!("Yay, Player 2 ( O ) won !");
                    break 'outer;
                } else if turn == 9 {
                    players.board_state();
                    println!("OX: it's a draw ");
                    break 'outer;
                }
                players.board_state();
            } else {
                continue 'p2;
            }
            break 'p2;
        }
    }
}
