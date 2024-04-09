fn main(){
    let num = 5;
    let mut it = 2;
    let mut result = 1;
    while it <= num{
        result*=it;
        it+=1;
    }
    print!("{}",result);
}

