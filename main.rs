use rand::Rng;

fn main() {
    let mut p1c = 0;
    let mut p2c = 0;
    let mut turncount = 0;
    let mut giveprotect = 0;

    loop {
        if turncount % 2 == 0 {
            let mut num = rand::thread_rng().gen_range(1..7);
            println!("player 1, you rolled a {}, you started with {} coins!", num, p1c);
            if giveprotect == 1 {
                if p1c < num {
                    num = num - p1c;
                    p1c = 0;
                } else {
                    p1c = p1c - num;
                }
                giveprotect = 0;
            }
            println!("player 1! you have {} coins!", p1c);
            let usaction = act(1, num);
            if usaction == 1 {
                // TAKE
                p1c = give(p1c, num);
            } else if usaction == 2 {
                // GIVE
                p2c = give(p2c, num);
                giveprotect = 1;
            } else if usaction == 3 {
                println!("player 1 wants to throw these coins. player 2, do you want to throw? (enter no for no, otherwise enter everything.)");
                let mut throwchoice = String::new();
                std::io::stdin().read_line(&mut throwchoice).expect("wow");
                if throwchoice == "no" {
                    give(p1c, num);
                    give(p2c, num);
                }
            }
        } else {
            let mut num = rand::thread_rng().gen_range(1..7);
            println!("player 2, you rolled a {}, you started with {} coins!", num, p2c);
            if giveprotect == 1 {
                if p2c < num {
                    num = num - p2c;
                    p2c = 0;
                } else {
                    p2c = p2c - num;
                }
                giveprotect = 0;
            }
            println!("player 2! you have {} coins!", p2c);
            let usaction = act(2, num);
            if usaction == 1 {
                // TAKE
                p2c = give(p2c, num);
            } else if usaction == 2 {
                // GIVE
                p1c = give(p1c, num);
                giveprotect = 1;
            } else if usaction == 3 {
                println!("player 2 wants to throw these coins. player 1, do you want to throw? (enter no for no, otherwise enter everything.)");
                let mut throwchoice = String::new();
                std::io::stdin().read_line(&mut throwchoice).expect("wow");
                if throwchoice == "no" {
                    give(p1c, num);
                    give(p2c, num);
                }
            }
        }

        if checkgamestate(p1c, p2c) > 0 {
            break;
        }

        turncount = turncount + 1;
    }
}

// check if either player has won.
fn checkgamestate(x: i8, y: i8) -> i8 {
    if x >= 7 {
        println! ("player 2 wins!");
        return 2
    } else if y >= 7 {
        println! ("player 1 wins!");
        return 1
    } else {
        return 0
    }
}

// function for claiming coin
fn give(mut c: i8, p: i8) -> i8 {
    c = c + p;
    return c
}

// function to gather user input for what they want to do with their coins
fn act(a: i8, b: i8) -> i8 {
    println!("player {}, what will you do with {} coins? (take/give/throw)", a, b);
    let mut user = String::new();
    let mut _x : i8;

    loop {
        std::io::stdin().read_line(&mut user).expect("wow");
        if user.trim().eq("take") {
            let x = 1;
            return x;
        } else if user.trim().eq("give") {
            let x = 2;
            return x;
        } else if user.trim().eq("throw") {
            let x = 3;
            return x;
        } else {
            println!("invalid input!");
            user = String::new();
        }
    }

}