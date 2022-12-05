use crate::{Chunk, CHUNK_SIZE, Operator, FromBytes, Page, InnerPage};
use std::default::Default;

pub struct ScanMock<F> {
    inner: Vec<Vec<F>>,
    state: u32,
}

impl<F: FromBytes + Clone + Default> ScanMock<F> {
    pub fn new(inner: Vec<Vec<F>>) -> Self{
        ScanMock {
            inner,
            state: 0,
        }
    }
}

impl<T: FromBytes> ScanMock<T>{
    fn from_page(item: Page) -> Chunk<T>{
       let ret =  T::to_self(&item);
        ret
    }
}

impl Default for ScanMock<InnerPage>{
    fn default() -> Self{
        ScanMock::new(vec![vec![1 as u8;CHUNK_SIZE],vec![2;CHUNK_SIZE],vec![3;CHUNK_SIZE],vec![5;CHUNK_SIZE],vec![6;CHUNK_SIZE],vec![7;CHUNK_SIZE],])
    }
}

impl<T: Clone> Operator<T> for ScanMock<T> {
    fn open() -> Self where Self: Sized {
        todo!()
    }

    fn next(&mut self) -> Option<Vec<T>> {
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
    use crate::operator::mock_scan::ScanMock;

    #[test]
    fn test_next() {
        let mut mm = ScanMock::new(vec![vec![5; CHUNK_SIZE + 3], vec![4; CHUNK_SIZE - 2], vec![3; CHUNK_SIZE], vec![2; CHUNK_SIZE], vec![6; CHUNK_SIZE + 1]]);
        let a = mm.next();
        let b = mm.next();
        let c = mm.next();
        let d = mm.next();

        assert!(a.is_some());
        assert!(b.is_some());
        assert!(c.is_some());
        assert!(d.is_some());
    }
}
