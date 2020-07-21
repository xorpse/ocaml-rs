//#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// C char type
pub type Char = cty::c_char;

/*#[macro_export]
/// Registers OCaml parameters with the GC
macro_rules! caml_param {
    ($($n:expr),*) => {
        let mut caml_roots = $crate::CamlRootsBlock::default();

        let mut n = 0;
        $(
            if n == 5 {
                n = 0;
            }

            if n == 0 {
                caml_roots = $crate::CamlRootsBlock::default();
                #[allow(unused_unsafe)]
                unsafe {
                    caml_roots.next = $crate::local_roots();
                    $crate::set_local_roots(&mut caml_roots);
                };
                caml_roots.nitems = 1;
            }

            caml_roots.tables[n] = &$n as *const _ as *mut _;

            n += 1;
            caml_roots.ntables = n;
        )*
    }
}

/// Initializes and registers the given identifier(s) as a local value with the OCaml runtime.
///
/// ## Original C code
///
/// ```c
/// #define CAMLlocal1(x) \
/// value x = Val_unit; \
/// CAMLxparam1 (x)
/// ```
///
#[macro_export]
macro_rules! caml_local {
    ($($local:ident),*) => {
        #[allow(unused_mut)]
        $(let mut $local = $crate::UNIT;)*
        #[allow(unused_unsafe)]
        $crate::caml_param!($($local),*);
    }
}

#[macro_export]
macro_rules! caml_body {
    ($(($($param:expr),*))? $code:block) => {
        {
            #[allow(unused_unsafe)]
            let caml_frame = unsafe { $crate::local_roots() };
            $(
                $crate::caml_param!($($param),*);
            )?
            #[allow(unused_mut)]
            let mut res = || $code;
            let res = res();
            #[allow(unused_unsafe)]
            unsafe { $crate::set_local_roots(caml_frame) };
            res
        }
    }
}*/

#[cfg(not(feature = "docs-rs"))]
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_version"));

#[cfg(feature = "docs-rs")]
/// OCaml version (4.10.0, 4.09.1, ...)
pub const VERSION: &str = "";

#[cfg(not(feature = "docs-rs"))]
pub const PATH: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_path"));

#[cfg(feature = "docs-rs")]
/// Path to OCaml libraries
pub const PATH: &str = "";

#[cfg(not(feature = "docs-rs"))]
pub const COMPILER: &str = include_str!(concat!(env!("OUT_DIR"), "/ocaml_compiler"));

#[cfg(feature = "docs-rs")]
/// Path to OCaml compiler
pub const COMPILER: &str = "";

/*mod mlvalues;
#[macro_use]
mod memory;
mod alloc;
pub mod bigarray;
mod callback;
mod custom;
mod fail;
mod runtime;
mod state;
mod tag;

pub use self::mlvalues::value;
pub use self::tag::Tag;
pub use alloc::*;
pub use callback::*;
pub use custom::*;
pub use fail::*;
pub use memory::*;
pub use mlvalues::*;
pub use runtime::*;
pub use state::*;*/

mod tag;
pub use tag::*;

pub mod bigarray;

pub const fn is_exception_result(val: value) -> bool {
    (val as usize) & 3 == 2
}

pub const fn extract_exception(val: value) -> value {
    val & !3
}

/// #ifdef ARCH_BIG_ENDIAN
/// #define Tag_val(val) (((unsigned char *) (val)) [-1])
/// #else
/// #define Tag_val(val) (((unsigned char *) (val)) [-sizeof(value)])
/// #endif
#[cfg(target_endian = "big")]
#[inline]
pub const unsafe fn tag_val(val: value) -> Tag {
    *(val as *const u8).offset(-1)
}

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn tag_val(val: value) -> tag_t {
    *(val as *const tag_t).offset(-(core::mem::size_of::<value>() as isize))
}

#[inline]
pub unsafe fn hd_val(val: value) -> header_t {
    *(val as *const header_t).offset(-1)
}

#[inline]
pub unsafe fn wosize_val(val: value) -> uintnat {
    hd_val(val) >> 10
}

/// `(((intnat)(x) << 1) + 1)`
pub const fn val_int(i: isize) -> value {
    ((i as value) << 1) + 1
}

pub const fn int_val(val: value) -> isize {
    ((val as isize) >> 1) as isize
}

pub fn is_block(v: value) -> bool {
    (v & 1) == 0
}

pub fn is_long(v: value) -> bool {
    (v & 1) != 0
}

/// The OCaml `()` (`unit`) value
pub const UNIT: value = val_int(0);

/// Empty list value
pub const EMPTY_LIST: value = val_int(0);

/// The OCaml `true` value
pub const TRUE: value = val_int(1);

/// OCaml `false` value
pub const FALSE: value = val_int(0);

/// Extracts a machine `ptr` to the bytes making up an OCaml `string`
#[inline]
pub const unsafe fn string_val(val: value) -> *mut u8 {
    val as *mut u8
}

/// Extract a field from an OCaml value
///
/// # Safety
///
/// This function does no bounds checking or validation of the OCaml values
pub unsafe fn field(block: value, index: usize) -> *mut value {
    (block as *mut value).add(index)
}

/// Stores the `$val` at `$offset` in the `$block`.
///
/// # Original C code
///
/// ```c
/// Store_field(block, offset, val) do{ \
///   mlsize_t caml__temp_offset = (offset); \
///   value caml__temp_val = (val); \
///   caml_modify (&Field ((block), caml__temp_offset), caml__temp_val); \
/// }while(0)
/// ```
///
/// # Example
/// ```norun
/// // stores some_value in the first field in the given block
/// store_field!(some_block, 1, some_value)
/// ```
macro_rules! store_field {
    ($block:expr, $offset:expr, $val:expr) => {
        let offset = $offset;
        let val = $val;
        let block = $block;
        $crate::caml_modify(field(block, offset), val);
    };
}

/// Stores the `value` in the `block` at `offset`.
///
/// # Safety
///
/// No bounds checking or validation of the OCaml values is done in this function
pub unsafe fn store_field(block: value, offset: usize, value: value) {
    store_field!(block, offset, value);
}