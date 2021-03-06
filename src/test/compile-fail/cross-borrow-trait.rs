// Copyright 2012-2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that cross-borrowing (implicitly converting from `Box<T>` to `&T`) is
// forbidden when `T` is a trait.

struct Foo;
trait Trait { fn foo(&self) {} }
impl Trait for Foo {}

pub fn main() {
    // FIXME (#22405): Replace `Box::new` with `box` here when/if possible.
    let x: Box<Trait> = Box::new(Foo);
    let _y: &Trait = x; //~  ERROR mismatched types
                        //~| expected `&Trait`
                        //~| found `Box<Trait>`
                        //~| expected &-ptr
                        //~| found box
}
