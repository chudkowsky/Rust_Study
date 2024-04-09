// Napisz funkcję o nagłówku

fn liczba_wystapien(napis: &str, znak: char) -> usize{
    let mut counter = 0;
    for letter in napis.chars(){
        if letter == znak{
            counter+=1;
        }
    }
    counter
}
fn main(){
    println!("{}",liczba_wystapien("maadadadffddffgsddadsaaaaaja",'a'));
}
