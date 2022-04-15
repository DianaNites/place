//! Placement new in Rust
#![cfg_attr(not(test), no_std)]

/// Initialize a struct in-place at `buf`, and return a mutable reference
///
/// `buf` is a MaybeUninit of your type
///
/// It is your responsibility to drop your type if needed when you're done with
/// it.
///
/// It is your responsibility to correctly produce the MaybeUninit
///
/// This macro will ensure that all fields are initialized, and is thus
/// safe to call.
///
/// # Examples
///
/// ```rust
/// # use place::place;
/// # use std::mem::MaybeUninit;
///
/// struct MyCoolStruct {
///     b: bool,
///     s: String,
/// }
///
/// let mut buf = MaybeUninit::uninit();
///
/// let x: &mut MyCoolStruct = place!(
///     buf,
///     MyCoolStruct {
///         b: true,
///         s: String::from("works"),
///     }
/// );
///
/// # // SAFETY: buf has been initialized above
/// # unsafe { buf.assume_init_drop() };
/// ```
#[macro_export]
macro_rules! place {
    (
        $buf:expr,
        $typ:ident {
            $(
                $f:ident: $f_val:expr
            ),*
            $(,)?
        }
    ) => {{
        use core::{mem::MaybeUninit, ptr::addr_of_mut};
        const _: () = {
            // Ignore useless warnings
            #[allow(unreachable_code, clippy::diverging_sub_expression)]
            fn _check_types() {
                // This check means Rust will validate that all struct fields were passed in,
                // meaning that all fields will be initialized below
                //
                // This check is the key to making this macro safe.
                $typ {
                    $(
                        $f: loop {}
                    ),*
                };
            }
        };
        // Ensures types are correct
        let buf: &mut MaybeUninit<$typ> = &mut $buf;
        let ptr = buf.as_mut_ptr();
        $(
            // SAFETY: Only pointers are used, and the above compile check
            // ensures all fields were specified
            unsafe { addr_of_mut!((*ptr).$f).write($f_val); }
        )*
        // SAFETY: All fields have been initialized above
        // The compiler ensures that all fields were used, all types were correct,
        // and that size and alignment are correct.
        unsafe { buf.assume_init_mut() }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::MaybeUninit;

    #[derive(Debug)]
    struct MyCoolStruct {
        b: bool,
        s: String,
        v: Vec<String>,
    }

    #[test]
    fn miri() {
        let mut buf = MaybeUninit::uninit();

        let x: &mut MyCoolStruct = place!(
            buf,
            MyCoolStruct {
                b: true,
                s: String::from("works"),
                v: vec![String::from("works")],
            }
        );
        dbg!(x);

        // SAFETY: buf has been initialized above
        unsafe { buf.assume_init_drop() };
    }
}
