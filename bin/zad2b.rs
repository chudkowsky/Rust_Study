// Zdefiniuj funkcję o nagłówku


// która dla danego napisu zwróci ten sam napis zaszyfrowany prostym szyfrem odwracającym — klucz określa długość odwracanych fragmentów. Przykłady:
/*
szyfruj("Aladyn", 2) == "lAdany"
szyfruj("Aladyn", 3) == "alAnyd"
szyfruj("Aladyn", 4) == "dalAny"
szyfruj("Aladyn", 5) = "ydalAn"
szyfruj("koza", 3) == "zoka"
szyfruj("kaszanka", 3) == "saknazak"
szyfruj("kot Mruczek", 9) == "zcurM tokke"
szyfruj("kot Mruczek", 1) == "kot Mruczek"
szyfruj("kot Mruczek", 2) == "ok trMcuezk"*/

fn szyfruj(napis: &str, klucz: usize) -> String{
    let result;
    for i in 0..(napis.len()/klucz){
        result += napis.chars().take(klucz).collect();

    }
    result
}

fn main(){
    println!("{}",szyfruj("Aladyn",2));
}
