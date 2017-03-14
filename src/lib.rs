#![cfg(target_arch="x86_64")]

// #![warn(missing_docs)]

#![feature(const_fn)]
#![feature(asm)]
#![feature(associated_consts)]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, allow(unused_features))]

#![no_std]

pub use address::{VirtualAddress, PhysicalAddress};

#[macro_use] extern crate bitflags;
extern crate bit_field;

pub mod instructions;
pub mod idt;
pub mod paging;
pub mod tss;
pub mod control_regs;
pub mod msr;
pub mod flags;
pub mod segmentation;
pub mod tlb;

mod address;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}
