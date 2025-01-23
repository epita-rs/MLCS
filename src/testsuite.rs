use crate::astar::mlcs_astar;
use crate::astar_app::astar_app;
use rand::{seq::SliceRandom, thread_rng, Rng}; // Random number generator

// @brief : shuffles a string
fn _shuffle_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

// @brief generates an random alphabet in the whole unicode range
// TODO keeping printable characters

fn _gen_rand_alphabet(count: usize) -> Vec<char> {
    let mut rng = thread_rng();
    let mut alphabet: Vec<char> = vec![];
    while alphabet.len() < count {
        let ch = char::from_u32(rng.gen_range(61..0x589)).unwrap();
        if !ch.is_control() {
            alphabet.push(ch);
        }
    }
    alphabet
}

// @brief: Outputs {nb} strings of size {length} with their MLCS being {pattern}
// OPTI: pattern could be a Vec<char> in the future
// @remark: The idea is to insert at random positions in every string
// so that the MLCS doesnt change
// @WARNING do not generate over ~100 strings
pub fn generate_testcase(pattern: &str, nb: usize, length: usize) -> Vec<String> {
    let alphab = "abcdefghijklmnopqrstuvwxyz\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ098765432[]#/.,{}~@?<>*!£$%^&*(_+-=)\
    БбВвГгДдЁёЖжЗзИиЙйКкЛлПпФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯяi\
    段包含许多汉字文字字符串用于测试展示中文字符简体繁体标点符号句号逗号问号\
    床前明月光疑是地上霜举头望低思故乡学习编程数据算法计算机网络科技发展未来\
    文化艺术传统创新🚀👩🤶🎅🎄👸🤴🍳🌾🎓🎤💻🔬🎨🚒✈️";
    let mut rng = thread_rng();

    // building an alphabet free from characters in pattern
    let mut alphabet: Vec<char> = alphab
                                  .chars()
                                  .filter(|x| !pattern.contains(*x))
                                  .collect();
    alphabet.shuffle(&mut rng);
    alphabet.truncate(nb);

    let mut res: Vec<String> = vec![];
    // picking a new unique character
    for ch in alphabet {
        let mut new_str: Vec<char> = pattern.chars().collect();
        let mut remaining = length;
        while remaining > 0 {
            // positions is a list of random positions
            let mut positions: Vec<usize> = (0..(new_str.len())).collect();
            positions.shuffle(&mut rng);

            // inserting
            for pos in positions {
                // inserting at rand position the chosen character
                new_str.insert(pos, ch);

                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }

        res.push(new_str.into_iter().collect());
    }
    res
}

#[cfg(test)]
mod functionnal {
    use super::*;

    macro_rules! astar_tests {
        ($($name:ident: $arg:expr,)*) => {
            $(
                #[test]
                 fn $name() {
                 let (pattern, string_nb, length, f) = $arg;
                 let s_string = generate_testcase(&pattern, string_nb, length);
                 // Line below is a basic cast from Vec<String> to Vec<&str>
                 let s = s_string.iter().map(|x| x.as_str()).collect();
                 assert_eq!(f(&s), pattern);
                 }

            )*
        };
    }
    
    macro_rules! astar_complete {
        ($name:expr) => {
            astar_tests! {
            random_4_10: ("grrrrr", 4, 10, $name),
            random_5_15: ("hohoho", 5, 15, $name),
            random_6_20: ("mouimoui", 6, 20, $name),
            random_7_30: ("999776543", 7, 30, $name),
            random_20_40: ("mouahahahahahahahihihihohoho", 20, 40, $name),
            random_60_60: ("jj998762bk--_-=-^%$£..mnHGb##", 60, 60, $name),
            random_70_40: ("j8762bk-f_u=-^%$£i.mnHGb#?", 70, 40, $name),
            random_50_100: ("j8762bk-f_u=-^%$£i.mnHGb#?", 50, 100, $name),
            /*
            random_70_1050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 70, 1050, $name),
            random_30_5050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 30, 5050, $name),
            random_20_10050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 20, 10_050, $name),
            random_20_50050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 20, 50_050, $name),
            random_20_500050: ("j8762bk-f_goulou][", 20, 500_050, $name),
            random_20_2_000_050: ("j0987654321-f_gou][", 20, 2_000_050, $name),
            */
            }
        };
    }   

    astar_complete!(mlcs_astar);
    
    #[test]
    fn basic_3_1() {
        let s1 = "wowww";
        let s2 = "ewwww";
        let s3 = "wwhjhkjkjkww";
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s), "wwww");
    }
    #[test]
    fn basic_3_2() {
        let s1 = "gxtxayb";
        let s2 = "abgtab";
        let s3 = "gyaytahjb";
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s), "gtab");
    }
    #[test]
    fn basic_3_3() {
        let s1 = "Hey world";
        let s2 = "Hello world !";
        let s3 = "Hi world!";
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s), "H world");
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
        assert_eq!(mlcs_astar(&s), "gtab");
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
        assert_eq!(mlcs_astar(&s), "gt#ab");
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
        assert_eq!(mlcs_astar(&s), "gtab");
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
        assert_eq!(mlcs_astar(&s), "gtab000-");
    }
    #[test]
    fn same_3_1() {
        let s1 = "gtab";
        let s2 = "gtab";
        let s3 = "gtab";
        let s = vec![s1, s2, s3];
        assert_eq!(mlcs_astar(&s), "gtab");
    }
    #[test]
    fn same_4_1() {
        let s1 = "Helllo";
        let s2 = "Helllo";
        let s3 = "Helllo";
        let s4 = "Helllo";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(mlcs_astar(&s), "Helllo");
    }
    #[test]
    fn no_match_4_1() {
        let s1 = "rtyui";
        let s2 = "Helllo";
        let s3 = "GGGGGGG";
        let s4 = "PRRRRRRRRRRR";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(mlcs_astar(&s), "");
    }
    #[test]
    fn no_match_4_2() {
        let s1 = "PJVGCVHJBJKBJBK";
        let s2 = "kdgdhdhfhsh";
        let s3 = "0987654567898765";
        let s4 = ")(*&^&*()(*&^";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(mlcs_astar(&s), "");
    }
}
#[cfg(test)]
mod astar_app {
    use super::*;

