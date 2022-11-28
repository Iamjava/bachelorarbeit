use crate::{Operator};
use crate::{Page, CHUNK_SIZE,};

pub struct Filter<T: Operator<Page>> {
    inner: Page,
    child:  T,
    // read pages as list of lists (vector at a time)
    state: usize,
    predicate: fn(&u8)->bool,
}

impl<T: Operator<Page>> Filter<T>
{

    fn instanciate(operator: T, predicate: fn(&u8) ->bool) -> Option<Self> {
        let mut filter = Filter {
            state: 0,
            inner: vec![],
            child: operator,
            predicate: predicate
        };

        filter.inner = filter.child.next().unwrap();
        Some(filter)
    }

    /*
    fn get_next_vectorized<P>(&mut self, p: P) -> Option<Page> where P: Fn(&&u8) -> bool {
        // vielleicht über iter_array_chunks nachdenken.
        let ret = self.inner_test.iter()
            .filter(p)
            .copied()
            .collect::<Vec<u8>>()
            .chunks(CHUNK_SIZE)
            .collect::<Vec<u8>>();
        Some(ret)
    }
*/
    fn get_next(&mut self, predicate: fn(&u8) -> bool) -> Option<Page> {
        let mut local_state: usize = self.state;
        let mut last_item = 0;
        let mut next_page_vecs: Vec<u8> = Vec::with_capacity(CHUNK_SIZE);

        while next_page_vecs.len() < CHUNK_SIZE {
            if local_state == self.inner.len(){
                match self.child.next(){
                    Some(a)=> self.inner=a,
                    None=>{
                        let to_take= (CHUNK_SIZE-next_page_vecs.len()).try_into().unwrap();
                        next_page_vecs.extend(vec![0;to_take]);
                        return Some(next_page_vecs);
                    }
                }
                local_state = 0;
                last_item = 0;
                self.state=0;
            }
            let item = self.inner[local_state];

            if predicate(&item) {
                next_page_vecs.push((item).clone());
                last_item = local_state;
            }

            local_state = local_state + 1;
        }
        self.state = last_item + 1;
        Some(next_page_vecs)
    }
}

impl<T: Operator<Page>> Operator<Page> for Filter<T> {
    fn open() -> Self {
        todo!()
    }
    fn next(&mut self) -> Option<Page> {
        self.get_next(self.predicate)
    }
    fn close(&self) {
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::operator::mock_scan::ScanMock;
    use crate::operator::filter::*;

    #[test]
    fn test_filter_2() {
        let bm = ScanMock::new( vec![vec![3; CHUNK_SIZE], vec![4; CHUNK_SIZE-3]]);
        let mut filter1 = Filter::instanciate( bm, |x|x>&2).unwrap();
        filter1.next();
        filter1.next();
        let last= filter1.next();
        assert!(last.is_some())
    }

    #[test]
    fn test_double_filter() {
        let bm = ScanMock::new( vec![vec![3; CHUNK_SIZE], vec![4; CHUNK_SIZE+2]]);
        let  filter1 = Filter::instanciate( bm, |x|x>&0).unwrap();
        let mut filter2 = Filter::instanciate(filter1, |x|x>&2).unwrap();
        filter2.next();
        filter2.next();
        filter2.next();
        assert!( filter2.next().is_some());
    }

    /*
    #[test]
    fn test_filter() {
        let vecs: Vec<Page> = vec![vec![1; CHUNK_SIZE], vec![5; CHUNK_SIZE -3], vec![4; CHUNK_SIZE -3], vec![6; CHUNK_SIZE -5]];

        let mut unary: Filter = Filter::new_test(vecs,Buffermanager::new(),|x|x>&3);
        assert!( &unary.get_next(|a| a > &3).is_some());
        assert!( &unary.get_next(|a| a > &3).is_some());
        assert!( &unary.get_next(|a| a > &3).is_some());
    }
     */
}
