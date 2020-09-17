use crate::Blueprint;

mod internal;
mod root;

pub fn api() -> Vec<Blueprint> {
    vec![root::blueprint(), internal::blueprint()]
}