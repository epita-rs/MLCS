use std::process::Command;
use std::env::args;
use std::fs::read_to_string;
use crate::astar;
use astar::mlcs_astar;

pub fn output_results() {
    // ==== getting CLI arguments ====
    // REMARK : the binary will be called like ./executable file1 file2 ...
    let args: Vec<String> = args().collect();
    let n = args.len();
    if n > 0 {
        // reading the files provided
        let strings: Vec<String> = args
                // this line skips the program's name A.K.A strings[0]
                .iter().skip(1)
                // reads the content of the files
                .map(|filename| read_to_string(filename)
                    .expect("Should have been able to read the file"))
                .collect();
        let casted: Vec<&str> = strings.iter().map(AsRef::as_ref).collect();
         // ==== Assuming mlcs_astar() will later output a string ====
         // REMARK : the output will be used in ./testsuite.sh
         println!("{}", mlcs_astar(&casted, n));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(1, 1);
    }
    #[test]
    fn test_1() {
        let output_cpp = Command::new("testfiles/cpp-implementation/./mlcs")
          .args(["testfiles/file1", "testfiles/file2"])
          .output()
          .expect("cpp implementation failed");
       let args:Vec<&str> = vec!["testfiles/file1", "testfiles/file2"];
       let strings: Vec<String> = args
                  // this line skips the program's name A.K.A strings[0]
                  .iter()
                  // reads the content of the files
                  .map(|filename| read_to_string(filename)
                      .expect("Should have been able to read the file"))
                  .collect();
       let casted: Vec<&str> = strings.iter().map(AsRef::as_ref).collect();
       // uncomment this line when mlcs_astar will output strings |||
       // rather than digits                                      vvv
       //assert_eq!(output_cpp.stdout, mlcs_astar(&casted, args.len())); 
    }
}
