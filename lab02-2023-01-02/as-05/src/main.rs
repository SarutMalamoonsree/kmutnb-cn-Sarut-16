use std::io;
fn main() {
    let mut y:i32=0;
    let mut input = String::new();
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    y = input.trim().parse().expect("Not a valid number");
    for i in 1..=y{
        for j in 1..=y{
            if j == 1 || j == i {
                print!("X ");
            }else if j == y{
                println!("X");
            }else{
                print!("O ");
            }
        }
    }
}
