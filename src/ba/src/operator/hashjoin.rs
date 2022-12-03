use std::collections::hash_map;
use crate::{InnerPage, Operator, Page};
use crate::buffermanager::Buffermanager;
use std::collections::hash_map::HashMap;
use std::hash::Hash;


#[derive(Debug)]
pub struct HashJoin<T: Operator<Page>,K: Eq + Hash> {
    left_child:  T,
    right_child: T,
    is_built: bool,
    hmap: hash_map::HashMap<K,InnerPage>,
    state: u32,
    join_on: fn(InnerPage)->K,
}

impl<T: Operator<Page>,K: Eq + Hash> HashJoin<T,K>
{
    fn new(left_child: T, right_child:T, func: fn(InnerPage) ->K) -> Self {
        Self{
            left_child,
            right_child,
            is_built:false,
            hmap: HashMap::<K,InnerPage>::new(),
            join_on: func,
            state: 0,
        }
    }

    fn build(& mut self){
       while let Some(next_page) = self.left_child.next(){
           for item in next_page{
               self.hmap.insert((self.join_on)(item),item);
           }
       }
    }

    fn probe(&mut self){

    }
}

impl<T: Operator<Page>,K: Eq + Hash> Operator<Page> for HashJoin<T,K>{
    fn open() -> Self where Self: Sized {
        todo!()
    } 
    fn next(&mut self) -> Option<Page> {
        if self.is_built {
            // return next page and set state to next
            todo!()
        }

        self.build();
        self.probe();
        self.is_built = true;
        None
    }

    fn close(&self) {
        todo!()
    }
    
}

#[cfg(test)]
mod tests {
    use crate::operator::hashjoin::HashJoin;
    use crate::operator::mock_scan::ScanMock;
    use crate::Operator;


#[test]
fn test_open(){
    let open = HashJoin::<ScanMock,&str>::new(ScanMock::default(),ScanMock::default(),|_a|"keyy");
    //open.next();
}
}
