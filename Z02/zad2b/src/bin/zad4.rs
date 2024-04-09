fn if_armstrong_num(x:u64)->bool{
    let mut x_copy = x;
    let mut vec = Vec::new();
    while x_copy>=1{
        vec.push(x_copy%10);
        x_copy = x_copy/10;
    }
    let size = vec.len();
    let mut sum = 0;
    for i in 0..size{
        sum += vec[i].pow(size as u32);
    }
    sum == x
}
fn main(){
    println!("{}",if_armstrong_num(1741725)); // true
    println!("{}",if_armstrong_num(152)); //false


}
