/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    // let mut result = Vec::new();
    // for i in v{
    //     result.push(i + n);
    // }
    // result

    let mut v = v.clone();
    for i in &mut v{
        *i += n;
    }
    v
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    // for i in 0..v.len() {
    //     v[i] += n;
    // }
    for i in v{
        *i += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    // Use a HashSet to track unique elements
    // let mut seen = HashSet::new();
    // let mut write_index = 0;
    // for &value in v.clone().iter() {
    //     if seen.insert(value) {
    //         // If the value was not seen before, write it to the current write index
    //         v[write_index] = value;
    //         write_index += 1;
    //     }
    // }
    // v.truncate(write_index);
    let mut set = HashSet::new(); 
    v.retain(|i| set.insert(*i));
    // The retain method keeps elements for which the closure returns true.
    // Here, we insert each element into the HashSet and keep it only if it was
    // not already present, effectively removing duplicates.
    // The HashSet ensures that each element is unique.
    // The retain method modifies the vector in place, so we don't need to return anything.
    // The HashSet is used to track which elements have already been seen.
    // This is a more efficient way to deduplicate elements in a vector.

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
