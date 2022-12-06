use std::collections::hash_map;
use crate::{CHUNK_SIZE, DynTuple, Operator, TupleChunk};
use std::collections::hash_map::HashMap;
use std::hash::Hash;



pub struct HashJoin< O: Operator, P:Operator, K: Eq + Hash> {
    left_child: O,
    right_child: P,
    is_built: bool,
    hmap: hash_map::HashMap<K,DynTuple>,
    state: u32,
    join_on_a: fn(&DynTuple)->K,
    join_on_b: fn(&DynTuple)->K,
    combine: fn(&DynTuple,&DynTuple)->DynTuple,
    inner: Vec<DynTuple>,
}

impl<O: Operator, P: Operator, K: Eq + Hash> HashJoin<O,P,K>
{
    fn new(left_child: O, right_child: P, fn_left_to_join_attribute: fn(&DynTuple) ->K, fn_right_to_join_attribute: fn(&DynTuple) ->K, combine_fn: fn(&DynTuple, &DynTuple) ->DynTuple) -> Self {
        Self{
            left_child,
            right_child,
            is_built:false,
            hmap: HashMap::<K,DynTuple>::new(),
            join_on_a: fn_left_to_join_attribute,
            join_on_b: fn_right_to_join_attribute,
            state: 0,
            combine: combine_fn,
            inner: Vec::<DynTuple>::new(),
        }
    }

    fn build(& mut self){
       while let Some(next_page) = self.left_child.next(){
           for item in next_page{
               self.hmap.insert((self.join_on_a)(&item),item.clone());
           }
       }
    }

    fn probe(&mut self){
    }
}

impl<O: Operator, P: Operator, K: Eq + Hash> Operator for HashJoin<O,P,K>{
    fn open() -> Self where Self: Sized {
        todo!()
    }

    fn next(&mut self) -> Option<TupleChunk> {
        if self.is_built {
            let mut next_page = Vec::with_capacity(CHUNK_SIZE);
            let mut local_state = self.state as usize;
            let jb = self.join_on_b;
            let combine = self.combine;
            while next_page.len()!=CHUNK_SIZE{
                dbg!(&self.state, &self.inner.len());
                if self.state==self.inner.len().try_into().unwrap(){
                    self.state = 0;
                    local_state=0;
                    self.inner = self.right_child.next().unwrap();
                }
                let item = &self.inner[local_state];
                let key = &(jb)(item);
                if self.hmap.contains_key(key){
                    next_page.push(combine(&self.hmap.get(key).unwrap(),&item))
                }

                local_state = local_state + 1;
                self.state = local_state as u32;
            }
            Some(next_page)
        }else {
            self.build();
            self.probe();
            self.is_built = true;
            self.next()
        }

    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::hashjoin::HashJoin;
    use crate::operator::buffer_mock::BufferMock;
    use crate::operator::scan::Scan;
    use super::*;


    #[test]
    fn test_open(){
        let mut open = HashJoin::new(Scan::default(), Scan::default(), |a|a[0].inner_int() , |a|a[0].inner_int(), |a,b|{let mut a = a.clone();a.append(&mut b.clone());a});
        dbg!(&open.next());
        dbg!(&open.hmap);
    }
}
