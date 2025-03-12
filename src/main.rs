use std::io;
fn main() {
    println!("Welcome to my little game!\nYou shall not beat me.");
    let v = make_checklist();
    let mut total: u32 = calc_optimal_move(0, &v);
    println!(
        "I call first! I start with {}. On your turn, type 1, 3, or 4. I will add it for you. If you try any funny business, I will panic.",
        total
    );
    loop {
        total = get_number(total);
        println!(
            "I play {}, bringing the total to {}.",
            calc_optimal_move(total, &v),
            calc_optimal_move(total, &v) + total
        );
        total = calc_optimal_move(total, &v) + total;
        if total >= 25 {
            break;
        }
    }
    println!("I win!");
}

fn get_number(total: u32) -> u32 {
    let mut input = String::with_capacity(1);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line - I blame you.");
    let number: i32 = input
        .trim()
        .parse()
        .expect("That isn't a number - I hate you!");
    if (number == 1 || number == 3 || number == 4) && number as u32 + total <= 25 {
        let number: u32 = number as u32;
        total + number
    } else {
        println!("Bad number");
        get_number(total)
    }
}

fn calc_optimal_move(total: u32, v: &Vec<bool>) -> u32 {
    if v[(total + 4) as usize] {
        4
    } else if v[(total + 3) as usize] {
        3
    } else if v[(total + 1) as usize] {
        1
    } else {
        0
    }
}

fn make_checklist() -> Vec<bool> {
    let mut v: Vec<bool> = vec![false; 26];
    for number in (0..26).rev() {
        if number == 25 {
            v[number] = true;
        } else if v[number + 1]
            || (number + 3 <= v.len() - 1 && v[number + 3])
            || (number + 4 <= v.len() - 1 && v[number + 4])
        {
            v[number] = false;
        } else {
            v[number] = true;
        }
        // Debug code
        // println!("{}, {}", number, v[number]);
    }
    v
}
