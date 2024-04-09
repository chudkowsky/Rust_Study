//Wyświetl tabelę widzialnych znaków ASCII wraz kodami (od 33 do 126).
fn main(){
    let mut i:u8 = 33;
    loop{
    	println!("{}:{}", i,i as char);
    	if i == 126{
    	break;}
    	else{
    		i+=1;
    	}
        
    }
}
