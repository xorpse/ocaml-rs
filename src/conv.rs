use crate::{
    value::{FromValue, IntoValue, Value},
    Runtime, Tag,
};

macro_rules! value_i {
    ($t:ty) => {
        unsafe impl IntoValue for $t {
            fn into_value(self, _rt: &Runtime) -> $crate::Value {
                unsafe { $crate::Value::int(self as crate::Int) }
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                unsafe { v.int_val() as $t }
            }
        }
    };
    ($($t:ty),*) => {
        $(value_i!($t);)*
    }
}

macro_rules! value_f {
    ($t:ty) => {
        unsafe impl IntoValue for $t {
            fn into_value(self, rt: &Runtime) -> $crate::Value {
                unsafe { $crate::Value::float(rt, self as crate::Float) }
            }
        }

        unsafe impl FromValue for $t {
            fn from_value(v: $crate::Value) -> $t {
                unsafe { v.float_val() as $t }
            }
        }
    };
    ($($t:ty),*) => {
        $(value_f!($t);)*
    }
}

value_i!(i8, u8, i16, u16, crate::Int, crate::Uint);
value_f!(f32, f64);

unsafe impl IntoValue for i64 {
    fn into_value(self, rt: &Runtime) -> crate::Value {
        unsafe { Value::int64(rt, self) }
    }
}

unsafe impl FromValue for i64 {
    fn from_value(v: Value) -> i64 {
        unsafe { v.int64_val() }
    }
}

unsafe impl IntoValue for u64 {
    fn into_value(self, rt: &Runtime) -> crate::Value {
        unsafe { Value::int64(rt, self as i64) }
    }
}

unsafe impl FromValue for u64 {
    fn from_value(v: Value) -> u64 {
        unsafe { v.int64_val() as u64 }
    }
}

unsafe impl IntoValue for i32 {
    fn into_value(self, rt: &Runtime) -> crate::Value {
        unsafe { Value::int32(rt, self) }
    }
}

unsafe impl FromValue for i32 {
    fn from_value(v: Value) -> i32 {
        unsafe { v.int32_val() }
    }
}

unsafe impl IntoValue for u32 {
    fn into_value(self, rt: &Runtime) -> crate::Value {
        unsafe { Value::int32(rt, self as i32) }
    }
}

unsafe impl FromValue for u32 {
    fn from_value(v: Value) -> u32 {
        unsafe { v.int32_val() as u32 }
    }
}

struct Incr(usize);

impl Incr {
    fn get(&mut self) -> usize {
        let i = self.0;
        self.0 = i + 1;
        i
    }
}

macro_rules! tuple_impl {
    ($($t:ident: $n:tt),*) => {
        unsafe impl<'a, $($t: FromValue),*> FromValue for ($($t,)*) {
            fn from_value(v: Value) -> ($($t,)*) {
                let mut i = Incr(0);
                #[allow(unused)]
                (
                    $(
                        $t::from_value(unsafe { v.field(i.get()) }),
                    )*
                )
            }
        }

        unsafe impl<$($t: IntoValue),*> IntoValue for ($($t,)*) {
            fn into_value(self, rt: &Runtime) -> crate::Value {
                #[allow(unused)]
                let mut len = 0;
                $(
                    #[allow(unused)]
                    {
                        len = $n + 1;
                    }
                )*

                unsafe {
                    let mut v = $crate::Value::alloc(rt, len, Tag(0));
                    $(
                        v.store_field(rt, $n, self.$n);
                    )*

                    v
                }
            }
        }
    };
}

tuple_impl!(A: 0);
tuple_impl!(A: 0, B: 1);
tuple_impl!(A: 0, B: 1, C: 2);
tuple_impl!(A: 0, B: 1, C: 2, D: 3);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18, T: 19);
tuple_impl!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7, I: 8, J: 9, K: 10, L: 11, M: 12, N: 13, O: 14, P: 15, Q: 16, R: 17, S: 18, T: 19, U: 20);

unsafe impl IntoValue for bool {
    fn into_value(self, _rt: &Runtime) -> Value {
        unsafe { Value::int(self as isize) }
    }
}

unsafe impl FromValue for bool {
    fn from_value(v: Value) -> bool {
        unsafe { v.int_val() != 0 }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl IntoValue for String {
    fn into_value(self, rt: &Runtime) -> Value {
        unsafe { Value::string(rt, self.as_str()) }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl FromValue for String {
    fn from_value(value: Value) -> String {
        unsafe { value.string_val().into() }
    }
}

unsafe impl IntoValue for () {
    fn into_value(self, _rt: &Runtime) -> Value {
        Value::unit()
    }
}

unsafe impl<'a, T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Option<T> {
        if value == Value::none() {
            return None;
        }

        Some(unsafe { value.field(0) })
    }
}

unsafe impl<'a, T: IntoValue> IntoValue for Option<T> {
    fn into_value(self, rt: &Runtime) -> Value {
        match self {
            Some(y) => crate::frame!(rt: (x) {
                x = y.into_value(rt);
                unsafe { Value::some(rt, x) }
            }),
            None => Value::none(),
        }
    }
}

unsafe impl<'a> FromValue for &str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe {
            let slice = ::core::slice::from_raw_parts(ptr, len);
            ::core::str::from_utf8(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl<'a> IntoValue for &str {
    fn into_value(self, rt: &Runtime) -> Value {
        frame!(rt: (value) {
            unsafe {
                value.0 = crate::sys::caml_alloc_string(self.len());
                let ptr = crate::sys::string_val(value.0);
                core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
                value
            }
        })
    }
}

unsafe impl<'a> FromValue for &mut str {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe {
            let slice = ::core::slice::from_raw_parts_mut(ptr, len);
            ::core::str::from_utf8_mut(slice).expect("Invalid UTF-8")
        }
    }
}

