fn main() {
    let mut a : usize = 0;
    let mut b : usize = 1;
    let mut c : usize;
    for _i in 0..50 {
        c = a + b;
        a = b;
        b = c;
        println!("{}", c);
    }
}
