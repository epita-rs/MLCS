use std::rc::Rc;
use crate::utils::*;

/// Outputs the Longest Common Subsequence among Multiple strings (MLCS)
///
/// # Arguments
///
/// * `S` - Array of strings.
///
/// # Returns
///
/// * `String` if the strings are not anagrams.
/// * `String::new("")' if no MLCS was found
pub fn mlcs_astar(chains: &Vec<&str>) -> String {
    // Preprocessing
    let mut ctx = Context::new(chains);

    let mut queue: Vec<Rc<Vec<usize>>> = vec![];
    init_queue(&mut ctx, &mut queue);

    while queue.len() > 0 {
        let p: Rc<Vec<usize>> = queue.pop().unwrap();

        if heuristic(&ctx, &p) == 0 {
            // An MLCS match was found
            return common_seq(&ctx, p);
        } else {
            // inserting all succesors in the queue
            let succs = get_successors(&ctx, &p);
            for q in succs {
                // basically saying if the queue queue does not already
                // contain the point q
                update_suc(&mut ctx, &p, &q);
                if !queue.contains(&q) {
                    queue.push(Rc::clone(&q));
                }
            }

            reorder_queue(&mut ctx, &mut queue);
        }
    }
    return String::from("");
}
