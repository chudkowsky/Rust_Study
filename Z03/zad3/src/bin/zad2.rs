//Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów
//(dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.
fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn fun(a: &mut i32, b: &mut i32,c: &mut i32){
    if *a>*b{
        swap( a, b);
        if *b>*c{
        swap( b, c);}
    }
    if *a>*c{
        swap( a, c);
        if *b>*c{
        swap( b, c);}
    }
    if *b>*c{
        swap( b, c);
        if *a>*b{
            swap(a,b);
        }
    }

}

fn main(){
    let mut a = 3;
    let mut b = 1;
    let mut c = 2;
    fun(&mut a,&mut b,&mut c);
    println!("{},{},{}", a, b,c);

}
