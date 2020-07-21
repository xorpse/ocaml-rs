use crate::error::{CamlError, Error};
use crate::sys;
use crate::tag::Tag;
use crate::Root;

/// Size is an alias for the platform specific integer type used to store size values
pub type Size = usize;

/// Value wraps the native OCaml `value` type transparently, this means it has the
/// same representation as an `ocaml_sys::Value`
#[derive(Debug, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Value(pub sys::value);

impl Clone for Value {
    fn clone(&self) -> Value {
        Value(self.0)
    }
}

/// `ToValue` is used to convert from Rust types to OCaml values
pub unsafe trait ToValue {
    /// Convert to OCaml value
    fn to_value(self, frame: &Root) -> Value;
}

/// `FromValue` is used to convert from OCaml values to Rust types
pub unsafe trait FromValue {
    /// Convert from OCaml value
    fn from_value(v: Value) -> Self;
}

unsafe impl ToValue for Value {
    fn to_value(self, _: &Root) -> Value {
        Value(self.0)
    }
}

unsafe impl FromValue for Value {
    #[inline]
    fn from_value(v: Value) -> Value {
        v
    }
}

const NONE: Value = Value(sys::val_int(0));
const UNIT: Value = Value(sys::UNIT);

impl Value {
    /// Returns a named value registered by OCaml
    pub fn named<T: FromValue>(name: &str) -> Option<T> {
        unsafe {
            let s = match crate::util::CString::new(name) {
                Ok(s) => s,
                Err(_) => return None,
            };
            let named = sys::caml_named_value(s.as_ptr());
            if named.is_null() {
                return None;
            }

            Some(FromValue::from_value(Value(*named)))
        }
    }

    /// Allocate a new value with the given size and tag.
    pub fn alloc(root: &Root, n: usize, tag: Tag) -> Value {
        unsafe { root.wrap(sys::caml_alloc(n as sys::uintnat, tag.into())) }
    }

    /// Allocate a new tuple value
    pub fn alloc_tuple(root: &Root, n: usize) -> Value {
        unsafe { root.wrap(sys::caml_alloc_tuple(n as sys::uintnat as sys::uintnat)) }
    }

    /// Allocate a new small value with the given size and tag
    pub fn alloc_small(root: &Root, n: usize, tag: Tag) -> Value {
        unsafe { root.wrap(sys::caml_alloc_small(n as sys::uintnat, tag.into())) }
    }

    /// Allocate a new value with a finalizer
    ///
    /// This calls `caml_alloc_final` under-the-hood, which can has less than ideal performance
    /// behavior. In most cases you should prefer `Pointer::alloc_custom` when possible.
    pub fn alloc_final<T>(
        root: &Root,
        finalizer: unsafe extern "C" fn(Value),
        cfg: Option<(usize, usize)>,
    ) -> Value {
        let (used, max) = cfg.unwrap_or_else(|| (0, 1));
        unsafe {
            root.wrap(sys::caml_alloc_final(
                core::mem::size_of::<T>() as sys::uintnat,
                core::mem::transmute(finalizer),
                used as sys::uintnat,
                max as sys::uintnat
            ))
        }
    }

    /// Allocate custom value
    pub fn alloc_custom<T: crate::Custom>(root: &Root) -> Value {
        let size = core::mem::size_of::<T>();
        unsafe {
            root.wrap(sys::caml_alloc_custom(&mut T::ops() as *mut _ as *mut sys::custom_operations, size as u64, T::USED as u64, T::MAX as u64))
        }
    }

    /// Allocate an abstract pointer value, it is best to ensure the value is
    /// on the heap using `Box::into_raw(Box::from(...))` to create the pointer
    /// and `Box::from_raw` to free it
    pub fn alloc_abstract_ptr<T>(root: &Root, ptr: *mut T) -> Value {
        let x = Self::alloc(root, 1, Tag::ABSTRACT);
        let dest = x.0 as *mut *mut T;
        unsafe {
            *dest = ptr;
        }
        x
    }

