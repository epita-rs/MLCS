use std::collections::HashSet;
use std::mem;

// prend a peu pres 30sec sur mon ordi pour une chaine de 23 caracteres.
fn get_all_subsequences(s: &str) -> HashSet<String> {
    let mut subsequences = HashSet::new();
    let n = s.len();
    if (n >= 8 * mem::size_of::<usize>()) {
        panic!("la sous-sequence est trop grande \
               ({n}) pour l'implementation actuelle.");
    }
    
    // Il y a 2^n sous-séquences possibles (y compris la chaîne vide)
    for i in 0..((1 as usize) << n) {
        let mut subsequence = String::new();
        for j in 0..n {
            if (i as usize) & ((1 as usize) << j) != 0 {
                subsequence.push(s.chars().nth(j).unwrap());
            }
        }
        subsequences.insert(subsequence);
    }
    subsequences
}

pub fn mlcs_brute_force(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    // Générer les sous-séquences de la première chaîne
    let mut common_subsequences = get_all_subsequences(strings[0]);

    // println!("sous-sequences 1 trouvees.");

    // Comparer avec les sous-séquences des autres chaînes
    let mut i = 2;
    for &s in &strings[1..] {
        let subsequences_s = get_all_subsequences(s);
        // println!("sous-sequences {i} trouvees.");
        common_subsequences = common_subsequences.intersection(&subsequences_s).cloned().collect();
        i+=1;
    }

    // Trouver la plus longue sous-séquence commune
    common_subsequences.into_iter().max_by_key(|subseq| subseq.len()).unwrap_or_else(|| String::new())
}

fn main() {
    let s1 = "gxtxayb";
    let s2 = "abgtab";
    let s3 = "gyaytahjb";
    let s4 = "gyayjjjtab";
    let s5 = "gyaytahhhhb";
    let s6 = "ygaytppppahjb";
    let s7 = "ylllgaytmmajb";
    let s = vec![s1, s2, s3, s4, s5, s6, s7];

    let mlcs = mlcs_brute_force(&s);
    println!("MLCS: {}", mlcs);
}
