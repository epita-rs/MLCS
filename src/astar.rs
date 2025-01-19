use crate::utils::*;

// We make S to be a ref to Vec instead of a ref 
// to Array due to the possible unknown size of S.
pub fn mlcs_astar(S : &Vec<&str>, d : usize) -> String {

    // Preprocessing
    let mut infos = Infos::new(S, d);

    // Queue
    let mut Q:Vec<Vec<usize>> = vec![];
    init_queue(&mut Q, S, d, &mut infos);

    while Q.len() > 0 {

        let p:Vec<usize> = Q.pop().unwrap().clone();

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
                update_suc(p.clone(), q.clone(), &mut infos);
                if !Q.contains(&q) {
                    Q.push(q);
                }
            }

            reorder_queue(&mut Q, &mut infos);
        }

    }
    return String::from("");
}
