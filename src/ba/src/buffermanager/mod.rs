use std::collections;
use crate::{FromBytes, Operator,  PageIdentifier, VulcanoRequest, Page};
use crate::BUFFER_SIZE;
use crate::VulcanoRequest::{Inedx, ScanAll};

mod diskreader;

pub struct Buffermanager<F: FromBytes>{
    buffer: collections::HashMap<PageIdentifier,Page>,
    disk_reader: diskreader::DiskReader,
    workload: VulcanoRequest,
    state: usize,
    produces: fn (&Page)->Vec<F>,
}

impl<T: FromBytes> Buffermanager<T>{
    pub fn new()->Self{
        Buffermanager{
            buffer: collections::HashMap::with_capacity(BUFFER_SIZE.try_into().unwrap()),
            disk_reader: diskreader::DiskReader::default(),
            workload: ScanAll,
            state: 0,
            produces: T::to_self
        }
    }

    pub fn get_page(&mut self, pageid: &PageIdentifier)->Option<&Page>{

        if !self.buffer.contains_key(pageid){
            let read_page = self.disk_reader.read_classic(*pageid)?;
                self.buf_insert(*pageid,read_page.clone());
        }
        self.state = self.state +1;
        Some(&self.buffer.get(pageid).unwrap())  //safe since key is part of buffer
    }

    fn buf_insert(&mut self, pageid: PageIdentifier, page: Page){
        self.buffer.insert(pageid,page.clone());
    }

    // eviction stratey here
    fn buf_contains(&mut self, pageid: &PageIdentifier)->bool{
        self.buffer.contains_key(pageid)
    }
}

impl<F: FromBytes> Operator<F> for Buffermanager<F>{
    fn open() -> Self where Self: Sized{
        Buffermanager::new()
    }

    fn next(&mut self) -> Option<Vec<F>> {
        match &self.workload{
            Inedx(_indicies)=>{ todo!() },
            ScanAll =>{
                let to_read = self.state.clone();
                let prod = self.produces;
                let read = self.get_page(&to_read.try_into().unwrap() )?;
                let res = (prod)(read);
                Some(res)
            }
        }
    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::buffermanager::Buffermanager;
    use crate::buffermanager::diskreader::DiskReader;
    use crate::{BUFFER_SIZE,  InnerPage};
    use crate::Operator;

    #[test]
    fn test_next(){
        let mut  bm  = Buffermanager::<u32>::open();
        let next: Vec<u32>  = bm.next().unwrap();
        print!("{:?}",next)
    }


    #[test]
    fn test_next_fail(){
        let mut  bm  =Buffermanager::<InnerPage>::open();
        for _i in 0..1000{
            let _ = bm.next();
        }
        let next  = bm.next();
        assert!(bm.state!=0);
        assert_eq!(next,None)

    }

    #[test]
    fn test_default(){
        let dr = DiskReader::default();
        let data = dr.read_classic(0);
        dbg!(data);
    }

    #[test]
    fn test_buffermanager_overflow() {
        let mut  bm  = Buffermanager::<InnerPage>::new();
        for i in 0..BUFFER_SIZE+1{
            bm.buf_insert(i,vec![2 as u8;10]);
        }
    }
    #[test]
    fn test_buffermanager() {
        // let mut bm = Buffermanager::new();
        let mut  bm  = Buffermanager::<InnerPage>::new();
        bm.get_page(&0);
        assert!(bm.buf_contains(&0));
        assert!(!bm.get_page(&0).expect("ERROR BUFFERMANAGER").is_empty());
    }
}
