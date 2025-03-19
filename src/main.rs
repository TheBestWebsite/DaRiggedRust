use std::io;
fn main() {
    println!("Welcome to my little game!\nYou shall not beat me.");
    let checklist = make_checklist();
    let mut total: u32 = calc_optimal_move(0, &checklist);
    println!(
        "I call first! I start with {}. On your turn, type 1, 3, or 4. I will add it for you.",
        total
    );
    loop {
        total = get_number(total);
        println!(
            "I play {}, bringing the total to {}.",
            calc_optimal_move(total, &checklist),
            calc_optimal_move(total, &checklist) + total
        );
        total = calc_optimal_move(total, &checklist) + total;
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
    let number: i32 = input.trim().parse().unwrap_or(0);
    if (number == 1 || number == 3 || number == 4) && number as u32 + total <= 25 {
        let number: u32 = number as u32;
        total + number
    } else {
        println!("Bad input, try again with total {}", total);
        get_number(total)
    }
}

fn calc_optimal_move(total: u32, checklist: &Vec<bool>) -> u32 {
    if total + 4 <= checklist.len() as u32 - 1 && checklist[(total + 4) as usize] {
        4
    } else if total + 3 <= checklist.len() as u32 - 1 && checklist[(total + 3) as usize] {
        3
    } else if checklist[(total + 1) as usize] {
        1
    } else {
        0
    }
}

fn make_checklist() -> Vec<bool> {
    let mut checklist: Vec<bool> = vec![false; 26];
    for number in (0..26).rev() {
        if number == 25 {
            checklist[number] = true;
        } else if checklist[number + 1]
            || (number + 3 <= checklist.len() - 1 && checklist[number + 3])
            || (number + 4 <= checklist.len() - 1 && checklist[number + 4])
        {
            checklist[number] = false;
        } else {
            checklist[number] = true;
        }
        // Debug code
        // println!("{}, {}", number, checklist[number]);
    }
    checklist
}
