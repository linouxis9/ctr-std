// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-C panic=abort
// error-pattern: language item required, but not found: `panic_info`

#![feature(lang_items)]
#![feature(no_core)]
#![feature(panic_handler)]
#![no_core]
#![no_main]

#[panic_handler]
fn panic() -> ! {
    loop {}
}

#[lang = "sized"]
trait Sized {}
