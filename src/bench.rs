mod astar;
mod testsuite;
use astar::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testsuite::generate_testcase;

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

pub fn easy_random_1(c: &mut Criterion) {
    let pattern = "mouahahahahahahahihihihhohohoho";
    let s_string = generate_testcase(&pattern, 10, 40);
    // Line below is a basic cast from Vec<String> to Vec<&str>
    let s = s_string.iter().map(|x| x.as_str()).collect();
    c.bench_function("easy_random_1", |c|
            c.iter(|| mlcs_astar(black_box(&s), black_box(s.len()))));
}

pub fn medium_random_1(c: &mut Criterion) {
    let pattern = "dkjsdsbbhhvVGVGVUVJvhjvjfsdkhfihsfugvdqwv";
    let s_string = generate_testcase(&pattern, 20, 20);
    // Line below is a basic cast from Vec<String> to Vec<&str>
    let s = s_string.iter().map(|x| x.as_str()).collect();
    c.bench_function("easy_random_20_20", |c|
            c.iter(|| mlcs_astar(black_box(&s), black_box(s.len()))));
}

pub fn medium_random_2(c: &mut Criterion) {
    let pattern = "dkjsdsbbhhvVGVGVUVJvhjvjfsdkhfihsfugvdqw098763787658927v";
    let s_string = generate_testcase(&pattern, 3, 80);
    // Line below is a basic cast from Vec<String> to Vec<&str>
    let s = s_string.iter().map(|x| x.as_str()).collect();
    c.bench_function("easy_random_3_80", |c|
            c.iter(|| mlcs_astar(black_box(&s), black_box(s.len()))));
}

pub fn easy(c: &mut Criterion) {
    easy_1(c);
    easy_random_1(c);
}

pub fn medium(c: &mut Criterion) {
    medium_random_1(c);
    medium_random_2(c);
}


criterion_group!(benches, easy_1, easy_random_1, medium_random_2, medium_random_2);
criterion_main!(benches);
