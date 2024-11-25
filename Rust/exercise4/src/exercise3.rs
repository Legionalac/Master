// ***********FIRST VERSION**************

// fn my_assert( status: bool) -> Result<bool, bool>{
//     if status{
//         return Ok(true);
//     } else {
//         return Err(false);
//     }
//
// }
//
// fn main() {
//     match my_assert(3 == 5) {
//         Ok(_) => {}
//         Err(_) => { panic!("Assertion failed")}
//     }
// }

// ***********SECOND VERSION**************
//  fn my_assert( status: bool){
//      if !status{
//          panic!("Assertion failed");
//      }
//  }
//
//  fn main() {
//      my_assert(3 == 5);
// }

// ***********THIRD VERSION**************

macro_rules! my_assert {
    ($status: expr) => {
        if !$status{
            panic!("Assertion failed");
        }
    };
}

fn main() {
    my_assert!(3 == 5);
}
