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
}