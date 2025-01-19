use crate::utils::*;

const k:usize = 2000;
const c:u64 = 20;

// We make S to be a ref to Vec instead of a ref 
// to Array due to the possible unknown size of S.
pub fn astar_app(S : &Vec<&str>, d : usize) -> String {

    // Preprocessing
    let mut infos = Infos::new(S, d);

    // Queue
    let mut Q:Vec<Vec<usize>> = vec![];
    init_queue(&mut Q, S, d, &mut infos);

    while Q.len() > 0 {

        // y = max( {f(p) | p in Q} )
        let mut y = f(&infos, Q.last().unwrap());

        // y = y - c // without overflow
        if y > c {
            y -= c;
        }

        // R = { p | p in Q and y <= f(p) }
        let R = Q.clone().into_iter()
                         .filter(|p| y <= f(&infos, p))
                         .collect::<Vec<Vec<usize>>>();
        Q.clear();

        for p in R {
            if h(&infos.MS, &p, d) == 0 {
                // An MLCS match was found
                return common_seq(&infos, &p, S);
            }
            else
            {
                // inserting all succesors in the queue
                let succs = get_successors(&infos, &S, &p);
                for q in succs {
                    // basically saying if the queue Q does not already 
                    // contain the point q
                    if !Q.contains(&q) {
                        update_suc(p.clone(), q.clone(), &mut infos);
                        Q.push(q);
                    }
                    else if g(&infos, &q) < g(&infos, &p) + 1
                    {
                        update_suc(p.clone(), q.clone(), &mut infos);
                    }
                }
            }
        }
        reorder_queue(&mut Q, &mut infos);
    }
    return String::from("");
}
