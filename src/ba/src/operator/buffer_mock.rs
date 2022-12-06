use crate::{Chunk, CHUNK_SIZE, Operator, Page, InnerPage, TupleType, TupleChunk, DynTuple};
use std::default::Default;
use crate::TupleType::{TBool, TFloat, TInt};

pub struct BufferMock {
    inner: Vec<Chunk<DynTuple>>,
    state: u32,
}

impl BufferMock {
    pub fn new(inner: Vec<TupleChunk>) -> Self{
        BufferMock {
            inner,
            state: 0,
        }
    }
}


impl Default for BufferMock{
    fn default() -> Self{
        BufferMock::new(vec![vec![vec![TInt(1);3]; CHUNK_SIZE],vec![vec![TInt(2);3]; CHUNK_SIZE],vec![vec![TInt(3);3]; CHUNK_SIZE]])
    }
}

impl Operator for BufferMock{
    fn open() -> Self where Self: Sized {
        todo!()
    }

    fn next(&mut self) -> Option<TupleChunk> {
        if self.state >= self.inner.iter().flatten().count().try_into().unwrap() {
            return None;
        }

        let page = self.inner.iter().flatten()
            .skip(self.state.try_into().unwrap())
            .take(CHUNK_SIZE)
            .cloned()
            .collect();
        self.state = self.state + CHUNK_SIZE as u32;
        &self.state;
        Some(page)
    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::CHUNK_SIZE;
    use crate::Operator;
    use crate::operator::buffer_mock::BufferMock;
    use crate::TupleType::TBool;

    #[test]
    fn test_next_TupleType() {
        let mut mm = BufferMock::default();
        let mut mm = BufferMock::default();
        let mut mm = BufferMock::default();
        let a = mm.next();
        let b = mm.next();
        dbg!(&b);

        assert!(a.is_some());
    }
    #[test]
    fn test_next() {
        let mut mm = BufferMock::default();
        let a = mm.next();
        assert!(a.is_some());
    }
}
