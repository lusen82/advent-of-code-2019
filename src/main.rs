use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use std::error::Error;
use std::cmp::{max, min};
use std::ops::BitAnd;
use std::slice::SliceIndex;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_8;
fn main() {
    //day_1::day_1();
    //day_2::day_2();
   // day_3::day_3();
  //  day_4::day_4_b();
    day_5::day_5_b();
    //day_6::day_6();
    //day_8::day_8();

}


//#[test]
//fn version() {
//    assert_that!(&["--version"], starts_with("rustfmt "));
//    assert_that!(&["--version"], starts_with("rustfmt "));
//    assert_that!(&["--", "-V"], starts_with("rustfmt "));
//    assert_that!(&["--", "--version"], starts_with("rustfmt "));
//}









