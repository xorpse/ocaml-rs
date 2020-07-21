use crate::{ToValue, Root};

#[cfg(not(feature = "no-std"))]
static PANIC_HANDLER_INIT: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(feature = "no-std"))]
#[doc(hidden)]
pub fn init_panic_handler() {
    if PANIC_HANDLER_INIT.compare_and_swap(false, true, std::sync::atomic::Ordering::Relaxed) {
        return;
    }

    ::std::panic::set_hook(Box::new(|info| {
        let err = info.payload();
        let msg = if err.is::<&str>() {
            err.downcast_ref::<&str>().unwrap()
        } else if err.is::<String>() {
            err.downcast_ref::<String>().unwrap().as_ref()
        } else {
            "rust panic"
        };

        let root = Root::new();
        let s = msg.to_value(&root);
        if let Some(err) = crate::Value::named("Rust_exception") {
            unsafe {
                crate::sys::caml_raise_with_arg(err, s.0);
            }
        }

        unsafe {
            crate::sys::caml_failwith_value(s.0);
        }
    }))
}

#[cfg(feature = "no-std")]
#[doc(hidden)]
pub fn init_panic_handler() {}


/// `body!` is needed to help the OCaml runtime to manage garbage collection, it should
/// be used to wrap the body of each function exported to OCaml. Panics from Rust code
/// will automatically be unwound/caught here (unless the `no-std` feature is enabled)
///
/// ```rust
/// #[no_mangle]
/// pub extern "C" fn example(a: ocaml::Value, b: ocaml::Value) -> ocaml::Value {
///     ocaml::body!(root: (a, b) {
///         let a = a.int_val();
///         let b = b.int_val();
///         ocaml::Value::int(a + b)
///     })
/// }
/// ```
#[macro_export]
macro_rules! body {
    ($frame:ident: $(($($param:expr),*))? $code:block) => {{
        // Ensure panic handler is initialized
        $crate::init_panic_handler();

        // Initialize OCaml frame
        #[allow(unused_unsafe)]
        let $frame = $crate::Root::new();

        // Initialize parameters
        $(
            $($frame.wrap_ref(&$param);)*
        )?

        // Execute Rust function
        #[allow(unused_mut)]
        let mut res = || {$code };
        let res = res();
        $frame.end();

        res
    }}
}

#[macro_export]
/// Convenience macro to create an OCaml array
macro_rules! array {
    ($($x:expr),*) => {{
        $crate::ToValue::to_value(&vec![$($crate::ToValue::to_value(&$x)),*])
    }}
}

#[macro_export]
/// Convenience macro to create an OCaml list
macro_rules! list {
    ($($x:expr),*) => {{
        let mut l = $crate::list::empty();
        for i in (&[$($x),*]).into_iter().rev() {
            $crate::list::push_hd(&mut l, $crate::ToValue::to_value(i));
        }
        l
    }};
}
