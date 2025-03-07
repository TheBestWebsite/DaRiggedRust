fn main() {
    println!("Welcome to my little game!\nYou shall not beat me.");
    //let mut v: Vec<u8> = Vec::new()
    let mut v = vec![false; 25];
    for number in (1..25).rev() {
        if number == 25 {
            v[number - 1] = true
        } else if v[number] == false && v[number + 2] == false && v[number + 3] == false {
            v[number - 1] = true
        } else {
            v[number - 1] = false
        }
    }
}