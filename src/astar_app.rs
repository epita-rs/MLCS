use crate::utils::*;


/// Outputs the Longest Common Subsequence among Multiple strings (LCSM)
/// using a similar approach to the A* algorithm in graph theory
/// # Arguments
///
/// * `S` - Array of strings.
///
/// # Returns
///
/// * `String` if a Longest Common Subsequence exists
/// * `String::new("")' if no LCS was found
pub fn astar_app(chains: &Vec<&str>) -> String {

    const C: u64 = 20;
    const _K: usize = 2000;

    // Preprocessing
    let mut ctx = Context::new(chains);

    // queueueue
    let mut queue: Vec<Vec<usize>> = vec![];
    init_queue(&mut ctx, &mut queue);

    while !queue.is_empty() {
        // y = max( {f(p) | p in queue} )
        let mut y = f(&ctx, queue.last().unwrap());

        // y = y - c // without overflow
        if y > C {
            y -= C;
        }

        // R = { p | p in queue and y <= f(p) }
        let second_queue = queue
            .clone()
            .into_iter()
            .filter(|p| y <= f(&ctx, p))
            .collect::<Vec<Vec<usize>>>();
        queue.clear();

        for p in second_queue {
            if heuristic(&ctx, &p) == 0 {
                // An MLCS match was found
                return common_seq(&ctx, p);
            } else {
                // inserting all succesors in the queue
                let succs = get_successors(&ctx, &p);
                for q in succs {
                    // basically saying if the queue queue does not already
                    // contain the point q
                    if !queue.contains(&q) {
                        update_suc(&mut ctx, p.clone(), q.clone());
                        queue.push(q);
                    } else if g(&ctx, &q) < g(&ctx, &p) + 1 {
                        update_suc(&mut ctx, p.clone(), q.clone());
                    }
                }
            }
        }
        reorder_queue(&mut ctx, &mut queue);
    }
    String::from("")
}
