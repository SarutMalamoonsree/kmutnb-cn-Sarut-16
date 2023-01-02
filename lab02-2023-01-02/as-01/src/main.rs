fn main() {
    for i in 1..=5{
        for j in 5-i..=5-1{
            if j != 5-1{
                print!("* ");
            }else{
                println!("*");
            }
        }
    }
}
