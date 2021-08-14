// example run x = 321 ->
// while loop 1 :
        // response = (0 * 10) + (321 % 10) = 1
        // x = 321 / 10 = 32
// while loop 2:
        // response = (1 * 10) + (32 % 10) = 12
        // x = 32 / 10 = 3
// while loop 3:
        // response = (12 * 10) + (3 % 10) = 123
        // x = 3 / 10 = 0
// response = 123

impl Solution {
  pub fn reverse(x: i32) -> i32 {
     fn solver(mut x: i32) -> Option<i32> {
          let mut response: i32 = 0;
          while x.abs() != 0 {
              response = response.checked_mul(10)?.checked_add(x % 10)?;
              x /= 10;
          }
          Some(response)
      }
      solver(x).unwrap_or_default()
  }
}