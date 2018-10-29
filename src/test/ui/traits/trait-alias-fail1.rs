// Copyright 2017-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(trait_alias)]

trait CloneDefault<T> = Default where T: Clone;
trait BoundedAlias<T: Clone = ()> = Default;

trait Foo {}
trait A<T: Foo> {}
trait B<T> = A<T>; // T cannot be unbounded

impl CloneDefault for () {}

fn main() {}
