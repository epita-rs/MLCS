use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod utils;
mod astar;
mod astar_app;
mod testsuite;
use astar::mlcs_astar;
use astar_app::astar_app;
use testsuite::generate_testcase;

pub fn random(c: &mut Criterion, f: fn(&Vec<&str>, usize) -> String,
              infos:Vec<usize>) {

    let pattern = "09876_c-DGK(*&^";
    let nb_string= infos[0];
    let string_length= infos[1];

    let s_string = generate_testcase(&pattern, nb_string, string_length);

    // cast from Vec<String> to Vec<&str>
    let s = s_string.iter().map(|x| x.as_str()).collect();

    let description = format!(
               "benchmark random inputs. (string nb, length) = ({}, {})",
               nb_string,
               string_length);

    c.bench_function(
        &description, 
        |c| c.iter(|| f(black_box(&s), black_box(s.len())))
    );
}

pub fn medium(c: &mut Criterion, f: fn(&Vec<&str>, usize) -> String) {
    random(c, f, vec![10, 10]);
    random(c, f, vec![10, 40]);
    random(c, f, vec![20, 40]);
    random(c, f, vec![5, 50]);
}

pub fn medium_astar_app(c: &mut Criterion) {
    medium(c, astar_app);
}

pub fn medium_astar(c: &mut Criterion) {
    medium(c, mlcs_astar);
}
criterion_group!(astar_app_bench,
                 medium_astar_app);

criterion_group!(astar_bench,
                medium_astar);

criterion_main!(astar_app_bench, astar_bench);
