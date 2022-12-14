#![feature(associated_type_bounds)]
use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Windows;
use std::slice::Iter;
const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const PUZZLE_INPUT: &str = "cvtvbvfbvfbfqfrrtnnmqqmcqqpfplfpphchpcpcfppmwwhhscccjwjnnctcjtjgjbgjjdzjjwzwfffnhhgppjmpmgppppzbzbrzznttfqqhtqqbssrjjffgssnqnsqnsqnnmlmmztmtqmqccqscsshrsrtstrrjwrrswwpnnftfwwwzznsstbbbtzzncnbcnbccwtwctwwjgjcjtccblclgcgjgngwgbggpbbbbbgcczzfsfbbwlwwnqwnwbnwnmmbjjnfnqfqfvvgvqqvcvggpnpssptpnpjjvvpdpqpcqpcqqnhhnqncqclcfcfjcjhjdjqddhndnmmvmfmsmdsdwsdswsnsgsmsrmrjjpzzzzcppczcscsgcczqcqnnpzpbbdqqqqsqlslzljzjdjbjdjvvlhvvnbbjtbbqbdqdggzwgghhdssmttjtttmwtmtjjcpjpjptjpjmpptnntgttjpttljjwtjwwrgrvgrvrmvvrvdrdcdmmvvbpbddfbdbppdccjdjvjpjjgzjzdzfddqcqlccbvbqvqvjqjggqrqdrqrhhwggmvgmgvgrvvzwzddlzlbbdpbdbtbhthnthttjwtwffwvvrccnpcnclcgcrrgdrrgzgmzggzhhftfppnwppqhhmddzhdzzzjqjhqqcgqqmjjwppfrprlrjlrjjlhjlhjjnbnlblbzlbbfjjwfjjdcdndvvbllfmlmfmvmnnlcnncsnslsvlsvlsltslslqsqzssslqlssqrrpdpwdppsfpsprpffchhcvcpvvjpjvjmvvrjrqrzqqqlvvzzggcgrcgcmmvnmmchmmswsvwsvwswnwswdwmwzmmqfqrqtrqqwqlqlmqmsmggpzpjjlvllzplzlbzzbhhglhhwwhfwwpwgwrwlwplppfvflftltffccmcrcgrcgghgmmbrbppwvvghvhphhztzpzzwbbtpppcnpccnvcvbccvfvmffmtmbttdqdrrfnrnffsgstttbhhgmmtffhthllsbllsqsllptpwpvpdpsddsgsrrngnpggwbwwppfqfmqfmfvfzzzldltdtvdvtvtbvtgpvtnvcsdbwjrqwrfzdhwfwnmtrltdhsfzwwfpscrlsffchnfszqdddsztwzcbwdwfnljpfdsqgpnjffqfgsprggfwgvfldvzmnrpnzvfscswnzbljtvshzmcztqqwtzdstsqggqqvvlmspjjcmllcmsndwpllrvlrglphcqjgwzntwqffthrfmfgpcvjvclvdrjrpnwtrlshlnwgbbznpflsfmgvbwmnwjpnqhmdnsmwnmblqzjfzfcslhntnjhbrvvlfbffsbhrtvjltqrqszsrpmdghmhsrqlmrjdsgtsgwggdlqptqrvgmwrpcjrnmmmcdpgjzgghqmshhmddfvfrqwnwvwbvzzpmwzmvvgsqlsgvmtczwnbzrtnzjnpjjghdzsmcgvqpdhltvltbfvgsgzhvqjjnpspngqpvqwvtqrczqzwrclqbnplmvlnvcwdbzlrnwbppvdfzmblnqfpfqlngttrrcjzfjnrnmssdrrvbvhtrfzblzsjbqwbttsshmcmfhrfbfqspvcfmqbwjszmvwqnpszcfhtnqtftvtpvcjrcnlmfnvlpjzspcnmvvshpsdfhtbhgjcmjmtpmtcbwnlgtnvlwlggbnzfrrqljmjrwjtzfmgbllmmjtfhrszfrfpftmpzhzdhfwdbdnwqspjbrlglgdwtfzcrdhwgqltvthbnrqfngjgnddqlcpjflqzvbhmlzsvpvlzdvhspmchrjlzfgnlpwdnszqdqpnrchdrswfwnflslcpfmbfrplsmchbgjhhhlzjlvztpdhslrpmmcthhjwfczjtmbmsvzvsqrvzrcnflqldfnhjzlcsfhgrtqtscnbzqjglgjnqbtprrprrdtqqtlqwdqnmgbglvwtrpnpjqdsmvczbnldvmgmtpmmqbwgjbfsqbjvnjhhlrppptlrjsptpvghbgtvwvsrnggznjsbjpnwpdsqmmjjqjdwshqwzvqgpnlhdldwzmvmdnhrnhpscrbnbdtmfqwlmrlszljpjdqgjpgcdwtqzsbqsndjdslrstjwbbfnjvrjlbrphqspltjzpvsgccfrmqhfhjmlslncpgpflhjlrzqsffrtcgsvzpjmlqjznscgjrjhzfjbbchvgjgvfsrpnzdrstdtsltqldfldvfdjqdwsltcbnnvcgvnjpgdjzsbzgfvgvtmghwfmrmblscfdqlsdgcdbgsphfvvvnffcqwlrvnsrjmvvdrjdljlppsfsrmwwvmsqcgtgtfjhqbqvlsnwdwzhncgprbspjzqnbllwcjnqpfzlhscdtvlldffngwttntsbtlctdfbzjgclgbhhjtlffvtgbdsswczlgmwntrrnnbdtflplfbtqcfrhtgjnrltvzqlcvtdthldqrvjdmlntdjzcncrplgmdbqhlrrfdszrnbrllhgpzzgjnqzrjrtjjntqvdphgvjtbtqfjwvlpprrcvmpjcdmntvtqjvjrhngbtmdtflpqmlmmvjjjpdlcvntlvlgstbblwfwjrbcqjgzvjjzflfhdvgnhpnqprcvmhsrpmgfcgjpdbqpmpvlnmzdngzqcqtdsbbqbdhtgpqmqwtcctsvhwmlhpgzgpwwsqngsrwzghfhzbqcpspfhqbllbvpwgwdnghqnvjtfqtwgvnbfbpcffbwwbfqgqnftpqdtwhmrprpgdlnwrcmdndllwbvlrrjvmwlbgfwgbhtmbqhfvhlhlthbrljwnwcgsdgsbbzgnwzmjfgnfjcqhspjwctntznjptsnbpwtfzmrqwjzrmdthfwdvttrcghbjndfmtswhsmdwwwffmgsvshmlhdbwdscjnvrvqnwttlsmfbssnsscftcrpwfbbplbpmvzmttvbgpnjtqqzcfdfwqdphggwbnhfvssmdjvbvtlrcghltthqrqdbvjnbqpzjcggrbnrswhvdcrmgncsmhsmstmcbwgbfwvzqtdrpjbgljdgcncgcdslhftptnvfhgpsjqqmnsjmqhztnfzqqqhccbnqjlhqcwwbfzgpmbvjszswjlhcqshtqdmwmhpgqgzbwbtchrdwgnvfqcgcmngtddpzfgjrrssdmcnbtpbhfvfbdgzmcbjqtdqrnrdchtrcfpmtnvfczdlnjplwccfmzjwwvdsrcjznrcbczdqwshjgvhjpgdgjvtvmdzncrqgffzhcbwstvltqbrzfbnrrvlntjmzctvnqhdcgvvmmbfplzpwgsdzdtvnwnshnwbwqplbctntslczwhblsnzdcsgltvlfvbtvthvlsmzmfgzhpvrvczdhmcpmgqzbdhbcdbstrrtzgrbhhwwghzlgcdtlcjtjdcwdtqpvpwzrwmhczfsdmwjlgtmzvdbwzdnsztvqzhjpfrqzllgvrsbmhvrlzwtfmpzchftrzlmndpwqgcmhdzmjrjvpbnrcmjvzbzlqngmrzcbdwfcwmlnlqsdgbvmvqcsjtwgvtpmqdtdqfrshmlgnjqdwsrpjcfnfpjzlhslsvppmgprqlpcfztcngmqgrvqvpcszzgtbpcrghmgnvgplctgdljhwjgbzmjrdsccdtrwjrrbttzmpvmzlmqdfwdccfzqztdjmcgrcdwghgmmntdwrclshlgnsmhmltwcczncndtglnqvcvbwlvcgqtzpnqnlllchwjwrlhcdlbjlzvhvtjsttqhvhfqbsqjpfcmpsfshgrrwgprjtfnhsjqqntrjfgjjmjbqwdjlcjjdljpppgvqvgrrwvclbmncmqglhbtjcfrbjmhqgmlpzgljntqvrcggmrgtbrftvhrnjbvzstlnzrqcgncnjhdrnqcnwcrstvdsgnwzjzmqsdtwzzjmfgzpslhqclhtjnrlwwfrqgmhsttfdprhgmphzfdfqntwztltnnfqwgdnnbrlgzmtwqsvhbjjqtcgmghnbchdfgfrdtjnvlgtqdgrfwmgnmpslqmqzdgqztnppbc";
fn main() {
    const first_window_size: usize = 4;
    const second_window_size:usize = 14;

    let position1 = find_marker::<first_window_size>(PUZZLE_INPUT).unwrap();
    let position2 = find_marker::<second_window_size>(PUZZLE_INPUT).unwrap();

    println!("window size {} position: {}", first_window_size, position1);
    println!("window size {} position: {}", second_window_size, position2);

}

