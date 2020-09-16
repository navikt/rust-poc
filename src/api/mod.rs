use crate::Blueprint;

mod internal;
mod root;

pub struct Api {
    pub root: Blueprint,
    pub internal: Blueprint
}

impl Api {
    pub fn new() -> Api {
        Api {
            root: root::blueprint(),
            internal: internal::blueprint()
        }
    }
}