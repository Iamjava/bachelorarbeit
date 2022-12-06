use std::os::linux::raw::stat;
use crate::{Chunk, Operator, DynTuple, TupleType, CHUNK_SIZE};
use crate::buffermanager::Buffermanager;
use crate::operator::buffer_mock::BufferMock;
use crate::TupleType::*;

pub struct Scan<O: Operator>{
    sources: Vec<O>,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl Default for Scan<BufferMock>{
    fn default() -> Self {
        let m1 = BufferMock::default();
        let m2  = BufferMock::default();
        let m3  = BufferMock::default();

        Scan{
            sources: vec![m1,m2,m3],
            state: 0,
        }
    }
}

impl<O: Operator> Operator for Scan<O>{
    fn open() -> Self where Self: Sized{
        todo!()
    }

    fn next(&mut self) -> Option<Chunk<DynTuple>> {
        let mut next = Vec::with_capacity(CHUNK_SIZE);
        // Multithreading

        if self.state
        for s in &mut self.sources{
            let n= s.next().unwrap();
            let lower: usize = self.state as usize;
            let upper = lower + CHUNK_SIZE;
            let vecs = n[lower..upper].to_vec();
            next.push(vecs);
        }
        self.state = self.state + CHUNK_SIZE;
        self.next()
    }


    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::scan::Scan;
    use crate::Operator;
    #[test]
    fn test_open(){
        let open = Scan::default();
        assert_eq!(open.state, 0);
    }
}
