use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;

fn h(a:&Vec<u64>) -> u64
{
    // dummy to be later removed
    1
}
// given two strings s1 and s2 we compute the score matrix
fn score_matrix(s1: &str, s2: &str) -> Vec<Vec<u64>>
{
    let n = s1.chars().count();
    let m = s2.chars().count();
    let mut M:Vec<Vec<u64>> = vec![vec![0; n]; m];
    for i in (0..(m - 1)).rev() {
        for j in (0..(n - 1)).rev() {
            M[i][j] = 
            if s1.chars().nth(i + 1).unwrap() == s2.chars().nth(j + 1).unwrap()
            {
                M[i + 1][j + 1] + 1
            }
            else
            {
                max(M[i][j + 1], M[i + 1][j])
            };
        }
    }

    M
}
// given the list of strings we compute the set of score matrices
fn matrices_score(S : &Vec<&str>, d : usize) -> Vec<Vec<Vec<u64>>>
{
       let mut scores: Vec<Vec<Vec<u64>>> = vec![];
       for s1 in S.iter() {
           for s2 in S.iter() {
                scores.push(score_matrix(s1, s2));
           }
       }
       
       scores
}

//given given 2D coordinates, translates into 1D coordinates
fn translate(i: &usize, j: &usize, d: usize) -> usize
{
    i * d + j 
}

// given a point, computes the heuristic function
fn he(M:Vec<Vec<Vec<u64>>>, p:Vec<usize> , d: usize) -> u64
{
    let mut similarity:Vec<u64> = vec![0; d];
    for i in p.iter() {
        for j in p.iter() {
            similarity.push(M[translate(i, j, d)][*i][*j]);
        }
    }

    *similarity.iter().min().unwrap()
}
// We make S to be a ref to Vec instead of a ref 
// to Array due to the possible unknown size of S.
fn mlcs_astar(S : &Vec<&str>, d : usize) -> u64 {

    // "Definitions and Basic Properties"

    // Preprocessing;

    let p0 = vec![0; d];

    let mut parents: HashMap<_, Option<&Vec<u64>>> = HashMap::new();
    parents.insert(&p0, None);

    let mut g = HashMap::new();
    g.insert(&p0, 0);

    let mut f: HashMap<&Vec<u64>, u64> = HashMap::new();
    f.insert(&p0, h(&p0));

    // We make our own PriorityQueue based on a Vec
    // because the priority_queue package is not efficient
    // when we have a particular comparison function
    // to deal with.
    let mut Q = vec![&p0];

    let mut T: HashSet<&Vec<u64>> = HashSet::new();

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
                f: &'s dyn Fn(&CommonSeq, &Vec<u64>) -> u64 
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

    1
}

fn main() {

    let s1 = "Hey world";
    let s2 = "Hello world !";
    let s3 = "Hi world!";

    let S = vec![s1, s2, s3];
    
    mlcs_astar(&S, 3);
}
