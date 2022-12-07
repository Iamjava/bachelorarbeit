extern crate core;

use crate::DynValue::{TBool, TFloat, TInt};

pub mod buffermanager;
pub mod operator;

pub type InnerPage = u8;
pub type Page = Vec<InnerPage>;
pub type PageIdentifier = u64; // RID equivalent
pub type Chunk<T>= Vec<T>;
pub type DynTuple = Vec<DynValue>;
pub type TupleChunk=Chunk<DynTuple>;
pub type Column<T>= Vec<T>;

pub const CHUNK_SIZE: usize = 5;
pub const PAGE_SIZE: usize = 5;
pub const DEFAULT_FILENAME: &str = "../../.gitignore";
pub const BUFFER_SIZE: u64 = 20;

#[derive(Debug)]
pub enum VulcanoRequest{
    ScanAll,
    Inedx(Vec<PageIdentifier>)
}

#[derive(Debug, Clone)]
pub enum DynValue {
    TFloat(f32),
    TBool(bool),
    TInt(u32),
    Empty,
}

impl DynValue {
    pub fn inner_float(&self) ->f32{
       if let TFloat(a) = self {
           a.clone()
       }else {panic!("cast failed")}
    }

    pub fn inner_bool(&self) ->bool{
        if let TBool(a) = self {
            a.clone()
        }else {panic!("cast failed")}
    }

    pub fn inner_int(&self)->u32{
        if let TInt(a) = self {
            a.clone()
        }else {panic!("cast failed")}
    }
}

pub trait Operator{
    fn open()->Self where Self: Sized;
    fn next(&mut self)-> Option<TupleChunk>;
    fn close(&self);
}

pub trait BufferOperator{
    fn open()->Self where Self: Sized;
    fn next(&mut self)-> Option<Column<DynValue>>;
    fn close(&self);
}
pub fn a() -> i32 {
    return 1;
}

