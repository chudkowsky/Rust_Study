//Napisz funkcję o nagłówku
//która dla napisu reprezentującego liczbę w zapisie rzymskim (zakładamy jego poprawność) zwraca liczbę reprezentowaną przez ów napis. Przykłady:

//rzymskie("III") == 3
//rzymskie("IX") == 9
//rzymskie("XIX") == 19
//rzymskie("MCMX") == 1910
fn znak(znak: char) -> usize{
    if znak == 'I'{return 1;}
    if znak == 'V'{return 5;}
    if znak == 'X'{return 10;}
    if znak == 'L'{return 50;}
    if znak == 'C'{return 100;}
    if znak == 'D'{return 500;}
    if znak == 'M'{return 1000;
    }
    return 0;
}


fn rzymskie(napis: &str) -> usize {
    let mut suma = 0;
    let mut current;
    let mut next;
    //let mut prev;
    for i in (0..napis.len()-1).rev() {
        current = znak(napis.chars().nth(i).unwrap());
        next = znak(napis.chars().nth(i + 1).unwrap());
        if current >= next {
            suma += current;
        } else {
            suma += current-next;
        }
    }
    suma += znak(napis.chars().nth(0).unwrap());
    return suma;
}
fn main() {
    println!("{}", rzymskie("III"));
    println!("{}", rzymskie("IX"));
    println!("{}", rzymskie("XIX"));
    println!("{}", rzymskie("MCMX"));
}
