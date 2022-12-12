use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Windows;
const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
fn main() {
    let str = INPUT1;
    let chars = str.as_bytes();
    let slice = ['r','u','s','t'];

    find_marker(INPUT1);

    
}

fn find_marker(string:&str){
    string.as_bytes().windows(4).enumerate().for_each(|(i, window)| {
        if window.is_unique() {
            println!("is unique{}: {}", i, window.iter().map(|&c| c as char).collect::<String>());
        }else{
            println!("not unique{}: {}", i, window.iter().map(|&c| c as char).collect::<String>());
        }
        // let a = window.iter().map(|&x| x as char).collect::<String>();
        //     println!("Found at {},{:?}--{}", i, window,a);
    });

}
trait Uniqueness
{
    fn is_unique(&self) -> bool;

}
impl <T> Uniqueness for &[T]
    where T: Hash + Eq + Clone,
{
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
    #[test]
    fn test_is_unique() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.is_unique();
        assert_eq!(v.is_unique(), true);
        v.push(1);
        assert_eq!(v.is_unique(), false);
    }
}