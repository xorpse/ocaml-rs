use crate::sys;

/// OCaml tags are used to provide type information to the garbage collector
///
/// Create a tag from an integer:
///
/// ```rust
/// let _ = ocaml::Tag(0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Tag(pub sys::tag_t);

impl From<Tag> for sys::tag_t {
    fn from(t: Tag) -> sys::tag_t {
        t.0
    }
}

impl From<sys::tag_t> for Tag {
    fn from(x: sys::tag_t) -> Tag {
        Tag(x)
    }
}

macro_rules! tag_def {
    ($name:ident) => {
        pub const $name: Tag = Tag(sys::$name);
    };
}

#[allow(missing_docs)]
impl Tag {
    tag_def!(FORWARD);
    tag_def!(INFIX);
    tag_def!(OBJECT);
    tag_def!(CLOSURE);
    tag_def!(LAZY);
    tag_def!(ABSTRACT);
    tag_def!(NO_SCAN);
    tag_def!(STRING);
    tag_def!(DOUBLE);
    tag_def!(DOUBLE_ARRAY);
    tag_def!(CUSTOM);
}
