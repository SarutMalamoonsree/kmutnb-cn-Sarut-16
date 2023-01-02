fn main() {
    for i in 1..=5{
        for j in i..=5-1{
            print!("  ");
        }
        let x = (i*2)-1;
        for k in 1..=x{
            if k == x {
                println!("*");
            }else{
                print!("* ");
            }
        }
    }
}
