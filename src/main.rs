use std::io;
struct OX {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
    h: char,
    i: char,
}

impl OX {
    fn board_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let e = self.e;
        let f = self.f;
        let g = self.g;
        let h = self.h;
        let i = self.i;
        println!(
            "                               ==============================\n                                       O X  G A M E\n                               =============================="
        );
        println!("Player 1 (X)\n------------\n    vs\n------------\nPlayer 2 (O)\n");
        println!(
            "+----+----+----+\n| {a}  | {b}  | {c}  | \n+----+----+----+\n| {d}  | {e}  | {f}  | \n+----+----+----+\n| {g}  | {h}  | {i}  | \n+----+----+----+\n"
        );
    }
    fn game_over(&self, p: char) -> bool {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;
        let e = self.e;
        let f = self.f;
        let g = self.g;
        let h = self.h;
        let i = self.i;
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

    fn manuplicate_ox(&mut self, n: u8, p: char) -> bool {
        let mut noerr = true;
        if n == 1 && self.a == '_' {
            self.a = p;
        } else if n == 2 && self.b == '_' {
            self.b = p;
        } else if n == 3 && self.c == '_' {
            self.c = p;
        } else if n == 4 && self.d == '_' {
            self.d = p;
        } else if n == 5 && self.e == '_' {
            self.e = p;
        } else if n == 6 && self.f == '_' {
            self.f = p;
        } else if n == 7 && self.g == '_' {
            self.g = p;
        } else if n == 8 && self.h == '_' {
            self.h = p;
        } else if n == 9 && self.i == '_' {
            self.i = p;
        } else if n > 9 {
            println!("Baka ! Enter number in between 1 to 9");
            noerr = false;
        } else {
            println!("Baka ! Can't you see it is pre-occupied ?");
            noerr = false;
        }
        noerr
    }
    // Player input just takes input
    fn playerinput() -> u8 {
        let mut pinput = String::new();
        io::stdin()
            .read_line(&mut pinput)
            .expect("failed to take input ");
        let pinput: u8 = pinput
            .trim()
            .parse()
            .expect("Man please enter numbers only ");
        pinput
    }
}
fn main() {
    let mut players = OX {
        a: '_',
        b: '_',
        c: '_',
        d: '_',
        e: '_',
        f: '_',
        g: '_',
        h: '_',
        i: '_',
    };
    players.board_state();
    'outer: loop {
        'p1: loop {
            let p1 = 'X';
            println!("Turn: Player 1 (X)\n");
            eprint!("Enter your move(1-9): ");
            if players.manuplicate_ox(OX::playerinput(), p1) {
                if players.game_over(p1) {
                    players.board_state();
                    println!("Yay, Player 1 ( X ) won !");
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
                if players.game_over(p2) {
                    players.board_state();
                    println!("Yay, Player 2 ( O ) won !");
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
