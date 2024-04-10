//Napisz program, który wyświetla liczbę dni miesiąca na podstawie jego numeru i numeru roku.
fn leap_year(year: usize)->bool{
    (year%4==0 && year%100!=0) || year%400 == 0
    
}

fn days_in_month(year: usize, month: usize)->usize{
   match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap_year(year) {
                29
            } else {
                28
            }
        }, 
        _ => 0, // Invalid month number
    }
}
fn main() {
    let years = [2000,1900];
    let months = [1,2,3,4,5,6,7,8,9,10,11,12];
    for year in years{
        for month in months{
                    println!("{}",days_in_month(year,month));
        }
    }
}


