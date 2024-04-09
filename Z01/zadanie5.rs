// Napisz program, który dla danych dwóch poprawnych pór jednej doby (w postaci całkowitych godzin, minut i sekund) wyświetla różnicę czasów (także w postaci analogicznej trójki, z minutami i sekundami w przedziale [0;59])
fn main() {
  let mut h1 = 23;
  let h2 = 11;
  let mut m1 = 11;
  let m2 = 22;
  let s1 = 45;
  let s2 = 51;
  let h3;
  let m3;
  let s3;
  if (s1-s2)<0{
    m1 = m1 - 1;
    s3 = 60+s1 - s2;
  }else
  {s3 = s1-s2};
  if (m1-m2)<0{
    h1 = h1 - 1;
    m3 = 60+m1 - m2;
  }else
  {m3 = m1-m2};
  h3 = h1 - h2;
  println!("{}:{}:{}",h3,m3,s3);

}


