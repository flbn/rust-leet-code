// example run nums = [3, 2, 4], target = 6 ->
// for (0, 3) -> match (&(6 - 3)) { Some(&3) { return vec![&3, 0]} , None { map.insert(3, 0)} } *None*
// for (1, 2) -> match (&(6 - 2)) { Some(&4) { return vec![&4, 1]} , None { map.insert(2, 1)} } *None*
// for (2, 4) -> match (&(6 - 4)) { Some(&2) { return vec![&2, 2]} , None { map.insert(4, 2)} } *Some*
// output = [1, 2] AKA: [ index for location of value 2 in Map, current index]

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - *v)) {
                Some(&resp) => { println!("some {}, {}", resp, i as i32); return vec![resp, i as i32] },
                None => { println!("none {}, {}", *v, i as i32); map.insert(*v, i as i32) },
            };
        }
        vec![]
    }
}