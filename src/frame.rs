use crate::*;

/// Garbage collector frame
pub struct Frame {
    local_roots: *mut sys::CamlRootsBlock,
    block: sys::CamlRootsBlock,
    index: usize,
}

impl Frame {
    /// Create a new frame
    pub fn new() -> Frame {
        Frame {
            local_roots: unsafe { sys::local_roots() },
            block: sys::CamlRootsBlock::default(),
            index: 0,
        }
    }

    /// Create a local variable registered with the OCaml garbage collector
    pub fn local(&mut self) -> Value {
        let mut x = Value(sys::UNIT);

        if self.index == 5 {
            self.index = 0;
        }

        if self.index == 0 {
            self.block = sys::CamlRootsBlock::default();
            #[allow(unused_unsafe)]
            unsafe {
                self.block.next = sys::local_roots();
                sys::set_local_roots(&mut self.block);
            };
            self.block.nitems = 1;
        }

        self.block.tables[self.index] = &mut x.0 as *mut _;

        self.index += 1;
        self.block.ntables = self.index;
        x
    }

    /// Ensure an existing value is valid for the given frame
    pub fn local_value(&mut self, v: Value) -> Value {
        let mut x = self.local();
        x.0 = v.0;
        x
    }

    /// Finish the frame
    pub fn end(mut self) {
        if !self.local_roots.is_null() {
            unsafe { sys::set_local_roots(self.local_roots) };
            self.local_roots = std::ptr::null_mut();
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        if self.local_roots.is_null() {
            return;
        }

        unsafe { sys::set_local_roots(self.local_roots) };
    }
}
