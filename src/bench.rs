use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod astar;
mod astar_app;
mod testsuite;
mod utils;
use astar::mlcs_astar;
use astar_app::astar_app;
use testsuite::generate_testcase;

pub fn random(c: &mut Criterion, f: fn(&Vec<&str>) -> String, infos: Vec<usize>) {
    let pattern = "09876_c-DGK(*&^";
    let nb_string = infos[0];
    let string_length = infos[1];

    let s_string = generate_testcase(&pattern, nb_string, string_length);

    // cast from Vec<String> to Vec<&str>
    let s = s_string.iter().map(|x| x.as_str()).collect();

    let description = format!(
        "benchmark random inputs. (string nb, length) = ({}, {})",
        nb_string, string_length
    );

    c.bench_function(&description, |c| c.iter(|| f(black_box(&s))));
}

pub fn medium(c: &mut Criterion, f: fn(&Vec<&str>) -> String) {
    random(c, f, vec![10, 10]);
    random(c, f, vec![10, 40]);
    random(c, f, vec![20, 40]);
    random(c, f, vec![5, 50]);
}
pub fn hard(c: &mut Criterion, f: fn(&Vec<&str>) -> String) {
    random(c, f, vec![50, 50]);
    random(c, f, vec![5, 80]);
}
pub fn complete_astar_app(c: &mut Criterion) {
    medium(c, astar_app);
    hard(c, astar_app);
}

pub fn complete_astar(c: &mut Criterion) {
    medium(c, mlcs_astar);
    hard(c, mlcs_astar);
}
criterion_group!(astar_app_bench, complete_astar_app);

criterion_group!(astar_bench, complete_astar);

criterion_main!(astar_app_bench, astar_bench);
