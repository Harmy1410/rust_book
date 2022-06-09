fn main() {
    let r = 10;
    let c = 20;
    let x = gen_mat(r, c);
    let y = gen_mat(r, c);
    for _ in 1..100_000 {
        for i in 0..x.len() {
            for j in 0..x[0].len() {
                let _ = x[i][j] * y[i][j];
            }
        }
    }
    println!("{:?}", x);
}

fn gen_mat(r: u64, c: u64) -> Vec<Vec<u64>> {
    if c < r {
        println!("nope");
        return vec![];
    }
    let mut a = Vec::new();
    let mut sum = 1;
    for _ in 0..r {
        let mut x = Vec::new();
        for j in sum..c + sum {
            x.push(j);
            sum = j + 1;
        }
        a.push(x)
    }

    return a;
}
