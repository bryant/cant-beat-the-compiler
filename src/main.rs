#![feature(asm)]

extern crate rand;
extern crate libc;

use std::vec::Vec;
use std::ops::{Index, IndexMut};
use rand::{Rand, Rng};

#[derive(Copy, Clone, Debug)]
struct Item {
    key: i32,
    value: i32,
}

struct UnsafeItemz<'a, T: 'a>(&'a mut [T]);

impl<'a, T: 'a> Index<usize> for UnsafeItemz<'a, T> {
    type Output = T;
    fn index(&self, i: usize) -> &T { unsafe { self.0.get_unchecked(i) } }
}

impl<'a, T: 'a> IndexMut<usize> for UnsafeItemz<'a, T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        unsafe { self.0.get_unchecked_mut(i) }
    }
}

impl<'a, T: 'a + Copy> UnsafeItemz<'a, T> {
    fn swap(&mut self, a: usize, b: usize) {
        let tmp = self[a];
        self[a] = self[b];
        self[b] = tmp;
    }
}

fn rdtscp() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!("rdtscp" : "={eax}"(low), "={edx}"(high) : : "ecx": "volatile");
    }
    (high as u64) << 32 | low as u64
}

impl Rand for Item {
    fn rand<R: Rng>(r: &mut R) -> Self {
        Item {
            key: r.gen(),
            value: r.gen(),
        }
    }
}

struct StackStack<T> {
    space: [T; 1000],
    top: usize,
}

impl<T: Copy> StackStack<T> {
    fn new() -> Self {
        StackStack {
            space: unsafe { [std::mem::uninitialized(); 1000] },
            top: 0,
        }
    }

    fn push(&mut self, t: T) {
        unsafe { *self.space.get_unchecked_mut(self.top) = t };
        self.top += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.top {
            0 => None,
            _ => {
                self.top -= 1;
                unsafe { Some(*self.space.get_unchecked(self.top)) }
            }
        }
    }
}

// [start, end], inclusive
fn pivot_sort(items: &mut UnsafeItemz<Item>, start: usize, end: usize,
              stack: &mut StackStack<(usize, usize)>)
              -> Option<(usize, usize)> {
    let mut partition = start;
    let pivot = items[end];
    for pos in start..end - 1 {
        unsafe {
            if items[pos].key <= pivot.key {
                items.swap(pos, partition);
                partition += 1;
            }
        }
    }

    items[end] = items[partition];
    items[partition] = pivot;

    if partition + 1 < end {
        stack.push((partition + 1, end));
    }
    if start + 1 < partition { Some((start, partition - 1)) } else { None }
}

#[inline(never)]
fn sort(items: &mut [Item]) {
    if items.len() < 2 {
        return;
    }

    let mut stack = StackStack::new();
    let mut cur = (0, items.len() - 1);

    loop {
        match pivot_sort(&mut UnsafeItemz(items), cur.0, cur.1, &mut stack) {
            None => {
                match stack.pop() {
                    None => return,
                    Some(thing) => cur = thing,
                }
            }
            Some(thing) => cur = thing,
        }
    }
}

#[allow(dead_code)]
fn check(items: &mut [Item]) -> Option<usize> {
    for (idx, x) in items.windows(2).enumerate() {
        if x[0].key > x[1].key {
            return Some(idx);
        }
    }
    None
}

fn main() {
    let mut r = rand::weak_rng();
    let mut run = || {
        unsafe { libc::srand(12345) };
        let mut items = (0..1000_000)
            .map(|_| unsafe {
                Item {
                    key: libc::rand(),
                    value: libc::rand(),
                }
            })
            .collect::<Vec<Item>>();
        let now = rdtscp();
        sort(&mut items);
        let rv = rdtscp() - now;
        rv
    };
    println!("{}", (0..100).map(|_| run()).min().unwrap());
}
