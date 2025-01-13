use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
use std::rc::Rc;

const IMPOSSIBLE_NB:usize = 999_999_999_999;

// OPTI considerations
// 1. use min-heap instead of a vec for O(1) access to next successor
// plus O(log(n)) insertion
// 2. precompute indexes using MT table
// 3. Store pointers instead of cloning everything everywhere
// Look at Rc and RefCell
// 4. Build a common alphabet 
// because no need to look for matchs that do not exists

// builds the MT table used for accessing the index of the next char
fn MT_table(alphabet:&Vec<char>, S:&Vec<&str>) -> Vec<Vec<Vec<usize>>>
{
    let sc:Vec<Vec<char>> = S.iter().map(|x| x.chars().collect()).collect();

    let mut MT:Vec<Vec<Vec<usize>>> = vec![];

    for s in sc {
        let mut chain:Vec<Vec<usize>> = vec![];

        for ch in alphabet {
           let mut V:Vec<usize> = vec![IMPOSSIBLE_NB; s.len()];
           let mut lpos = IMPOSSIBLE_NB; 

           for i in (0..(s.len())).rev() {
                if s[i] == *ch {
                    lpos = i;
                }

                V[i] = lpos;
           }

           chain.push(V);
        }

        MT.push(chain);
    }
    
    MT
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

// gets the successors of a specific point
pub fn get_successors(infos: &Infos, S : &Vec<&str>, p: &Vec<usize>) 
    -> Vec<Vec<usize>>
{
    // OPTI : we may be passing the alphabet param directly as an iterator
    let mut successors:Vec<Vec<usize>> = vec![];
    let mut ch_idx:usize = 0;

    // for all alphabet letters
    for ch in infos.alphabet.iter()
    {
        // for each string, finds the next position of that letter
        let mut succ:Vec<usize> = vec![]; 
        for i in 0..(S.len())
        {
            let next_ch_idx = infos.MT[i][ch_idx][p[i] + 1];
            if next_ch_idx == IMPOSSIBLE_NB
            {
                break;
            }

            succ.push(next_ch_idx);
        }

        if succ.len() == S.len()
        {
            successors.push(succ);
        }

        ch_idx += 1;
    }
    successors
}

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

// gets the first matches 
pub fn get_starting_p(infos: &Infos, S: &Vec<&str>) -> Vec<Vec<usize>>
{
    // OPTI : we may be passing the alphabet param directly as an iterator
    let mut successors:Vec<Vec<usize>> = vec![];
    let mut ch_idx:usize = 0;

    // for all alphabet letters
    for ch in infos.alphabet.iter()
    {
        // for each string, finds the next position of that letter
        let mut succ:Vec<usize> = vec![]; 
        for i in 0..(S.len())
        {
            let next_ch_idx = infos.MT[i][ch_idx][0];
            if next_ch_idx == IMPOSSIBLE_NB
            {
                break;
            }

            succ.push(next_ch_idx);
        }

        if succ.len() == S.len()
        {
            successors.push(succ);
        }

        ch_idx += 1;
    }

    successors
}

// given the list of strings, finds their common alphabet
pub fn get_common_alphabet(S : &Vec<Vec<char>>) -> Vec<char>
{
    // TODO get first line, collect the rest as hashmaps
    //let mut alphabet:HashMap<char, bool> = S[-1];

    vec![]
}

// saves all the infos needed to perform the algo in one place
pub struct Infos {
alphabet : Vec<char>,
         parents : HashMap<Vec<usize>, Option<Vec<usize>>>,
         MS : Vec<Vec<Vec<u64>>>,
         MT : Vec<Vec<Vec<usize>>>,
         g : HashMap<Vec<usize>, u64>,
         f : HashMap<Vec<usize>, u64>,
         d : usize
}
impl Infos {
    // basic preprocessing
    pub fn new(S : &Vec<&str>, d : usize) -> Self
    {
        let p0 = vec![IMPOSSIBLE_NB; d];

        let alphabet:Vec<char> = get_alphabet(S);

        let MS:Vec<Vec<Vec<u64>>> = matrices_score(S);

        let mut parents : HashMap<_, Option<Vec<usize>>> = HashMap::new();
        parents.insert(p0.clone(), None);

        let mut g = HashMap::new();
        g.insert(p0.clone(), 0);

        let mut f: HashMap<Vec<usize>, u64> = HashMap::new();
        f.insert(p0.clone(), 0);
        
        let MT = MT_table(&alphabet, &S);

        return Infos { alphabet, parents, MS, MT, g, f, d};
    }
}

// runs the successor a first time
// this could be avoided
fn init_queue(Q: &mut Vec<Vec<usize>>, S: &Vec<&str>, d:usize, infos:&mut Infos)
{
    *Q = get_starting_p(&infos, &S);

    for q in Q.clone() {
        update_suc(vec![IMPOSSIBLE_NB; d], q.clone(), infos);
    }
    reorder_queue(Q, infos);
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

// sorts the queue
fn reorder_queue(Q: &mut Vec<Vec<usize>>, i: &mut Infos)
{
    Q.sort_unstable_by(|p, q| {
            if (i.f.get(p) > i.f.get(q)) || (i.f.get(p) == i.f.get(q) 
                   && h(&i.MS, p, i.d) > h(&i.MS, q, i.d)) {
                    Ordering::Greater
            }
            else {
                Ordering::Less
            }
  });
}

fn is_match(P: &Vec<usize>, S: &Vec<&str>) -> bool
{
    let v:Vec<char> = S.iter().map(|s| s.chars().nth(0).unwrap()).collect();
    let first = v[0];
    
    v.iter().all(|c| *c == first)
}

// ascend back up the parent tree to form the string
fn common_seq(i :&Infos, p: &Vec<usize>, S: &Vec<&str>) -> String
{
    let ref_str:Vec<char> = S[0].chars().collect();
    let mut s:Vec<char> = vec![];
    // Gaining mutability
    let mut p = p;

    while *i.parents.get(p).unwrap() != None {
        s.push(ref_str[p[0]]);

        // getting the parent of current point
        p = &i.parents.get(p).unwrap().as_ref().unwrap(); 
    }

    s.iter().rev().collect::<String>()
}

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
