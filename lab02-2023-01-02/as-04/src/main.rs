use std::io;
fn main() {
    let mut x:i32=0;
    let mut input = String::new();
    let n = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97];
    let mut a = [0; 25];
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    x = input.trim().parse().expect("Not a valid number");
    print!("{} = ",x);
    for i in 0..100{
        for j in n{
            if x%j == 0 {
                a[i] = j ;
                
                x = x/j;
                print!("{}",a[i]);
                if x != 1{
                    print!("*");
                }else{
                    break;
                }
                break;
            }
        }

    }
}
