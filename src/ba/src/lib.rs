extern crate core;

//#![feature(associated_type_bounds)]
pub mod buffermanager;
pub mod operator;

pub type InnerPage = u8;
pub type Page = Vec<InnerPage>;
pub type PageIdentifier = u64; // RID equivalent
pub const CHUNK_SIZE: usize = 5;
pub const DEFAULT_FILENAME: &str = "../../.gitignore";
pub const BUFFER_SIZE: u64 = 20;

#[derive(Debug)]
pub enum VulcanoRequest{
    ScanAll,
    Inedx(Vec<PageIdentifier>)
}

pub trait Operator<T>{
    fn open()->Self where Self: Sized;
    fn next(&mut self)->Option<T>;
    fn close(&self);
}

pub fn a() -> i32 {
    return 1;
}

