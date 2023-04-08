// It is possible to give alias when bringing paths into the scope with the
// `as` keyword. For example:
use std::fmt::Result;
use std::io::Result as IoResult;

// If it is necessary to bring more than one module or package into scope that
// belongs to the same crate, it is possible to use a single line:
use std::{cmp::Ordering, io};  // this brings the cmp::Ordering and io to scope


// It is possible to merge common paths into with the command self. Example:
// use std::{io};
// use std::io::Write;fn 
//
// The commom part in this use statement is std::io. It is possible to merge
// with:
// use std::{self, Write}
//
// This line brings `io` ans `Write` to scope.


fn function1() -> Result {
    unimplemented!();
}


fn function2() -> IoResult {
    unimplemented!();
}




fn main() {
    function1();
    function2();
}
