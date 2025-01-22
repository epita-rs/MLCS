use crate::utils::*;

const k: usize = 2000;
const c: u64 = 20;

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
pub fn astar_app(S: &Vec<&str>) -> String {
    // Preprocessing
    let d = S.len();

    let mut ctx = Context::new(S, d);

    // queueueue
    let mut queue: Vec<Vec<usize>> = vec![];
    init_queue(&mut queue, S, d, &mut ctx);

    while queue.len() > 0 {
        // y = max( {f(p) | p in queue} )
        let mut y = f(&ctx, queue.last().unwrap());

        // y = y - c // without overflow
        if y > c {
            y -= c;
        }

        // R = { p | p in queue and y <= f(p) }
        let R = queue
            .clone()
            .into_iter()
            .filter(|p| y <= f(&ctx, p))
            .collect::<Vec<Vec<usize>>>();
        queue.clear();

        for p in R {
            if heuristic(&ctx.MS, &p, d) == 0 {
                // An MLCS match was found
                return common_seq(&ctx, &p, S);
            } else {
                // inserting all succesors in the queue
                let succs = get_successors(&ctx, &S, &p);
                for q in succs {
                    // basically saying if the queue queue does not already
                    // contain the point q
                    if !queue.contains(&q) {
                        update_suc(p.clone(), q.clone(), &mut ctx);
                        queue.push(q);
                    } else if g(&ctx, &q) < g(&ctx, &p) + 1 {
                        update_suc(p.clone(), q.clone(), &mut ctx);
                    }
                }
            }
        }
        reorder_queue(&mut queue, &mut ctx);
    }
    return String::from("");
}
