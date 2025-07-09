use std::io;
pub fn run() {
    let checklist = make_checklist();
    let moves = vec![4, 3, 1];
    let mut total: u32 = calc_optimal_move(0, &checklist, &moves);
    println!(
        "I call first! I start with {}. On your turn, type 1, 3, or 4. I will add it for you.",
        total
    );
    loop {
        total = get_number(total, &moves);
        println!(
            "I play {}, bringing the total to {}.",
            calc_optimal_move(total, &checklist, &moves),
            calc_optimal_move(total, &checklist, &moves) + total
        );
        total = calc_optimal_move(total, &checklist, &moves) + total;
        if total >= 25 {
            break;
        }
    }
    println!("I win!");
}

pub fn get_number(total: u32, moves: &Vec<i32>) -> u32 {
    let mut input = String::with_capacity(1);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line - I blame you.");
    let number: i32 = input.trim().parse().unwrap_or(0);
    if moves.contains(&number) && number as u32 + total <= 25 {
        let number: u32 = number as u32;
        total + number
    } else {
        println!("Bad input, try again with total {}", total);
        get_number(total, moves)
    }
}

pub fn calc_optimal_move(total: u32, checklist: &[bool], moves: &Vec<i32>) -> u32 {
    for number in moves {
        if total + (*number as u32) <= checklist.len() as u32 - 1 && checklist[(total + (*number as u32)) as usize] {
            return *number as u32;
        }
    };
    1
}

pub fn make_checklist() -> Vec<bool> {
    let mut checklist: Vec<bool> = vec![false; 26];
    for number in (0..=25).rev() {
        checklist[number] = number == 25
            || (number + 3 > checklist.len() - 1 || !checklist[number + 3])
            && !checklist[number + 1]
            && (number + 4 > checklist.len() - 1 || !checklist[number + 4]);
        // Debug code
        // println!("{}, {}", number, checklist[number]);
    }
    checklist
}
