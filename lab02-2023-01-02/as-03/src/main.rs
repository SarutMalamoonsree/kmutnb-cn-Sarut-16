fn main() {
    for i in 1..=5{
        for j in 5-(i-2)..=5{
            print!("  ");
        }
        let x = ((5-(i-1))*2)-1;
        for k in 1..=x{
            if k == x {
                println!("*");
            }else{
                print!("* ");
            }
        }
    }
}
