//która zamieni wartości dwóch podanych elementów pewnej tablicy.
fn swap_arr(arr: &mut[usize], i: usize, j: usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn main(){
    let mut tablica = [1, 2, 4, 10];
    swap_arr(&mut tablica,1,2);
    println!("{:?}",tablica);
}
