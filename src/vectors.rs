// vectors - resizable arrays
pub fn run_vectors () {
    let mut v_num: Vec<i32> = vec![1,3,5,7,9];
    v_num.push(11);
    v_num.push(13);

    println!("{:?}", v_num);
    v_num.pop();
    println!("{:?}", v_num);
    print!("{:?} {}", v_num.len(), "\n");
    println!("{}", v_num[4]);
}