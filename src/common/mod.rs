pub(crate) mod yarn;
pub(crate) mod syntax_tree;
pub(crate) mod parser;
pub(crate) mod threads;


use std::{collections::HashMap, mem, sync::{Mutex, OnceLock}};

use self::{syntax_tree::ObjDescriptor, yarn::Yarn};

pub(crate) static mut OBJECT_INDEX: OnceLock<Mutex<Vec<Box<ObjDescriptor<'static>>>>> = OnceLock::new();
pub(crate) static mut DEFINED: OnceLock<Mutex<Box<HashMap<yarn::Yarn<'static>, usize>>>> = OnceLock::new();

pub(crate) unsafe fn transmute<'a, T>(src: &'a T) -> &'static T {
    mem::transmute::<&'a T, &'static T>(&src)
}


pub(crate) fn register_object<'a>(obj: ObjDescriptor<'a>) -> Option<()> {
    unsafe {
        OBJECT_INDEX.get_mut()?.lock().unwrap().push(Box::new(obj.immortalize()));
    };
    Some(())
}

pub(crate) fn register_defined<'a>(def: (Yarn<'a>, usize)) -> Option<()> {
    unsafe {
        DEFINED.get_mut()?.lock().unwrap().insert(def.0.immortalize(), def.1);
    };
    Some(())
}

pub(crate) enum Targets {}

pub(crate) struct Constants(usize);

pub(crate) struct Globals {
    target: Targets,
    start: yarn::Yarn<'static>,
    os: Constants
}