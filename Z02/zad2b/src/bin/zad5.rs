fn perfect_number(x:usize)->bool{
    let mut sum = 0;
    for i in 1..x/2 +1{
        if x%i == 0{
            sum+=i}
    }
    sum==x
}
fn main(){
    println!("{}",perfect_number(308089)==false);//false
    println!("{}",perfect_number(496)==true);//true
    println!("{}",perfect_number(28)==true);//true
}