#[derive(Debug, PartialEq)]
pub struct  UniquenessError;

fn find_marker<const WINDOW_SIZE: usize>(string:&str) -> Result<u32, UniquenessError> {//Result<u32, &'static str> {
    // string.as_bytes().windows(4).enumerate().for_each(|(i, window)| {
    for (i, window) in string.as_bytes().windows(WINDOW_SIZE).enumerate() {
        if window.is_unique() {
            println!("is unique{}: {}", i, window.iter().map(|&c| c as char).collect::<String>());
             return Ok((i  + WINDOW_SIZE) as u32);
        }else{
            println!("not unique{}: {}", i, window.iter().map(|&c| c as char).collect::<String>());
        }
    };
    Err(UniquenessError)
}
trait Iterable<'a> {
    type ItemType;

    type IterType: Iterator<Item = Self::ItemType>;

    fn into_iter(&'a self) -> Self::IterType;
}

trait Uniqueness<'a>: Iterable<'a>
{
    // type Bitem;
    // fn is_unique(&self) -> bool;

    fn is_unique(&'a self) -> bool
    // where Self::Bitem: Eq + Hash,
    where <Self as Iterable<'a>>::ItemType: Hash + Eq,
          Self: Sized,
    {
        let iter = self.into_iter();

        let mut in_list= HashMap::new();

        for (i, x) in iter.enumerate() {
            if in_list.contains_key(&x) {
                return false;
            }
            in_list.insert(x, i);
        }
        true
    }

}

impl<'a,T: 'a> Iterable<'a> for Vec<T> {
    type ItemType = &'a T;
    type IterType = Iter<'a, T>;