unsafe impl<'a> IntoValue for &mut str {
    fn into_value(self, rt: &Runtime) -> Value {
        frame!(rt: (value) {
            unsafe {
                value.0 = crate::sys::caml_alloc_string(self.len());
                let ptr = crate::sys::string_val(value.0);
                core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
                value
            }
        })
    }
}

unsafe impl<'a> FromValue for &[u8] {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe { ::core::slice::from_raw_parts(ptr, len) }
    }
}

unsafe impl<'a> IntoValue for &[u8] {
    fn into_value(self, rt: &Runtime) -> Value {
        frame!(rt: (value) {
            unsafe {
                value.0 = crate::sys::caml_alloc_string(self.len());
                let ptr = crate::sys::string_val(value.0);
                core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
                value
            }
        })
    }
}

unsafe impl<'a> FromValue for &mut [u8] {
    fn from_value(value: Value) -> Self {
        let len = unsafe { crate::sys::caml_string_length(value.0) };
        let ptr = unsafe { crate::sys::string_val(value.0) };
        unsafe { ::core::slice::from_raw_parts_mut(ptr, len) }
    }
}

unsafe impl<'a> IntoValue for &mut [u8] {
    fn into_value(self, rt: &Runtime) -> Value {
        frame!(rt: (value) {
            unsafe {
                value.0 = crate::sys::caml_alloc_string(self.len());
                let ptr = crate::sys::string_val(value.0);
                core::ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
                value
            }
        })
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<'a, V: IntoValue> IntoValue for Vec<V> {
    fn into_value(self, rt: &Runtime) -> Value {
        let len = self.len();
        let mut arr = unsafe { Value::alloc(rt, len, Tag(0)) };

        for (i, v) in self.into_iter().enumerate() {
            unsafe {
                arr.store_field(rt, i, v);
            }
        }

        arr
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<'a, V: FromValue> FromValue for Vec<V> {
    fn from_value(v: Value) -> Vec<V> {
        unsafe {
            let len = crate::sys::caml_array_length(v.0);
            let mut dst = Vec::with_capacity(len);
            for i in 0..len {
                dst.push(V::from_value(Value::new(*crate::sys::field(v.0, i))))
            }
            dst
        }
    }
}

unsafe impl<'a> FromValue for &'a [Value] {
    fn from_value(value: Value) -> &'a [Value] {
        unsafe {
            ::core::slice::from_raw_parts(
                crate::sys::field(value.0, 0) as *mut Value,
                crate::sys::wosize_val(value.0),
            )
        }
    }
}

unsafe impl<'a> FromValue for &'a mut [Value] {
    fn from_value(value: Value) -> &'a mut [Value] {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                crate::sys::field(value.0, 0) as *mut Value,
                crate::sys::wosize_val(value.0),
            )
        }
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: Ord + FromValue, V: FromValue> FromValue for std::collections::BTreeMap<K, V> {
    fn from_value(v: Value) -> std::collections::BTreeMap<K, V> {
        let mut dest = std::collections::BTreeMap::new();
        unsafe {
            let mut tmp = v;
            while tmp.0 != crate::sys::EMPTY_LIST {
                let (k, v) = tmp.field(0);
                dest.insert(k, v);
                tmp = tmp.field(1);
            }
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<K: IntoValue, V: IntoValue> IntoValue for std::collections::BTreeMap<K, V> {
    fn into_value(self, rt: &Runtime) -> Value {
        let mut list = crate::List::empty();

        crate::frame!(rt: (l, k_, v_) {
            for (k, v) in self.into_iter().rev() {
                k_ = k.into_value(rt);
                v_ = v.into_value(rt);
                list = list.add(rt, (k_, v_));
            }

            l = list.into_value(rt);
            l
        })
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: FromValue> FromValue for std::collections::LinkedList<T> {
    fn from_value(v: Value) -> std::collections::LinkedList<T> {
        let mut dest = std::collections::LinkedList::new();

        unsafe {
            let mut tmp = v;
            while tmp.0 != crate::sys::EMPTY_LIST {
                let t = tmp.field(0);
                dest.push_back(t);
                tmp = tmp.field(1);
            }
        }

        dest
    }
}

#[cfg(not(feature = "no-std"))]
unsafe impl<T: IntoValue> IntoValue for std::collections::LinkedList<T> {
    fn into_value(self, rt: &Runtime) -> Value {
        let mut list = crate::List::empty();

        frame!(rt: (l, x) {
            for t in self.into_iter().rev() {
                let v = t.into_value(rt);
                list = list.add(rt, v);
            }
            l = list.into_value(rt);
            l
        })
    }
}

unsafe impl<'a> IntoValue for &Value {
    fn into_value(self, _rt: &Runtime) -> Value {
        unsafe { Value::new(self.0) }
    }
}
