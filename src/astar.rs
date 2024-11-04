use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn h(p: &Vec<i32>) -> i32 {
    -1 // TODO.
}

// We make S to be a ref to Vec instead of a ref 
// to Array due to the possible unknown size of S.
pub fn mlcs_astar(S : &Vec<&str>, d : usize) -> i32 {

    // "Definitions and Basic Properties"


    // Preprocessing;

    let p0 = vec![0; d];

    let mut parents: HashMap<_, Option<&Vec<i32>>> = HashMap::new();
    parents.insert(&p0, None);

    let mut g = HashMap::new();
    g.insert(&p0, 0);

    let mut f: HashMap<&Vec<i32>, i32> = HashMap::new();
    f.insert(&p0, h(&p0));

    // We make our own PriorityQueue based on a Vec
    // because the priority_queue package is not efficient
    // when we have a particular comparison function
    // to deal with.
    let mut Q = vec![&p0];

    let mut T: HashSet<&Vec<i32>> = HashSet::new();


    // The .len() Vec method returns a length variable
    // value rather than iterating over the vector.
    while Q.len() > 0 {

        &Q.sort_unstable_by(|p, q| {
            if (f[p] > f[q]) || (f[p] == f[q] && h(p) > h(q)) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let p = &Q.pop().unwrap();

        if h(p) == 0 {

            struct CommonSeq<'s> {
                f: &'s dyn Fn(&CommonSeq, &Vec<i32>) -> i32 
            }
            let tmp = CommonSeq {
                f: &|core, p| {

                    let parent_p = parents.get(p);
                    if parent_p != None && *parent_p.unwrap() != None {
                        (core.f)(core, parent_p.unwrap().unwrap());
                    }

                    // print!(S[0][p]); // TODO
                    
                    // If the algorithm is correct,
                    // then it should never be None.
                    *g.get(p).unwrap()
                }
            };
            return (tmp.f)(&tmp, p);

        }

    }

    -1
}
