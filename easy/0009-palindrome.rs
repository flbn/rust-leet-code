/*  **** AS STRING **** */

// Strings in Rust are dynamic heaps. Think Vectors. We use it when we need to modify string data, str is not able to do this.
// We are comparing values and modifying the value for num_backwards_as_string, so num_as_string must match it's data type.
/* impl Solution {
   pub fn is_palindrome(x: i32) -> bool {
     let num_as_string = x.to_string();
     let num_backwards_as_string = num_as_string.chars().rev().collect::<String>();
     return num_as_string == num_backwards_as_string;
  }
} */

/* **** AS INTEGET WITH OVERFLOW PROTECTION **** */

// example run x = 121 ->
// while loop 1 :
        // response = (0 * 10) + (121 % 10) = 1
        // x = 121 / 10 = 12
// while loop 2:
        // response = (1 * 10) + (12 % 10) = 12
        // x = 12 / 10 = 1
// while loop 3:
        // response = (12 * 10) + (1 % 10) = 121
        // x = 3 / 10 = 0
// response = 121


// example run x = 10 ->
// while loop 1 :
        // response = (0 * 10) + (10 % 10) = 0
        // x = 0 / 10 = 0
// response = 0

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
      if x < 0 || (x != 0 && x % 10 == 0) {
          return false
      }
      fn solver(mut x: i32) -> Option<i32> {
          let mut response: i32 = 0;
          while x != 0 {
              response = response.checked_mul(10)?.checked_add(x % 10)?;
              x /= 10;
          }
          Some(response)
      }
      let response = solver(x).unwrap_or_default();
      response == x || response / 10 == x
  }
}