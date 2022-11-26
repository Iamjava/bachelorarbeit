use crate::buffermanager::Buffermanager;
use crate::{Operator};
use crate::{Page, CHUNK_SIZE, PageIdentifier};

pub struct Filter<T> {
    inner_test: Vec<Page>,
    child: T,
    bm: Buffermanager,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl<T: Operator<Page>> Filter<T>
{
    fn new(inner: Vec<Page>) -> Self {
        Filter {
            inner_test: inner,
            state: 0,
            bm: Buffermanager::new(),
            child: T::open(),
        }
    }

    fn get_next_vectorized<P>(&mut self, p: P) -> Option<Vec<Page>> where P: Fn(&&u8) -> bool {
        // vielleicht über iter_array_chunks nachdenken.
        let ret = self.inner_test.iter()
            .flatten()
            .filter(p)
            .copied()
            .collect::<Vec<u8>>()
            .chunks(CHUNK_SIZE)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<u8>>>();
        Some(ret)
    }

    fn get_next(&mut self, predicate: fn(&u8) -> bool) -> Option<Page> {
        let mut counter: usize = self.state;
        let mut last_item = 0;
        let mut next_page_vecs: Vec<u8> = Vec::with_capacity(CHUNK_SIZE);

        let inner = self.inner_test.clone();
            let flat_inner: Vec<&u8> = inner.iter().flatten().collect();
        while next_page_vecs.len() < CHUNK_SIZE {
            let item = flat_inner[counter];
            if predicate(item) {
                next_page_vecs.push((*item).clone());
                last_item = counter;
            }
            counter = counter + 1;
            if counter == flat_inner.len()-1{
               self.inner_test = vec![self.child.next()?];
                counter = 0;
                last_item = 0;
            }
        }

        self.state = last_item + 1;
        Some(next_page_vecs)
    }
}

impl<T:  Operator<Page>> Operator<Page> for Filter<T> {
    fn open() -> Self {
        todo!()
    }
    fn next(&mut self) -> Option<Page> {
        todo!()
    }
    fn close(&self) {
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::operator::filter::*;

    #[test]
    fn test_filter_2() {
        let vecs: Vec<Page> = vec![vec![1; CHUNK_SIZE], vec![5; CHUNK_SIZE +3], vec![4; CHUNK_SIZE +3], vec![6; CHUNK_SIZE +1]];
        let vecs :Vec<Page>= vec![vec![1; CHUNK_SIZE]];
        let mut unary: Filter<Buffermanager> = Filter::new(vecs);
        assert!(&unary.get_next_vectorized(|a| a > &&3).is_some());
    }

    #[test]
    fn test_filter() {
        let vecs: Vec<Page> = vec![vec![1; CHUNK_SIZE], vec![5; CHUNK_SIZE -3], vec![4; CHUNK_SIZE -3], vec![6; CHUNK_SIZE -5]];

        let mut unary: Filter<Buffermanager> = Filter::new(vecs);
        assert!( &unary.get_next(|a| a > &3).is_some());
        assert!( &unary.get_next(|a| a > &3).is_some());
        assert!( &unary.get_next(|a| a > &3).is_some());
    }
}
