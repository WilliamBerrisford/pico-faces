#![cfg_attr(not(test), no_std)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod external;
