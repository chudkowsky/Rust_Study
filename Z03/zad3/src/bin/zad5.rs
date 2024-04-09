fn rand(seed: &mut u128, min_rand: i128, max_rand: i128) -> i128 {
    *seed = (*seed * 75 + 74) % 65536 + 1;
    (*seed as i128 % (max_rand - min_rand + 1)) + min_rand
}
fn swap_arr(arr: &mut[usize], i: usize, j: usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn shuffle(arr: &mut[usize]){
    let mut seed = 100;
    for i in 0.. arr.len(){
        let j = rand(&mut seed,0,arr.len() as i128);
        swap_arr(arr,i,j as usize);
    }

}

fn main(){
    let mut tablica = [1, 2, 4, 10];
    shuffle(&mut tablica);
    println!("{:?}",tablica);
}
