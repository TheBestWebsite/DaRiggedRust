use std::io;
fn main() {
    println!("Welcome to my little game!\nYou shall not beat me.");
    //let mut v: Vec<u8> = Vec::new()
    let mut v: Vec<bool> = vec![false; 26];
    for number in (0..25).rev() {
        if number == 25 {
            v[number] = true;
        } else if v[number + 1] == false
            && (number + 3 <= v.len() - 1 && v[number + 3] == false)
            && (number + 4 <= v.len() - 1 && v[number + 4] == false)
        {
            v[number] = true;
        } else {
            v[number] = false;
        }
    }
    let mut total: u32 = 4;
    println!(
        "I call first! I start with {}. On your turn, type 1, 3, or 4. I will add it for you. If you try any funny business, I will panic.",
        total
    );
    total = get_number(total);
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
    if number == 1 || number == 3 || number == 4 {
        let number: u32 = number as u32;
        total + number
    } else {
        println!("Bad number");
        get_number(total)
    }
}
