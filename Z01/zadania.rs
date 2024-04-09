//zad1
fn main() {
    let years = [2000,1900,1992,2001] ; //true
    for year1 in years{
    if year1 % 4 == 0 &&( ( year1 % 100 == 0 && year1 % 400 == 0) || year1 % 100 != 0 ){
        println!("{} jest rokiem przestepnym",year1);
    }else{
        println!("{} nie jest rokiem przestepnym",year1);
        }}
}
