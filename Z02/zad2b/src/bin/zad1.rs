fn f1(x: f64) -> f64{
    x*x-1.0
}
fn fp1(x: f64) -> f64{
    2.0*x
}



fn f2(x: f64) -> f64{
    x*x*x+2.0*x*x-1.0
}
fn fp2(x: f64) -> f64{
    3.0*x*x + 4.0*x
}
fn met_newt_loop(x0: f64, eps: f64, n: u128,f:fn(f64)->f64,fp:fn(f64)->f64) -> f64{
    let mut it = 0;
    let mut xk = x0;
    loop{
        if it >= n || f(xk).abs()<=eps{
            return xk;
        }
        it+=1;
        xk = xk - f(xk)/fp(xk);
    }
}

fn main() {
    let x = 5.0;
    println!("{}",met_newt_loop(x,0.000005,100000,f1,fp1));
    println!("{}",met_newt_loop(x,0.00000000005,10000000000,f2,fp2));
}

