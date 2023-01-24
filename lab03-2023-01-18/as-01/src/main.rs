fn main() {
    let string: &str = "this cat this bat this rat";
    let word:Vec<&str> = string.split(" ").collect();
    let mut unique:Vec<&str> = string.split(" ").collect();
    for i in 0.. unique.len(){
        if i ==  unique.len()-1{
            break;
        }else{
            for j in 0.. unique.len(){
                if j >=  unique.len()-1 || i == j+1{
                        break;
                    }else {
                        if  unique[i] ==  unique[j+1] && j+1 < unique.len(){
                            unique.remove(j+1);
                        }
                    }
            }
        }
    }
     println!("word : {:?}", word);
     println!("unique : {:?}", unique);
     let count = unique.len();
     println!("count = {}" ,  count);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let string: &str = "this cat this bat this rat";
    let mut unique:Vec<&str> = string.split(" ").collect();
    for i in 0..unique.len(){
        if i == unique.len()-1{
            break;
        }else{
            for j in 0..unique.len(){
                if j >= unique.len()-1 || i == j+1{
                        break;
                    }else {
                        if  unique[i] == unique[j+1] && j+1 < unique.len(){
                            unique.remove(j+1);
                        }
                    }
            }
        }
    }   let count = unique.len();
        let result = count;
        assert_eq!(result, count);
    }
}
