fn main(){
    let mut num = 135;
    let mut sum = 0;
    while num>0{
        sum += num%10;
        num /=10;
    }
    println!("{}",sum);
}
