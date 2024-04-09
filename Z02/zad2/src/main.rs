fn f(x: f64) -> f64{
    x*x-1.0
}
fn fp(x: f64) -> f64{
    2.0*x
}
fn met_newt_loop(x0: f64, eps: f64, n: u128) -> f64{
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
fn met_newt_while(x0: f64, eps: f64, n: u128) -> f64{
    let mut it = 0;
    let mut xk = x0;
    while it<=n && f(xk).abs()>=eps{
        xk = xk - f(xk)/fp(xk);
        it+=1;
    }
    xk
}
fn met_newt_rec(x0: f64, eps: f64, n: u128) -> f64{
    if n<=0 || f(x0).abs()<=eps{
        x0
    }else{
        met_newt_rec(x0 - f(x0)/fp(x0),eps,n-1)
    }


}
fn met_newt(x0: f64, eps: f64, n: u128) -> f64{
    let mut it = 0;
    let mut xk = x0;
    for i in 0..=n{
        if it >= n || f(xk).abs()<=eps{
            break;
        }
        it+=1;
        xk = xk - f(xk)/fp(xk);
    }
    return xk;
}
fn main() {
    let x = -5.0;
    println!("{}",met_newt_loop(x,0.000005,100000));
    println!("{}",met_newt_while(x,0.000005,100000));
    println!("{}",met_newt_rec(x,0.000005,100000));
    println!("{}",met_newt(x,0.000005,100000));
}

