use std::collections::HashMap;
use std::hash::Hash;
use std::slice::Windows;
use std::slice::Iter;
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
trait Iterable {
    type InnerItem;
    fn iter(&self) -> Iter<'_, Self::InnerItem>;
}

trait Uniqueness: Iterable
{
    // type Bitem;
    // fn is_unique(&self) -> bool;

    fn is_unique(&self) -> bool
    // where Self::Bitem: Eq + Hash,
    where <Self as Iterable>::InnerItem: Hash + Eq,
    {
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

impl<T> Iterable for &[T] {
    type InnerItem = T;
    fn iter(&self) -> Iter<'_, Self::InnerItem> {
        self.iter()
    }
}

impl<T, const SIZE: usize> Iterable for [T; SIZE] {
    type InnerItem = ();

    fn iter(&self) -> Iter<'_, Self::InnerItem> {
        self.iter()
    }
}

impl <T, const SIZE: usize> Uniqueness for [T; SIZE] {}
impl <T> Uniqueness for &[T]
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
    #[test]
    fn test_is_unique() {
        let mut v1:[u8;5]  = [1, 2, 3, 4, 5];
        v1.is_unique();
        assert_eq!(v1.is_unique(), true);
        let mut v2  = [1, 2, 3, 4, 5,1];
        assert_eq!(v2.is_unique(), false);
    }
    #[test]
    fn test_is_unique_vec() {
        // let mut v = vec![1, 2, 3, 4, 5];
        // v.is_unique();
        // assert_eq!(v.is_unique(), true);
        // v.push(1);
        // assert_eq!(v.is_unique(), false);
    }
}