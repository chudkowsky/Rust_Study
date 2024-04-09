fn co_drugi_znak(napis: &str) -> String{
    napis.chars().step_by(2).collect()
}


fn main(){
    println!("{}",co_drugi_znak("rabarbar"));
}
