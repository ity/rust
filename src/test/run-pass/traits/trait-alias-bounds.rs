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

use std::marker::PhantomData;

trait SimpleAlias = Default;
trait GenericAlias<T> = Iterator<Item = T>;
trait Partial<T> = IntoIterator<Item = T>;
trait SpecificAlias = GenericAlias<i32>;
trait PartialEqRef<'a, T> = PartialEq<&'a T>;
trait StaticAlias = 'static;

trait Things<T> {}
trait Romeo {}
#[allow(dead_code)]
struct The<T>(T);
#[allow(dead_code)]
struct Fore<T>(T);
impl<T, U> Things<T> for The<U> {}
impl<T> Romeo for Fore<T> {}

trait WithWhere<Art, Thou> = Romeo + Romeo where Fore<(Art, Thou)>: Romeo;
trait BareWhere<Wild, Are> = where The<Wild>: Things<Are>;

trait Empty {}
trait EmptyAlias = Empty;
trait CloneDefault = Clone + Default;
trait SendSyncAlias = Send + Sync;
trait WhereSendAlias = where Self: Send;
trait SendEqAlias<T> = Send where T: PartialEq<Self>;
trait I32Iterator = Iterator<Item = i32>;

#[allow(dead_code)]
struct Foo<T: SendSyncAlias>(PhantomData<T>);
#[allow(dead_code)]
struct Bar<T>(PhantomData<T>) where T: SendSyncAlias;

impl EmptyAlias {}

impl<T: SendSyncAlias> Empty for T {}

fn a<T: CloneDefault>() -> (T, T) {
    let one = T::default();
    let two = one.clone();
    (one, two)
}

fn b(x: &impl SendEqAlias<i32>) -> bool {
    22_i32 == *x
}

fn c<T: I32Iterator>(x: &mut T) -> Option<i32> {
    x.next()
}

fn d<T: SendSyncAlias>() {
    is_send_and_sync::<T>();
}

fn is_send_and_sync<T: Send + Sync>() {}

fn main() {
    let both = a::<i32>();
    assert_eq!(both.0, 0);
    assert_eq!(both.1, 0);
    let both: (i32, i32) = a();
    assert_eq!(both.0, 0);
    assert_eq!(both.1, 0);

    assert!(b(&22));

    assert_eq!(c(&mut vec![22].into_iter()), Some(22));

    d::<i32>();
}
