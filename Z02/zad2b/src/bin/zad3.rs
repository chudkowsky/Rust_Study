fn collatz_problem(v:f64)->u64{
    let mut n = v;
    let mut counter = 0;
    loop{
        if n == 1.0{
            return counter
        }
        if n%2.0 == 1.0{
            n = 3.0*n+1.0;
        }else{
            n = 0.5*n;
        }
        counter +=1;
    }
}

fn main(){
    let n = 12.0;
    print!("{}",collatz_problem(n));
}