    /// Create a new Value from an existing OCaml `value`
    #[inline]
    pub const fn new(v: sys::value) -> Value {
        Value(v)
    }

    /// Get array length
    pub fn array_length(self) -> usize {
        unsafe { sys::caml_array_length(self.0) as usize }
    }

    /// See caml_register_global_root
    pub fn register_global_root(&mut self) {
        unsafe { sys::caml_register_global_root(&mut self.0) }
    }

    /// Set caml_remove_global_root
    pub fn remove_global_root(&mut self) {
        unsafe { sys::caml_remove_global_root(&mut self.0) }
    }

    /// Get the tag for the underlying OCaml `value`
    pub fn tag(self) -> Tag {
        unsafe { sys::tag_val(self.0).into() }
    }

    /// Convert a boolean to OCaml value
    pub const fn bool(b: bool) -> Value {
        Value::int(b as crate::Int)
    }

    /// Allocate and copy a string value
    pub fn string<S: AsRef<str>>(s: S) -> Value {
        unsafe {
            let len = s.as_ref().len();
            let value = crate::sys::caml_alloc_string(len as u64);
            let ptr = crate::sys::string_val(value);
            core::ptr::copy_nonoverlapping(s.as_ref().as_ptr(), ptr, len);
            Value(value)
        }
    }

    /// Convert from a pointer to an OCaml string back to an OCaml value
    ///
    /// # Safety
    /// This function assumes that the `str` argument has been allocated by OCaml
    pub unsafe fn of_str(s: &str) -> Value {
        Value(s.as_ptr() as sys::value)
    }

    /// Convert from a pointer to an OCaml string back to an OCaml value
    ///
    /// # Safety
    /// This function assumes that the `&[u8]` argument has been allocated by OCaml
    pub unsafe fn of_bytes(s: &[u8]) -> Value {
        Value(s.as_ptr() as sys::value)
    }

    /// OCaml Some value
    pub fn some(root: &Root, v: Value) -> Value {
        unsafe {
            let mut x = root.wrap(sys::caml_alloc(1, 0));
            x.store_field(0, v);
            x
        }
    }

    /// OCaml None value
    #[inline(always)]
    pub const fn none() -> Value {
        NONE
    }

    /// OCaml Unit value
    #[inline(always)]
    pub const fn unit() -> Value {
        UNIT
    }

    /// Create a variant value
    pub fn variant(root: &Root, tag: u8, value: Option<Value>) -> Value {
        match value {
            Some(v) => unsafe {
                let mut x = root.wrap(sys::caml_alloc(1, tag as sys::tag_t));
                x.store_field(0, v);
                x
            },
            None => unsafe { root.wrap(sys::caml_alloc(0, tag as sys::tag_t)) },
        }
    }

    /// Result.Ok value
    pub fn result_ok(root: &Root, value: Value) -> Value {
        Self::variant(root, 0, Some(value))
    }

    /// Result.Error value
    pub fn result_error(root: &Root, value: Value) -> Value {
        Self::variant(root, 1, Some(value))
    }

    /// Create an OCaml `int`
    pub const fn int(i: crate::Int) -> Value {
        Value(sys::val_int(i))
    }

    /// Create an OCaml `int`
    pub const fn uint(i: crate::Uint) -> Value {
        Value(sys::val_int(i as crate::Int))
    }

    /// Create an OCaml `Int64` from `i64`
    pub fn int64(root: &Root, i: i64) -> Value {
        unsafe { root.wrap(sys::caml_copy_int64(i)) }
    }

    /// Create an OCaml `Int32` from `i32`
    pub fn int32(root: &Root, i: i32) -> Value {
        unsafe { root.wrap(sys::caml_copy_int32(i)) }
    }

