use crate::{FromBytes, Operator};
use crate::{ CHUNK_SIZE,};

pub struct Filter<F: FromBytes,O: Operator<F>> {
    inner: Vec<F>,
    child: O,
    is_finished:bool,
    // read pages as list of lists (vector at a time)
    state: usize,
    predicate: fn(&F)->bool,
}

impl<F: FromBytes + Clone + Default,O: Operator<F>> Filter<F,O>
{
    fn try_new(operator: O, predicate: fn(&F) ->bool) -> Option<Self> {
        let mut filter = Filter {
            state: 0,
            is_finished: false,
            inner: vec![],
            child: operator,
            predicate: predicate
        };
        filter.inner = filter.child.next().unwrap();
        Some(filter)
    }

    fn get_next(&mut self, predicate: fn(&F) -> bool) -> Option<Vec<F>> {
        if self.is_finished{
            return None
        }

        let mut local_state: usize = self.state;
        let mut next_page_vec: Vec<F> = Vec::with_capacity(CHUNK_SIZE);

        while next_page_vec.len() < CHUNK_SIZE {
            let item = &self.inner[local_state];
            if predicate(item){
                next_page_vec.push(item.clone());
            }
            local_state = local_state +1;

            if local_state == self.inner.len(){
                let new_page = self.child.next();
                if new_page.is_some(){
                    self.inner = new_page.unwrap();
                    local_state = 0;
                    self.state = 0;
                }else {
                    self.is_finished = true;
                    if next_page_vec.len() == CHUNK_SIZE{
                        return Some(next_page_vec);
                    }else if next_page_vec.len() ==0 {
                        return None;
                    }else{
                        let mut to_append = vec![F::default(); CHUNK_SIZE- next_page_vec.len()];
                        next_page_vec.append(& mut to_append);
                        return Some(next_page_vec);
                    }
                }
            }
        }
        assert_eq!(next_page_vec.len(), CHUNK_SIZE);
        Some(next_page_vec)
    }
}

impl<F:FromBytes + Clone + Default,O: Operator<F>,> Operator<F> for Filter<F,O> {
    fn open() -> Self {
        todo!()
    }
    fn next(&mut self) -> Option<Vec<F>> {
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
    fn test_filter_1() {
        let bm = ScanMock::<InnerPage>::new( vec![vec![3; 1]]);
        let mut filter1 = Filter::try_new(bm, |x|x>&2).unwrap();
        let last= filter1.next();
        assert!(last.is_some());
        assert!(filter1.next().is_none());
    }

    #[test]
    fn test_filter_2() {
        let bm = ScanMock::new( vec![vec![3; CHUNK_SIZE], vec![4; CHUNK_SIZE-3]]);
        let mut filter1 = Filter::try_new(bm, |x|x>&2).unwrap();
        filter1.next();
        let last= filter1.next();
        assert!(last.is_some())
    }

    #[test]
    fn test_double_filter() {
        let bm = ScanMock::new( vec![vec![3; 1]]);
        let filter1 = Filter::try_new(bm, |x|x>=&2).unwrap();
        let mut filter2 = Filter::try_new(filter1, |x|x>&10).unwrap();
        let a = filter2.next();
        assert!( a.is_none());
    }

    #[test]
    fn test_double_filter_1_5() {
        let bm = ScanMock::new( vec![vec![122; CHUNK_SIZE],vec![4;CHUNK_SIZE],vec![5;CHUNK_SIZE]]);
        let filter1 = Filter::try_new(bm, |x|x>=&2).unwrap();
        let mut filter2 = Filter::try_new(filter1, |x|x>&4).unwrap();
        let a = filter2.next();
        filter2.next();
        assert!( a.is_some());
    }
    #[test]
    fn test_double_filter_2() {
        let bm = ScanMock::new( vec![vec![122; CHUNK_SIZE-1],vec![4;CHUNK_SIZE],vec![5;CHUNK_SIZE]]);
        let filter1 = Filter::try_new(bm, |x|x>=&2).unwrap();
        let mut filter2 = Filter::try_new(filter1, |x|x>&4).unwrap();
        let a = filter2.next();
        filter2.next();
        assert!( a.is_some());
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
