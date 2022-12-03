#![feature(trait_alias)]
use crate::{Chunk, CHUNK_SIZE, Operator, FromBytes, Page};
use std::default::Default;

pub struct ScanMock<T> {
    inner: Vec<T>,
    state: u32,
}

impl ScanMock<Page> {
    pub fn new(inner: Vec<Page>) -> Self{
        ScanMock {
            inner,
            state: 0,
        }
    }
}

impl<T: FromBytes> ScanMock<T>{
    fn from_page(item: Vec<i32>) -> Chunk<T>{
       let ret =  T::toSelf();
        ret
    }
}

impl Default for ScanMock<Page>{
    fn default() -> Self{
        ScanMock::new(vec![vec![1;CHUNK_SIZE],vec![2;CHUNK_SIZE],vec![3;CHUNK_SIZE],vec![5;CHUNK_SIZE],vec![6;CHUNK_SIZE],vec![7;CHUNK_SIZE],])
    }
}

impl<T> Operator<T> for ScanMock<T> {
    fn open() -> Self where Self: Sized {
        todo!()
    }

    fn next(&mut self) -> Option<Vec<T>> {
        if self.state >= self.inner.len().try_into().unwrap() {
            return None;
        }

        let mut page: Page = self.inner.iter().skip(self.state.try_into().unwrap()).take(CHUNK_SIZE).copied().collect();
        if page.len() < CHUNK_SIZE.try_into().unwrap() {
            while page.len() < CHUNK_SIZE {
                page.push(0);
            }
        }
        self.state = self.state + CHUNK_SIZE as u32;
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