    /// Create an OCaml `Nativeint` from `isize`
    pub fn nativeint(root: &Root, i: isize) -> Value {
        unsafe { root.wrap(sys::caml_copy_nativeint(i as sys::intnat)) }
    }

    /// Create an OCaml `Float` from `f64`
    pub fn float(root: &Root, d: f64) -> Value {
        unsafe { root.wrap(sys::caml_copy_double(d)) }
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is a block
    pub fn is_block(self) -> bool {
        sys::is_block(self.0)
    }

    /// Check if a Value is an integer or block, returning true if
    /// the underlying value is an integer
    pub fn is_long(self) -> bool {
        sys::is_long(self.0)
    }

    /// Get index of underlying OCaml block value
    pub fn field<T: FromValue>(self, i: Size) -> T {
        unsafe { T::from_value(Value(*sys::field(self.0, i))) }
    }

    /// Set index of underlying OCaml block value
    pub fn store_field(&mut self, i: Size, val: Value) {
        unsafe { sys::store_field(self.0, i, val.0) }
    }

    /// Convert an OCaml `int` to `isize`
    pub const fn int_val(self) -> isize {
        sys::int_val(self.0)
    }

    /// Convert an OCaml `Float` to `f64`
    pub fn float_val(self) -> f64 {
        unsafe { *(self.0 as *const f64) }
    }

    /// Convert an OCaml `Int32` to `i32`
    pub fn int32_val(self) -> i32 {
        unsafe { *self.custom_ptr_val::<i32>() }
    }

    /// Convert an OCaml `Int64` to `i64`
    pub fn int64_val(self) -> i64 {
        unsafe { *self.custom_ptr_val::<i64>() }
    }

    /// Convert an OCaml `Nativeint` to `isize`
    pub fn nativeint_val(self) -> isize {
        unsafe { *self.custom_ptr_val::<isize>() }
    }

    /// Get pointer to data stored in an OCaml custom value
    pub fn custom_ptr_val<T>(self) -> *const T {
        unsafe { sys::field(self.0, 1) as *const T }
    }

    /// Get mutable pointer to data stored in an OCaml custom value
    pub fn custom_ptr_val_mut<T>(self) -> *mut T {
        unsafe { sys::field(self.0, 1) as *mut T }
    }

    /// Get pointer to the pointer contained by Value
    pub fn abstract_ptr_val<T>(self) -> *const T {
        unsafe { *(self.0 as *const *const T) }
    }

    /// Get mutable pointer to the pointer contained by Value
    pub fn abstract_ptr_val_mut<T>(self) -> *mut T {
        unsafe { *(self.0 as *mut *mut T) }
    }

    /// Extract OCaml exception
    pub fn exception<A: FromValue>(self) -> Option<A> {
        if !self.is_exception_result() {
            return None;
        }

        Some(A::from_value(Value(crate::sys::extract_exception(self.0))))
    }

    /// Call a closure with a single argument, returning an exception value
    pub fn call<A: ToValue>(self, root: &Root, arg: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = root.wrap(unsafe {
            sys::caml_callback_exn(
                self.0,
                arg.to_value(root).0,
            )
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with two arguments, returning an exception value
    pub fn call2<A: ToValue, B: ToValue>(self, root: &Root, arg1: A, arg2: B) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = root.wrap(unsafe {
            sys::caml_callback2_exn(
                self.0,
                arg1.to_value(root).0,
                arg2.to_value(root).0,
            )
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with three arguments, returning an exception value
    pub fn call3<A: ToValue, B: ToValue, C: ToValue>(
        self,
        root: &Root,
        arg1: A,
        arg2: B,
        arg3: C,
    ) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let mut v = root.wrap(unsafe {
            sys::caml_callback3_exn(
                self.0,
                arg1.to_value(root).0,
                arg2.to_value(root).0,
                arg3.to_value(root).0,
            )
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Call a closure with `n` arguments, returning an exception value
    pub fn call_n<A: AsRef<[Value]>>(self, root: &Root, args: A) -> Result<Value, Error> {
        if self.tag() != Tag::CLOSURE {
            return Err(Error::NotCallable);
        }

        let n = args.as_ref().len();
        let x = args.as_ref();

        let mut v = root.wrap(unsafe {
            sys::caml_callbackN_exn(
                self.0,
                n as i32,
                x.as_ptr() as *mut sys::value
            )
        });

        if v.is_exception_result() {
            v = v.exception().unwrap();
            Err(CamlError::Exception(v).into())
        } else {
            Ok(v)
        }
    }

    /// Modify an OCaml value in place
    pub fn modify(&mut self, v: Value) {
        unsafe { sys::caml_modify(&mut self.0, v.0) }
    }

    /// Determines if the current value is an exception
    pub fn is_exception_result(self) -> bool {
        crate::sys::is_exception_result(self.0)
    }

    /// Get hash variant as OCaml value
    pub fn hash_variant<S: AsRef<str>>(root: &Root, name: S, a: Option<Value>) -> Value {
        let s = crate::util::CString::new(name.as_ref()).expect("Invalid C string");
        let hash = unsafe { Value(sys::caml_hash_variant(s.as_ptr() as *mut sys::Char)) };
        match a {
            Some(x) => {
                let mut output = Value::alloc_small(root, 2, Tag(0));
                output.store_field(0, hash);
                output.store_field(1, x);
                output
            }
            None => hash,
        }
    }

    /// Get object method
    pub fn method<S: AsRef<str>>(self, root: &Root, name: S) -> Option<Value> {
        if self.tag() != Tag::OBJECT {
            return None;
        }

        let v = unsafe { sys::caml_get_public_method(self.0, Self::hash_variant(root, name, None).0) };

        if v == 0 {
            return None;
        }

        Some(Value(v))
    }

    /// Initialize OCaml value using `caml_initialize`
    pub fn initialize(&mut self, value: Value) {
        unsafe { sys::caml_initialize(&mut self.0, value.0) }
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated inside the OCaml heap,
    /// and may end up being moved or garbage collected.
    pub fn deep_clone_to_ocaml(self, root: &Root) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            let wosize = sys::wosize_val(self.0) as usize;
            let val1 = Self::alloc(root, wosize, self.tag());
            let ptr0 = self.0 as *const sys::value;
            let ptr1 = val1.0 as *mut sys::value;
            if self.tag() >= Tag::NO_SCAN {
                ptr0.copy_to_nonoverlapping(ptr1, wosize);
                return val1;
            }
            for i in 0..(wosize as isize) {
                sys::caml_initialize(
                    ptr1.offset(i),
                    Value(ptr0.offset(i).read()).deep_clone_to_ocaml(root).0,
                );
            }
            val1
        }
    }

    /// This will recursively clone any OCaml value
    /// The new value is allocated outside of the OCaml heap, and should
    /// only be used for storage inside Rust structures.
    #[cfg(not(feature = "no-std"))]
    pub fn deep_clone_to_rust(self) -> Self {
        if self.is_long() {
            return self;
        }
        unsafe {
            if self.tag() >= Tag::NO_SCAN {
                let slice0 = slice(self);
                let vec1 = slice0.to_vec();
                let ptr1 = vec1.as_ptr();
                core::mem::forget(vec1);
                return Value(ptr1.offset(1) as sys::value);
            }
            let slice0 = slice(self);
            let vec1: Vec<Value> = slice0
                .iter()
                .enumerate()
                .map(|(i, v)| if i == 0 { *v } else { v.deep_clone_to_rust() })
                .collect();
            let ptr1 = vec1.as_ptr();
            core::mem::forget(vec1);
            Value(ptr1.offset(1) as sys::value)
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe fn slice<'a>(value: Value) -> &'a [Value] {
    ::core::slice::from_raw_parts(
        (value.0 as *const Value).offset(-1),
        sys::wosize_val(value.0) as usize + 1,
    )
}
