fn main(){
    let num = 10;
    let mut value = 1;
    for i in 2..=num{
        value = value * i;
    }
    println!("{}",value);
}
