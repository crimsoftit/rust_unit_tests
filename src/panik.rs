pub fn test_panik (val: i64) -> i64 {
    let res: i64;
    let e_msg = "ERR: INVALID VALUE!!!";
    if val >= 50 {
        res = val * 2;
        println!("{}", res);
        res
    } else {
        println!("{}", e_msg);
        panic!("{}", e_msg);
    }
}