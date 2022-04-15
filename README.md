# Place

Placement new in Rust

A simple wrapper around `MaybeUninit` that allows one to simply and safely
initialize a struct field-by-field

## Usage

```rust
use place::place;
use std::mem::MaybeUninit;

struct MyCoolStruct {
    b: bool,
    u: u32,
}

let mut buf = MaybeUninit::uninit();

let x: &mut MyCoolStruct = place!(
    buf,
    MyCoolStruct {
        b: true,
        u: 69420,
    }
);

// SAFETY: buf has been initialized above
unsafe { buf.assume_init_drop() };
```
