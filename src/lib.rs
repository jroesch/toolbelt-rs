#![crate_type = "rlib"]
// The library is named "erty"
#![crate_name = "toolbelt"]

/* Optimize for efficiency later, we just return a String, could slice */
trait StringUtil: Str {
  fn chomp(&self) -> String;
  fn reverse(&self) -> String;
}

impl StringUtil for String {
  fn chomp(&self) -> String {
    self.as_slice().chars().rev().skip_while(|c| *c == '\r' || *c == '\n').collect()
  }

  fn reverse(&self) -> String {
    self.as_slice().chars().rev().collect()
  }
}

/* Not super efficient old solution was broken temp fix */
impl<'a> StringUtil for &'a str {
  fn chomp(&self) -> String {
    let s: String = self.chars().rev().skip_while(|c| *c == '\r' || *c == '\n').collect();
    s.reverse()
  }

  fn reverse(&self) -> String {
    self.chars().rev().collect()
  }
}

#[test]
fn chomp_removes_newlines() {
  assert_eq!("hello\n\n".chomp().as_slice(), "hello");
}

#[test]
fn chomp_removes_carriage_returns() {
  assert_eq!("hello\r\r".chomp().as_slice(), "hello");
}

#[test]
fn chomp_removes_both_newlines_and_cr() {
  assert_eq!("hello\n\r".chomp().as_slice(), "hello");
}

#[test]
fn chomp_perserves_chars() {
  assert_eq!("hello\n\rworld\n".chomp().as_slice(), "hello\n\rworld");
}
