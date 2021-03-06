// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(catch_expr)]

pub fn main() {
    let res: Result<i32, i32> = do catch {
        Err("")?; //~ ERROR the trait bound `i32: std::convert::From<&str>` is not satisfied
        Ok(5)
    };
    let res: Result<i32, i32> = do catch {
        Ok("") //~ mismatched types
    };
}
