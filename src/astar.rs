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
pub fn mlcs_astar(S: &Vec<&str>) -> String {
    // Preprocessing
    let d = S.len();

    let mut ctx = Context::new(S, d);

    let mut queue: Vec<Vec<usize>> = vec![];
    init_queue(&mut queue, S, d, &mut ctx);

    while queue.len() > 0 {
        let p: Vec<usize> = queue.pop().unwrap().clone();

        if heuristic(&ctx.MS, &p, d) == 0 {
            // An MLCS match was found
            return common_seq(&ctx, &p, S);
        } else {
            // inserting all succesors in the queue
            let succs = get_successors(&ctx, &S, &p);
            for q in succs {
                // basically saying if the queue queue does not already
                // contain the point q
                update_suc(p.clone(), q.clone(), &mut ctx);
                if !queue.contains(&q) {
                    queue.push(q);
                }
            }

            reorder_queue(&mut queue, &mut ctx);
        }
    }
    return String::from("");
}
