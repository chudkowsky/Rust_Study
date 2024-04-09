//Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.
fn if_leap_year(year1:i32)->bool{
    if year1 % 4 == 0 &&( ( year1 % 100 == 0 && year1 % 400 == 0) || year1 % 100 != 0 ){
            return true;
        }else{
            return false;
            }
        }


fn main() {
    let years = [2000,1900];
    let months = [1,2,3,4,5,6,7,8,9,10,11,12];
    for year in years{
        for month in months{
            if (month == 1 ||month == 3 || month ==5 || month ==7 || month ==8 || month ==10 || month ==12){
                println!("{} : Miesiac ma 31 dni",month);}
            else if month != 2 {
                println!("{} : Miesiac ma 30 dni",month );}
            else {
                if if_leap_year(year){
                    println!("{} : Miesiac ma 29 dni",month);
                } else {
                    println!("{} : Miesiac ma 28 dni",month);
                }
            }
        }

    }
}


