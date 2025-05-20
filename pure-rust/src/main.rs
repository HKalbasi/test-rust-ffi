use std::time::Instant;

fn build_vec(n: u64) -> Box<Vec<u64>> {
    let mut r = Box::new(vec![]);
    for i in 0..n {
        r.push(i);
    }
    r
}

fn main() {
    let start = Instant::now();
    for _ in 0..1_000_000 {
        std::hint::black_box(build_vec(1000));
    }
    println!("{:?}", start.elapsed());
}
