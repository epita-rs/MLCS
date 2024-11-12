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
