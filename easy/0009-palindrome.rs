// Strings in Rust are dynamic heaps. Think Vectors. We use it when we need to modify string data, str is not able to do this.
// We are comparing values and modifying the value for num_backwards_as_string, so num_as_string must match it's data type.
impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    let num_as_string = x.to_string();
    let num_backwards_as_string = num_as_string.chars().rev().collect::<String>();
    return num_as_string == num_backwards_as_string;
  }
}