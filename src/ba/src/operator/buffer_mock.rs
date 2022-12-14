use crate::DynValue::*;
use crate::{BufferOperator, Column, DynValue, CHUNK_SIZE};
use std::default::Default;

#[derive(Clone, Debug)]
pub struct BufferMock {
    inner: Vec<Column<DynValue>>,
    state: u32,
}

impl BufferMock {
    pub fn new(inner: Vec<Column<DynValue>>) -> Self {
        BufferMock { inner, state: 0 }
    }
}

impl Default for BufferMock {
    fn default() -> Self {
        BufferMock::new(vec![
            vec![TInt(1), TInt(2), TInt(3), TInt(4)],
            vec![TInt(1), TInt(2), TInt(3), TInt(4)],
            vec![TInt(1), TInt(2), TInt(3), TInt(4)],
            vec![TInt(1), TInt(2), TInt(3), TInt(4)],
            vec![TInt(2); CHUNK_SIZE + 4],
        ])
    }
}

impl BufferOperator for BufferMock {
    fn open() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn next(&mut self) -> Option<Column<DynValue>> {
        if self.state >= self.inner.iter().flatten().count().try_into().unwrap() {
            return None;
        }

        let page = self
            .inner
            .iter()
            .flatten()
            .skip(self.state.try_into().unwrap())
            .take(CHUNK_SIZE)
            .cloned()
            .collect();
        self.state = self.state + CHUNK_SIZE as u32;
        //&self.state;
        Some(page)
    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_next_tt() {
        let mut mm = BufferMock::default();
        let mut _mm2 = BufferMock::default();
        let mut _mm3 = BufferMock::default();
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
