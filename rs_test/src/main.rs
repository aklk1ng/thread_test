use std::io;

fn fib(n: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    if n == 1 {
        res.push(1);
        res
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut c: i32;
        while a < n {
            res.push(a);
            c = b;
            b = a + b;
            a = c;
        }
        res
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Failed to convert");
    let res = fib(n);
    for v in res {
        println!("{v}");
    }
}
