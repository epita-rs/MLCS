use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;

const IMPOSSIBLE_NB: usize = 999_999_999_999;

// OPTI considerations
// 1. use min-heap instead of a vec for O(1) access to next successor
// plus O(log(n)) insertion
// 2. Store pointers instead of cloning everything everywhere
// Look at Rc and RefCell

// builds the mt table used for accessing the index of the next char
// builds the common alphabet at the same time
// @params ca is the common alphabet
fn mt_table(chains: &Vec<Vec<char>>, alphabet: &mut Vec<char>) -> Vec<Vec<Vec<usize>>> {
    let mut mt: Vec<Vec<Vec<usize>>> = vec![];

    for ch in alphabet.clone() {
        let mut chain: Vec<Vec<usize>> = vec![];

        for s in chains {
            let mut v: Vec<usize> = vec![IMPOSSIBLE_NB; s.len()];
            let mut lpos = IMPOSSIBLE_NB;

            for i in (0..(s.len())).rev() {
                if s[i] == ch {
                    lpos = i;
                }

                v[i] = lpos;
            }

            chain.push(v);

            if lpos == IMPOSSIBLE_NB {
                alphabet.retain(|&x| x != ch);
                chain = vec![];
                break;
            }
        }

        if !chain.is_empty() {
            mt.push(chain);
        }
    }

    mt
}

//given given 2D coordinates, translates into 1D coordinates
fn translate(i: usize, j: usize, d: usize) -> usize {
    i * d + j
}

// given a point, computes the heuristic function
pub fn heuristic(ctx: &Context, p: &[usize]) -> u64 {
    let mut similarity: Vec<u64> = vec![];
    for i in 0..ctx.d {
        for j in 0..ctx.d {
            if i != j {
                similarity.push(ctx.ms[translate(i, j, ctx.d)][p[i]][p[j]]);
            }
        }
    }

    *similarity.iter().min().unwrap()
}

// retreives the value of f(p)
pub fn f(ctx: &Context, p: &Vec<usize>) -> u64 {
    *ctx.f.get(p).unwrap()
}

// retreives the value of g(p)
pub fn g(ctx: &Context, p: &Vec<usize>) -> u64 {
    *ctx.g.get(p).unwrap()
}

// gets the successors of a specific point
pub fn get_successors(ctx: &Context, p: &[usize]) -> Vec<Vec<usize>> {
    let mut successors: Vec<Vec<usize>> = vec![];

    // for all alphabet letters
    for (ch_idx, _) in ctx.alphabet.iter().enumerate() {
        // for each string, finds the next position of that letter
        let mut succ: Vec<usize> = vec![];
        for i in 0..(ctx.chains.len()) {
            let next_ch_idx = ctx.mt[ch_idx][i][p[i] + 1];
            if next_ch_idx == IMPOSSIBLE_NB {
                break;
            }

            succ.push(next_ch_idx);
        }

        if succ.len() == ctx.chains.len() {
            successors.push(succ);
        }
    }
    successors
}

// given two strings s1 and s2 we compute the score matrix
// @return matrix of size (m + 1) (n + 1)
fn score_matrix(s1: &[char], s2: &[char]) -> Vec<Vec<u64>> {
    let m = s1.len();
    let n = s2.len();
    let mut matrix: Vec<Vec<u64>> = vec![vec![0; n + 1]; m + 1];

    for i in (0..(m - 1)).rev() {
        for j in (0..(n - 1)).rev() {
            matrix[i][j] = if s1[i + 1] == s2[j + 1] {
                matrix[i + 1][j + 1] + 1
            } else {
                max(matrix[i][j + 1], matrix[i + 1][j])
            };
        }
    }

    matrix
}

// given the list of strings we compute the set of score matrices
pub fn matrices_score(chains: &[Vec<char>]) -> Vec<Vec<Vec<u64>>> {
    let mut scores: Vec<Vec<Vec<u64>>> = vec![];
    for s1 in chains.iter() {
        for s2 in chains.iter() {
            scores.push(score_matrix(s1, s2));
        }
    }

    scores
}

// given the list of strings, finds the minimal alphabet
// @detail finds the shortest string
// gets his alphabet
pub fn get_alphabet(chains: &[Vec<char>]) -> Vec<char> {
    // OPTI comment
    // use hashmap to keep track of inserted values
    let mut alphabet: Vec<char> = chains
        .iter()
        .min_by_key(|s| s.len())
        .expect("No minimum found")
        .to_vec();
    alphabet.sort();
    alphabet.dedup();

    alphabet
}

