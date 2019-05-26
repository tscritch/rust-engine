mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::rc::Rc;
use std::ops::Deref;
use std::os::raw::c_void;

pub use bindings::Gl as InnerGl;
pub use bindings::*;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const c_void
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}