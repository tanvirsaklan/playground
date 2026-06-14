fn sum_vec(v: Vec<i32>) -> (Vec<i32>, i32) {
    let sum: i32 = v.iter().sum::<i32>();
    (v, sum)
}
fn double_values(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len() {
        v[i] *= 2;
    }
    v
}
fn main() {
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let (v, s) = sum_vec(numbers);
    println!("Given list: {:?}\nTotal sum is: {}", v, s);
    println!("Doubled: {:?}", double_values(v.clone()));
    println!("Original: {:?}",v);
}
/*
fn main() {
    let numbers: Vec<i32> = vec![1,2,3,4,5];
    let (v, s) = sum_vec(numbers);
    println!("Given list: {:?}\nTotal sum is: {}", v, s);
    println!("Doubled: {:?}", double_values(v));
    println!("Original: {:?}",v); // can't print bcz v is borrowed and changed. original does not exist.
}
 */