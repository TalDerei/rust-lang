// control2.rs
// Make me compile! Scroll down for hints :)

#[derive(Debug)]
enum Colors {
  Red,
  Green,
  Blue,
  Orange,
  Yellow,
}

#[test]
fn main() {
  let color = Colors::Red;

  let fruit = match color {
    Colors::Red => "apple",
    Colors::Green => "grape",
    Colors::Yellow => "banana",
    Colors::Blue => "blueberry", 
    Colors::Orange => "orange", 
  };

  println!("{:?}s are {:?}", fruit, color)

}



























// The match expression is wrong is a couple ways. 
// First, it doesn't consider all the possible choices for color, which is of type Colors. Fix this by either using a wildcard pattern, or match each of the other possibilities for the enum.
// Second, one of the match arms does not match the type of the other arms. Make it match.
// Third, there is no semicolon at the end;

