// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]
#![feature(naked_functions, asm_sym)]

rust_syscall_tests::startup!();

use rust_syscall_tests::*;

fn main() -> Result<()> {
    if !is_enarx() {
        return Ok(());
    }

    if getuid()? != 1000 {
        return Err(1);
    }

    Ok(())
}