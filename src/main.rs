mod astar;
mod testsuite;
use astar::mlcs_astar;

fn main() {

    let s1 = "Hey world";
    let s2 = "Hello world !";
    let s3 = "Hi world!";

    let S = vec![s1, s2, s3];
    
    let res = mlcs_astar(&S, 3);
    println!("{}", res);
}
