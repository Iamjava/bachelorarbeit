use std::collections::hash_map;
use crate::{CHUNK_SIZE, FromBytes, Operator};
use std::collections::hash_map::HashMap;
use std::hash::Hash;



pub struct HashJoin<F: FromBytes, G: FromBytes, O: Operator<F>, P:Operator<G>, K: Eq + Hash,C> {
    left_child: O,
    right_child: P,
    is_built: bool,
    hmap: hash_map::HashMap<K,F>,
    state: u32,
    join_on_a: fn(&F)->K,
    join_on_b: fn(&G)->K,
    combine: fn(&F,&G)->C,
    inner: Vec<G>,
}

impl<F: FromBytes + Clone,G: FromBytes + Clone, O: Operator<F>, P: Operator<G>, K: Eq + Hash,C> HashJoin<F,G,O,P,K,C>
{
    fn new(left_child: O, right_child: P, fn_left_to_join_attribute: fn(&F) ->K, fn_right_to_join_attribute: fn(&G) ->K, combine_fn: fn(&F, &G) ->C) -> Self {
        Self{
            left_child,
            right_child,
            is_built:false,
            hmap: HashMap::<K,F>::new(),
            join_on_a: fn_left_to_join_attribute,
            join_on_b: fn_right_to_join_attribute,
            state: 0,
            combine: combine_fn,
            inner: Vec::<G>::new(),
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

impl<F: FromBytes + Clone, G: FromBytes + Clone, O: Operator<F>, P: Operator<G>, K: Eq + Hash,C> Operator<C> for HashJoin<F,G,O,P,K,C>{
    fn open() -> Self where Self: Sized {
        todo!()
    } 
    fn next(&mut self) -> Option<Vec<C>> {
        if self.is_built {
            let mut next_page = Vec::with_capacity(CHUNK_SIZE);
            let mut local_state = self.state as usize;
            let jb = self.join_on_b;
            let combine = self.combine;
            while next_page.len()<CHUNK_SIZE{
                dbg!(&self.state);

                if self.state==self.inner.len().try_into().unwrap(){
                    self.state = 0;
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
    use crate::operator::mock_scan::ScanMock;
    use super::*;


    #[test]
    fn test_open(){
        let mut open = HashJoin::<u8,u8,ScanMock<u8>,ScanMock<u8>,u8,u8>::new(ScanMock::default(), ScanMock::default(), |a| *a+1, |a|*a, |a,b|*a+*b+100);
        dbg!(&open.next());
        dbg!(&open.hmap);
    }
}
