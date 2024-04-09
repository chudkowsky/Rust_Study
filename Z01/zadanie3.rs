//Napisz program służący do konwersji wartości temperatury podanej w stopniach Celsjusza na stopnie w skali Fahrenheita
//F=32+95C
fn main() {
  let temp_in_c = 100.0;
  println!("{} w celicjuszach to {} w Fahrenheitach",temp_in_c, 32.0 + ((9.0/5.0)*temp_in_c));
}

