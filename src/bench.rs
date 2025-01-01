mod astar;
use astar::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn easy_1(c: &mut Criterion) {
    let s1 = "gxtxayb"; 
    let s2 = "abgtab"; 
    let s3 = "gyaytahjb"; 
    let s4 = "gyayjjjtab"; 
    let s5 = "gyaytahhhhb"; 
    let s6 = "ygaytppppahjb"; 
    let s7 = "ylllgaytmmajb"; 
    let s = vec![s1, s2, s3, s4, s5, s6, s7];
    c.bench_function("easy 1", |c|
            c.iter(|| mlcs_astar(black_box(&s), black_box(7))));
}

criterion_group!(benches, easy_1);
criterion_main!(benches);
