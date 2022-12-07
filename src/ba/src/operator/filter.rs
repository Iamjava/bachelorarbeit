use crate::{Chunk, DynTuple, Operator, TupleChunk, DynValue};
use crate::{ CHUNK_SIZE,};

pub struct Filter<O: Operator> {
    inner: Chunk<DynTuple>,
    child: O,
    is_finished:bool,
    // read pages as list of lists (vector at a time)
    state: usize,
    predicate: fn(&DynTuple)->bool,
}

impl<O: Operator> Filter<O>
{
    fn try_new(operator: O, predicate: fn(&DynTuple) ->bool) -> Option<Self> {
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

    fn get_next(&mut self, predicate: fn(&DynTuple) -> bool) -> Option<TupleChunk> {
        if self.is_finished{
            return None
        }

        let mut local_state: usize = self.state;
        let mut next_page_vec = Vec::with_capacity(CHUNK_SIZE);

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
                        let mut to_append = vec![vec![DynValue::Empty; 3]; CHUNK_SIZE- next_page_vec.len()];
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

impl<O: Operator,> Operator for Filter<O> {
    fn open() -> Self {
        todo!()
    }
    fn next(&mut self) -> Option<TupleChunk> {
        self.get_next(self.predicate)
    }
    fn close(&self) {
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::operator::filter::*;
    use crate::operator::scan::Scan;

    #[test]
    fn test_filter_1() {
        let bm = Scan::default();
        let mut filter1 = Filter::try_new(bm, |x|x[0].inner_int()>1).unwrap();
        let last= filter1.next();
        println!("{:?}",last);
        assert!(last.is_some());
        assert!(last.is_some());
    }
}