    macro_rules! astar_tests {
        ($($name:ident: $arg:expr,)*) => {
            $(
                #[test]
                 fn $name() {
                 let (pattern, string_nb, length, f) = $arg;
                 let s_string = generate_testcase(&pattern, string_nb, length);
                 // Line below is a basic cast from Vec<String> to Vec<&str>
                 let s = s_string.iter().map(|x| x.as_str()).collect();
                 assert_eq!(f(&s), pattern);
                 }

            )*
        };
    }
    
    macro_rules! astar_complete {
        ($name:expr) => {
            astar_tests! {
            random_4_10: ("grrrrr", 4, 10, $name),
            random_5_15: ("hohoho", 5, 15, $name),
            random_6_20: ("mouimoui", 6, 20, $name),
            random_7_30: ("999776543", 7, 30, $name),
            random_20_40: ("mouahahahahahahahihihihohoho", 20, 40, $name),
            random_60_60: ("jj998762bk--_-=-^%$£..mnHGb##", 60, 60, $name),
            random_70_40: ("j8762bk-f_u=-^%$£i.mnHGb#?", 70, 40, $name),
            random_50_100: ("j8762bk-f_u=-^%$£i.mnHGb#?", 50, 100, $name),
            /*
            random_70_1050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 70, 1050, $name),
            random_30_5050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 30, 5050, $name),
            random_20_10050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 20, 10_050, $name),
            random_20_50050: ("j8762bk-f_u=-^%$£i.mnHGb#?", 20, 50_050, $name),
            random_20_500050: ("j8762bk-f_goulou][", 20, 500_050, $name),
            random_20_2_000_050: ("j0987654321-f_gou][", 20, 2_000_050, $name),
            */
            }
        };
    }   

    astar_complete!(astar_app);
    

    #[test]
    fn basic_3_1() {
        let s1 = "wowww";
        let s2 = "ewwww";
        let s3 = "wwhjhkjkjkww";
        let s = vec![s1, s2, s3];
        assert_eq!(astar_app(&s), "wwww");
    }
    #[test]
    fn basic_3_2() {
        let s1 = "gxtxayb";
        let s2 = "abgtab";
        let s3 = "gyaytahjb";
        let s = vec![s1, s2, s3];
        assert_eq!(astar_app(&s), "gtab");
    }
    #[test]
    fn basic_3_3() {
        let s1 = "Hey world";
        let s2 = "Hello world !";
        let s3 = "Hi world!";
        let s = vec![s1, s2, s3];
        assert_eq!(astar_app(&s), "H world");
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
        assert_eq!(astar_app(&s), "gtab");
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
        assert_eq!(astar_app(&s), "gt#ab");
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
        assert_eq!(astar_app(&s), "gtab");
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
        assert_eq!(astar_app(&s), "gtab000-");
    }
    #[test]
    fn same_3_1() {
        let s1 = "gtab";
        let s2 = "gtab";
        let s3 = "gtab";
        let s = vec![s1, s2, s3];
        assert_eq!(astar_app(&s), "gtab");
    }
    #[test]
    fn same_4_1() {
        let s1 = "Helllo";
        let s2 = "Helllo";
        let s3 = "Helllo";
        let s4 = "Helllo";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(astar_app(&s), "Helllo");
    }
    #[test]
    fn no_match_4_1() {
        let s1 = "rtyui";
        let s2 = "Helllo";
        let s3 = "GGGGGGG";
        let s4 = "PRRRRRRRRRRR";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(astar_app(&s), "");
    }
    #[test]
    fn no_match_4_2() {
        let s1 = "PJVGCVHJBJKBJBK";
        let s2 = "kdgdhdhfhsh";
        let s3 = "0987654567898765";
        let s4 = ")(*&^&*()(*&^";
        let s = vec![s1, s2, s3, s4];
        assert_eq!(astar_app(&s), "");
    }
}
