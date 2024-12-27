use std::process::Command;
use std::env::args;
use std::fs::read_to_string;
use crate::astar;
use astar::mlcs_astar;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_3() {
        let s1 = "wowww"; 
        let s2 = "ewwww"; 
        let s3 = "wwhjhkjkjkww"; 
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s, 3), "ww");
    }
}
