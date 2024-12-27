mod astar;
mod testsuite;
use astar::mlcs_astar;

fn main() {

    let s1 = "Helllo";
    let s2 = "Helllo";
    let s3 = "Helllo";
    let s4 = "Helllo";

    let S = vec![s1, s2, s3, s4];
    
    let res = mlcs_astar(&S, 4);
    println!("{}", res);
}
