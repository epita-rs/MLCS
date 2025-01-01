use crate::astar;
use astar::*;

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn get_successors_simple() {
        let s1 = "abcde";
        let s2 = "ace";
        let alphabet = get_alphabet(&vec![s1, s2]);
        let successors = get_successors(&alphabet, &vec![s1, s2], &vec![0, 0]);
        assert!(!successors.is_empty());
        // Successeurs pour 'a'
        assert_eq!(successors[0], vec![2, 1]);
    }
    #[test]
    fn get_successors_no_match() {
        let s1 = "abc";
        let s2 = "xyz";
        let alphabet = get_alphabet(&vec![s1, s2]);
        let successors = get_successors(&alphabet, &vec![s1, s2], &vec![0, 0]);
        assert!(successors.is_empty()); // Pas de correspondances
    }
    #[test]
    fn alphabet_2_1() {
        let s1 = "abcdddddeee";
        let s2 = "aaaaaffgghijjjkllll";
        let alphabet = get_alphabet(&vec![s1, s2]);
        let res:String = alphabet.into_iter().collect();
        assert_eq!(res, "abcdefghijkl");
    }
    #[test]
    fn alphabet_3_1() {
        let s1 = "abcdddddeee";
        let s2 = "aaaaaffgghijjjkllll";
        let s3 = "aaaaaffgghijjjk####!llll";
        let alphabet = get_alphabet(&vec![s1, s2, s3]);
        let res:String = alphabet.into_iter().collect();
        assert_eq!(res, "abcdefghijkl#!");
    }
    #[test]
    fn alphabet_5_1() {
        let s1 = "abcdddddeee";
        let s2 = "aaaaaffgghijjjkllll";
        let s3 = "aaaaadddddeeefgghijjjkllllmno";
        let s4 = "aaaaaffgghijjjkllllpq";
        let s5 = "aaaaaffgghijjjkllllrs";
        let alphabet = get_alphabet(&vec![s1, s2, s3, s4, s5]);
        let res:String = alphabet.into_iter().collect();
        assert_eq!(res, "abcdefghijklmnopqrs");
    }

}

#[cfg(test)]
mod functionnal {
    use super::*;

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
    fn basic_7_1() {
        let s1 = "gxtxayb"; 
        let s2 = "abgtab"; 
        let s3 = "gyaytahjb"; 
        let s4 = "gyayjjjtab"; 
        let s5 = "gyaytahhhhb"; 
        let s6 = "ygaytppppahjb"; 
        let s7 = "ylllgaytmmajb"; 
        let s = vec![s1, s2, s3, s4, s5, s6, s7];
        assert_eq!(mlcs_astar(&s, 7), "gtab");
    }
    #[test]
    fn basic_7_2() {
        let s1 = "gxt#xayb"; 
        let s2 = "abgt#ab"; 
        let s3 = "gyayt#ahjb"; 
        let s4 = "gyayjjjt#ab"; 
        let s5 = "gyayt#ahhhhb"; 
        let s6 = "ygaytp#pppahjb"; 
        let s7 = "ylllgaytm#####majb"; 
        let s = vec![s1, s2, s3, s4, s5, s6, s7];
        assert_eq!(mlcs_astar(&s, 7), "gt#ab");
    }
    #[test]
    fn medium_12_1() {
        let s1 = "gxtxayb-000000000===++"; 
        let s2 = "abgtab"; 
        let s3 = "gyaytahjb"; 
        let s4 = "gyayjjjta88b"; 
        let s5 = "gyaytah3hhhb"; 
        let s6 = "ygaytppppahjb"; 
        let s7 = "y##hga3ytmakkbkk"; 
        let s8 = "ylllgannnnn89001tajbpppp"; 
        let s9 = "ylllg3aytmsmaj212121ffb"; 
        let s10 = "ylllgasetytmmlklajb"; 
        let s11 = "yll5lg25533ayedshtmlmjjaljb"; 
        let s12 = "ylll2ga2ytj345kmmajb"; 
        let s = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12];
        assert_eq!(mlcs_astar(&s, s.len()), "gtab");
    }
    #[test]
    fn medium_12_2() {
        let s1 = "gxtxayb-000000000===++-"; 
        let s2 = "abgtab00kkkkkk0-"; 
        let s3 = "gyaytahj23456766005550-b090909090909909090++=-"; 
        let s4 = "gyayjjjta88b000-"; 
        let s5 = "gyaytah3hhhb651250123_0__0;;-"; 
        let s6 = "ygaytppppahjbllll0000-"; 
        let s7 = "y##hga3ytmakkbkk000-"; 
        let s8 = "ylllgannnnn89001tajb###!!pppp0#0#0-"; 
        let s9 = "ylllg3aytmsmaj21212b1f0[0[0-f"; 
        let s10 = "ylllgasetytmmlklajb,,,0.0.0-"; 
        let s11 = "yll5lg25533ayedshtmlmjjaljb0.0.0.0.0.0..0-"; 
        let s12 = "ylll2ga2ytj345kmmajb000-"; 
        let s = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12];
        assert_eq!(mlcs_astar(&s, s.len()), "gtab000-");
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
