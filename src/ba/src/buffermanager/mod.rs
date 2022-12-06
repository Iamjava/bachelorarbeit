use std::collections;
use crate::{Operator, PageIdentifier, VulcanoRequest, Page, Chunk, DynTuple, TupleChunk, CHUNK_SIZE};
use crate::BUFFER_SIZE;
use crate::TupleType::TInt;
use crate::VulcanoRequest::{Inedx, ScanAll};

mod diskreader;

pub struct Buffermanager{
    buffer: collections::HashMap<PageIdentifier,Page>,
    disk_reader: diskreader::DiskReader,
    workload: VulcanoRequest,
    state: usize,
    produces: fn (&Page)->TupleChunk,
}

impl Buffermanager{
    pub fn new(produces: fn(&Page)->TupleChunk)->Self{
        Buffermanager{
            buffer: collections::HashMap::with_capacity(BUFFER_SIZE.try_into().unwrap()),
            disk_reader: diskreader::DiskReader::default(),
            workload: ScanAll,
            state: 0,
            produces: produces
        }
    }

    pub fn default()->Self{
        Buffermanager{
            buffer: collections::HashMap::with_capacity(BUFFER_SIZE.try_into().unwrap()),
            disk_reader: diskreader::DiskReader::default(),
            workload: ScanAll,
            state: 0,
            produces: |a| vec![vec![TInt(3);3];CHUNK_SIZE]
        }
    }

    pub fn get_page(&mut self, pageid: &PageIdentifier)->Option<&Page>{

        if !self.buffer.contains_key(pageid){
            let read_page = self.disk_reader.read_classic(*pageid)?;
                self.buf_insert(*pageid,read_page.clone());
        }
        self.state = self.state +1;
        Some(&self.buffer.get(pageid).unwrap())  //safe since key is part of buffer
    }

    fn buf_insert(&mut self, pageid: PageIdentifier, page: Page){
        self.buffer.insert(pageid,page.clone());
    }

    // eviction stratey here
    fn buf_contains(&mut self, pageid: &PageIdentifier)->bool{
        self.buffer.contains_key(pageid)
    }
}

impl Operator for Buffermanager{
    fn open() -> Self where Self: Sized{
        Buffermanager::default()
    }

    fn next(&mut self) -> Option<Chunk<DynTuple>>{
        match &self.workload{
            Inedx(_indicies)=>{ todo!() },
            ScanAll =>{
                let to_read = self.state.clone();
                let prod = self.produces;
                let read = self.get_page(&to_read.try_into().unwrap() )?;
                let res = (prod)(read);
                Some(res)
            }
        }
    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::buffermanager::Buffermanager;
    use crate::buffermanager::diskreader::DiskReader;
    use crate::{BUFFER_SIZE,  InnerPage};
    use crate::Operator;

    #[test]
    fn test_next(){
        let mut  bm  = Buffermanager::open();
        let next   = bm.next();
        print!("{:?}",next.unwrap());
    }

}