    fn into_iter(&'a self) -> Self::IterType {
        // let a : IntoIterator
        self.iter()
    }
}

impl<'a, T: 'a> Uniqueness<'a> for Vec<T>{

}

impl<'a,T> Iterable<'a> for &'a [T] {
    type ItemType = &'a T;

    type IterType = Iter<'a, T>;

    fn into_iter(&self) -> Self::IterType {
        self.iter()
    }
}


// impl<T, const SIZE: usize> Iterable for [T; SIZE] {
//     type IterType = Iter< T>;
//     type ItemType = T;
//     fn into_iter(self) -> Self::IterType {
//         self.iter()
//     }
// }

// impl <T, const SIZE: usize> Uniqueness for [T; SIZE] {}
impl<'a,T> Uniqueness<'a> for &'a [T]
    where T: Hash + Eq,
{
    // type Bitem = T;
    fn is_unique(&self) -> bool {
        let iter = self.iter();

        let mut in_list= HashMap::new();

        for (i, x) in iter.enumerate() {
            if in_list.contains_key(&x) {
                return false;
            }
            in_list.insert(x, i);
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use std::{assert_eq, vec};
    use super::*;

    // #[test]
    // #[ignore]
    // fn test_is_unique() {
    //     // let mut v1:[u8;5]  = [1, 2, 3, 4, 5];
    //     // v1.is_unique();
    //     // assert_eq!(v1.is_unique(), true);
    //     // let mut v2  = [1, 2, 3, 4, 5,1];
    //     // assert_eq!(v2.is_unique(), false);
    // }
    #[test]
    fn test_is_unique_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.is_unique();
        assert_eq!(v.is_unique(), true);
        v.push(1);
        assert_eq!(v.is_unique(), false);
    }

    #[test]
    fn test_marker_a() {
        let str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker::<4>(str).unwrap(), 7)
    }

    #[test]
    fn test_marker_b() {
        let str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker::<4>(str).unwrap(), 5)
    }

    #[test]
    fn test_marker_c() {
        let str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_marker::<4>(str).unwrap(), 6)
    }

    #[test]
    fn test_marker_d() {
        let str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_marker::<4>(str).unwrap(), 10)
    }

    #[test]
    fn test_marker_e() {
        let str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_marker::<4>(str).unwrap(), 11)
    }

    #[test]
    fn test_marker_f() {
        let str = "aaaaaaabbbbbbbbbbccccccddddddddd";
        assert_eq!(find_marker::<4>(str), Err(UniquenessError));
    }


    #[test]
    fn test_long_a(){
        let str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_marker::<14>(str).unwrap(), 19);
    }

    #[test]
    fn test_long_b(){
        let str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_marker::<14>(str).unwrap(),23);
    }
}