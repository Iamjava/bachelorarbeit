#![feature(trait_alias)]
extern crate core;

//#![feature(associated_type_bounds)]
pub mod buffermanager;
pub mod operator;

pub type InnerPage = u8;
pub type Page = Vec<InnerPage>;
pub type PageIdentifier = u64; // RID equivalent
pub type Chunk<T>= Vec<T>;
pub const CHUNK_SIZE: usize = 5;
pub const PAGE_SIZE: usize = 5;
pub const DEFAULT_FILENAME: &str = "../../.gitignore";
pub const BUFFER_SIZE: u64 = 20;

#[derive(Debug)]
pub enum VulcanoRequest{
    ScanAll,
    Inedx(Vec<PageIdentifier>)
}

pub trait FromBytes{
    fn size()->i32;
    fn to_self(page: &Page) ->Vec<Self> where Self: Sized;
}

pub trait Operator<F>{
    fn open()->Self where Self: Sized;
    fn next(&mut self)->Option<Vec<F>>;
    fn close(&self);
}

impl FromBytes for i32{
    fn size() -> i32 {
        1
    }
    fn to_self(page: &Page) -> Vec<Self> where Self: Sized {
        let a = page.get(0).unwrap().clone() as i32;
        vec![a;2]

    }
}
impl FromBytes for u32{
    fn size() -> i32 {
        1
    }
    fn to_self(page: &Page) -> Vec<Self> where Self: Sized {
        let a = page.get(0).unwrap().clone() as u32;
        vec![a;2]
    }
}

impl FromBytes for InnerPage{
    fn size() -> i32 { 1 }
    fn to_self(page: &Page) -> Vec<Self> where Self: Sized {
       page.clone()
    }
}
pub fn a() -> i32 {
    return 1;
}

