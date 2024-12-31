use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
use std::rc::Rc;

// OPTI considerations
// 1. use min-heap instead of a vec for O(1) access to next successor
// plus O(log(n)) insertion
// 2. precompute indexes using MT table
// 3. Store pointers instead of cloning everything everywhere
// Look at Rc and RefCell

// given two strings s1 and s2 we compute the score matrix
pub fn score_matrix(s1: &str, s2: &str) -> Vec<Vec<u64>>
{
    let m = s1.chars().count();
    let n = s2.chars().count();
    let mut M:Vec<Vec<u64>> = vec![vec![0; n + 1]; m + 1];
    for i in (0..(m - 1)).rev() {
        for j in (0..(n - 1)).rev() {
            M[i][j] = 
                // not efficient line : O(n) for 2 access
                // to be reviewed later on
                if s1.chars().nth(i + 1).unwrap() == s2.chars().nth(j + 1).unwrap()
                    //=============================================================
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
pub fn matrices_score(S : &Vec<&str>) -> Vec<Vec<Vec<u64>>>
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
fn translate(i: usize, j: usize, d: usize) -> usize
{
    i * d + j 
}

// given a point, computes the heuristic function
fn h(M:&Vec<Vec<Vec<u64>>>, p:&Vec<usize> , d: usize) -> u64
{
    let mut similarity:Vec<u64> = vec![];
    for i in 0..d {
        for j in 0..d {
            if i != j {
                similarity.push(M[translate(i, j, d)][p[i]][p[j]]);
            }
        }
    }

    *similarity.iter().min().unwrap()
}

// given the list of strings, finds the alphabet
pub fn get_alphabet(S : &Vec<&str>) -> Vec<char>
{
    // OPTI comment
    // use hashmap to keep track of inserted values
    let mut alphabet:Vec<char> = vec![]; 
    for s in S.iter()
    {
        for ch in s.chars()
        {
            // line running in O(n)
            if !alphabet.contains(&ch)
                // =======================
            {
                alphabet.push(ch);
            }
        }
    }

    alphabet
}
/*
   fn find_next_match(ch:char, s: &str, start_pos:usize, size:usize) -> usize
   {
// TODO   
}
 */
// gets the successors of a specific point
pub fn get_successors(alphabet : &Vec<char>, S : &Vec<&str>, p: &Vec<usize>) 
    -> Vec<Vec<usize>>
{
    // OPTI : we may be passing the alphabet param directly as an iterator
    let mut successors:Vec<Vec<usize>> = vec![];
    // for all alphabet letters
    for ch in alphabet.iter()
    {
        let mut i = 0;
        let mut succ:Vec<usize> = vec![]; 
        // for each string, finds the next position of that letter
        for s in S.iter()
        {
            // starting the search at the current point index + 1
            let mut j = p[i] + 1;
            let n = s.chars().count();
            // line below is ridiculous, O(n) for each access
            while j < n && s.chars().nth(j).unwrap() != *ch
                //================================================
            {
                j += 1;
            }
            if j < n {
                succ.push(j);
            }
            else
            {
                // discard the letter if its absent from any string
                break; 
            }
            i += 1;
        }
        if succ.len() == S.len()
        {
            successors.push(succ);
        }
    }
    successors
}
// saves all the infos needed to perform the algo in one place
pub struct Infos {
alphabet : Vec<char>,
         parents : HashMap<Vec<usize>, Option<Vec<usize>>>,
         MS : Vec<Vec<Vec<u64>>>,
         g : HashMap<Vec<usize>, u64>,
         f : HashMap<Vec<usize>, u64>,
         d : usize
}
impl Infos {
    // basic preprocessing
    pub fn new(S : &Vec<&str>, d : usize) -> Self
    {
        let p0 = vec![0; d];

        let alphabet:Vec<char> = get_alphabet(S);

        let MS:Vec<Vec<Vec<u64>>> = matrices_score(S);

        let mut parents : HashMap<_, Option<Vec<usize>>> = HashMap::new();
        parents.insert(p0.clone(), None);

        let mut g = HashMap::new();
        g.insert(p0.clone(), 0);

        let mut f: HashMap<Vec<usize>, u64> = HashMap::new();
        f.insert(p0.clone(), h(&MS, &p0, d));

        return Infos { alphabet, parents, MS, g, f, d};
    }
}
// given a point p and his successor q, computes necessary informations
fn update_suc(p:Vec<usize>, q:Vec<usize>, infos: &mut Infos)
{
    // g(q) = g(p) + 1
    let nb = infos.g.get(&p).unwrap() + 1;
    infos.g.insert(q.clone(), nb);
    // saves the cost function for point p : h(p) + g(p)
    infos.f.insert(q.clone(), h(&infos.MS, &q, infos.d) + nb);
    // saves the fact that p is the parent of q
    infos.parents.insert(q.clone(), Some(p));
}

fn reorder_queue(Q: &mut Vec<Vec<usize>>, i: &mut Infos)
{
    Q.sort_unstable_by(|p, q| {
            if (i.f.get(p) > i.f.get(q)) || (i.f.get(p) == i.f.get(q) 
                   && h(&i.MS, p, i.d) > h(&i.MS, q, i.d)) {
            Ordering::Greater
            } else {
            Ordering::Less
            }
            });
}
// ascend back up the parent tree to form the string
fn common_seq(i :&Infos, p: &Vec<usize>, S: &Vec<&str>) -> String
{
    let mut s = String::from(S[0].chars().nth(p[0]).unwrap());
    let mut p = p;
    while *i.parents.get(p).unwrap() != None {
        p = &i.parents.get(p).unwrap().as_ref().unwrap(); 
        // Inefficient line again O(n)
        s.push(S[0].chars().nth(p[0]).unwrap());
        // ============================
    }
    s.chars().rev().collect::<String>()
}

//Common seq
// TODO please document here
/*
   struct CommonSeq<'s> {
f: &'s dyn Fn(&CommonSeq, &Vec<usize>) -> u64 
}
let tmp = CommonSeq {
f: &|core, p| {

let parent_p = infos.parents.get(p);
if parent_p != None && *parent_p.unwrap() != None {
(core.f)(core, &parent_p.unwrap().clone().unwrap());
}

// print!(S[0][p]); 

// If the algorithm is correct,
// then it should never be None.
 *infos.g.get(p).unwrap()
 }
 };
 return (tmp.f)(&tmp, &p);
 */
//============================================================================

// We make S to be a ref to Vec instead of a ref 
// to Array due to the possible unknown size of S.
pub fn mlcs_astar(S : &Vec<&str>, d : usize) -> String {

    // "Definitions and Basic Properties"

    // Preprocessing;
    let mut infos = Infos::new(S, d);

    let p0 = vec![0; d];

    // We make our own PriorityQueue based on a Vec
    // because the priority_queue package is not efficient
    // when we have a particular comparison function
    // to deal with.
    let mut Q:Vec<Vec<usize>> = vec![p0];

    // The .len() Vec method returns a length variable
    // value rather than iterating over the vector.
    while Q.len() > 0 {

        let p:Vec<usize> = Q.pop().unwrap().clone();

        if h(&infos.MS, &p, d) == 0 {
            return common_seq(&infos, &p, S);
        }
        else
        {
            // inserting all succesors in the queue
            let succs = get_successors(&infos.alphabet, &S, &p);
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
    return String::from("Nothing was found!");
}
