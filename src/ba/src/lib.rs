//#![feature(associated_type_bounds)]
use crate::VulcanoRequest::*;

pub mod buffermanager;
pub mod operator;

pub type Page = Vec<u8>;
pub type PageIdentifier = u64; // RID equivalent
pub const CHUNK_SIZE: usize = 4096;
pub const DEFAULT_FILENAME: &str = "../../.gitignore";
pub const BUFFER_SIZE: u64 = 20;


#[derive(Debug)]
pub enum VulcanoRequest{
    SCAN_ALL,
    INDEX(Vec<PageIdentifier>)
}


pub trait Operator<T>{
    fn open()->Self;
    fn next(&mut self)->Option<T>;
    fn close(&self);
}

pub fn a() -> i32 {
    return 1;
}

