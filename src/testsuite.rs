use crate::astar;
use astar::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_get_successors_simple() {
        let s1 = "abcde";
        let s2 = "ace";
        let alphabet = get_alphabet(&vec![s1, s2]);
        let successors = get_successors(&alphabet, &vec![s1, s2], &vec![0, 0]);
        assert!(!successors.is_empty());
        // Successeurs pour 'a'
        assert_eq!(successors[0], vec![2, 1]);
    }

    #[test]
    fn unit_get_successors_no_match() {
        let s1 = "abc";
        let s2 = "xyz";
        let alphabet = get_alphabet(&vec![s1, s2]);
        let successors = get_successors(&alphabet, &vec![s1, s2], &vec![0, 0]);
        assert!(successors.is_empty()); // Pas de correspondances
    }
    /*
    #[test]
    fn test_score_matrix_full_match() {
        let s1 = "abc";
        let s2 = "abc";
        let matrix = score_matrix(s1, s2);

        // La valeur dans [0][0] doit être égale à la longueur de la chaîne
        assert_eq!(matrix[0][0], 3);
    }
    */

//============================

    #[test]
    fn basic_3_1() {
        let s1 = "wowww"; 
        let s2 = "ewwww"; 
        let s3 = "wwhjhkjkjkww"; 
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s, 3), "wwww");
    }
    #[test]
    fn basic_3_2() {
        let s1 = "gxtxayb"; 
        let s2 = "abgtab"; 
        let s3 = "gyaytahjb"; 
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s, 3), "gtab");
    }
    #[test]
    fn basic_3_3() {
        let s1 = "Hey world";
        let s2 = "Hello world !";
        let s3 = "Hi world!";
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s, 3), "H world");
    }
    #[test]
    fn same_3_1() {
        let s1 = "gtab"; 
        let s2 = "gtab"; 
        let s3 = "gtab"; 
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s, 3), "gtab");
    }
    #[test]
    fn same_4_1() {
        let s1 = "Helllo";
        let s2 = "Helllo";
        let s3 = "Helllo";
        let s4 = "Helllo";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(mlcs_astar(&s, s.len()), "Helllo");
    }

}
