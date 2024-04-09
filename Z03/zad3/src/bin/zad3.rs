//Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w
////pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.

fn rand(seed: &mut u128, min_rand: i128, max_rand: i128) -> i128 {
    *seed = (*seed * 75 + 74) % 65536 + 1;
    (*seed as i128 % (max_rand - min_rand + 1)) + min_rand
}
fn main(){
    let mut ziarno = 100;
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
    println!("{}",rand(&mut ziarno,1,6));
}
