fn main(){
    let mut num = 10;
    for i in 1..=num{
        for j in 1..=num{
            for k in 1..=num{
                if i*i == j*j + k*k {
                    println!("{} {} {} ",i,j,k);
                }
            }
        }
    }
}