// gets the first matches
pub fn get_starting_p(ctx: &Context) -> Vec<Vec<usize>> {
    // OPTI : we may be passing the alphabet param directly as an iterator
    let mut successors: Vec<Vec<usize>> = vec![];

    // for all alphabet letters
    for (ch_idx, _) in ctx.alphabet.iter().enumerate() {
        // for each string, finds the next position of that letter
        let mut succ: Vec<usize> = vec![];
        for i in 0..(ctx.chains.len()) {
            let next_ch_idx = ctx.mt[ch_idx][i][0];
            if next_ch_idx == IMPOSSIBLE_NB {
                break;
            }

            succ.push(next_ch_idx);
        }

        if succ.len() == ctx.chains.len() {
            successors.push(succ);
        }

    }

    successors
}

// saves all the ctx needed to perform the algo in one place
pub struct Context {
    alphabet: Vec<char>,
    parents: HashMap<Vec<usize>, Option<Vec<usize>>>,
    pub ms: Vec<Vec<Vec<u64>>>,
    mt: Vec<Vec<Vec<usize>>>,
    pub g: HashMap<Vec<usize>, u64>,
    f: HashMap<Vec<usize>, u64>,
    chains: Vec<Vec<char>>,
    pub d: usize,
}

impl Context {
    pub fn new(strings: &[&str]) -> Self {
        // cast to ease [index] accessibily
        let chains: Vec<Vec<char>> = strings.iter().map(|s| s.chars().collect()).collect();
        let d = strings.len();

        // an impossible point, father of all
        let p0 = vec![IMPOSSIBLE_NB; d];

        let ms: Vec<Vec<Vec<u64>>> = matrices_score(&chains);

        let mut parents: HashMap<_, Option<Vec<usize>>> = HashMap::new();
        parents.insert(p0.clone(), None);

        let mut g = HashMap::new();
        g.insert(p0.clone(), 0);

        let mut f: HashMap<Vec<usize>, u64> = HashMap::new();
        f.insert(p0.clone(), 0);

        let mut alphabet: Vec<char> = get_alphabet(&chains);

        let mt = mt_table(&chains, &mut alphabet);

        Context {
            alphabet,
            parents,
            ms,
            mt,
            g,
            f,
            chains,
            d,
        }
    }
}

// runs the successor a first time
// this could be avoided
pub fn init_queue(ctx: &mut Context, queue: &mut Vec<Vec<usize>>) {
    *queue = get_starting_p(ctx);

    for q in queue.clone() {
        update_suc(ctx, vec![IMPOSSIBLE_NB; ctx.d], q.clone());
    }
    reorder_queue(ctx, queue);
}

// given a point p and his successor q, computes necessary informations
// point p is marked PARENT of q
pub fn update_suc(ctx: &mut Context, p: Vec<usize>, q: Vec<usize>) {
    // g(q) = g(p) + 1
    let nb = ctx.g.get(&p).unwrap() + 1;
    ctx.g.insert(q.clone(), nb);
    // saves the cost function for point p : h(p) + g(p)
    ctx.f.insert(q.clone(), heuristic(ctx, &q) + nb);
    // saves the fact that p is the parent of q
    ctx.parents.insert(q.clone(), Some(p));
}

// sorts the queue
pub fn reorder_queue(ctx: &mut Context, queue: &mut [Vec<usize>]) {
    queue.sort_unstable_by(|p, q| {
        if (ctx.f.get(p) > ctx.f.get(q))
            || (ctx.f.get(p) == ctx.f.get(q) && heuristic(ctx, p) > heuristic(ctx, q))
        {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
}

// ascend back up the parent tree to form the common string
pub fn common_seq(ctx: &Context, p: &Vec<usize>) -> String {
    let ref_str: &Vec<char> = &ctx.chains[0];
    let mut common_sequence: Vec<char> = vec![];
    // Gaining mutability
    let mut p = p;

    while ctx.parents.get(p).unwrap().is_some() {
        common_sequence.push(ref_str[p[0]]);

        // getting the parent of current point
        p = ctx.parents.get(p).unwrap().as_ref().unwrap();
    }

    common_sequence.iter().rev().collect::<String>()
}
