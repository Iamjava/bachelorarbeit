use std::collections;
use crate::{Page, PAGE_SIZE, PageIdentifier};
use crate::BUFFER_SIZE;

mod diskreader;

#[derive(Debug)]
struct Buffermanager{
    buffer: collections::HashMap<PageIdentifier,Page>,
    disk_reader: diskreader::DiskReader,
}

impl Buffermanager{
    fn new()->Self{
        Buffermanager{
            buffer: collections::HashMap::with_capacity(BUFFER_SIZE.try_into().unwrap()),
            disk_reader: diskreader::DiskReader::default(),
        }
    }

    fn get_page(&mut self, pageid: PageIdentifier)->&Page{
        if !self.buffer.contains_key(&pageid){
            let read_page = dbg!(self.disk_reader.read_classic(pageid));
            self.buf_insert(pageid,read_page.clone());
        }
        &self.buffer.get(&pageid).unwrap()  //safe since key is part of buffer
    }
    fn buf_insert(&mut self, pageid: PageIdentifier, page: Page){
        self.buffer.insert(pageid,page.clone());
    }

    // eviction stratey here
    fn buf_contains(&mut self, pageid: &PageIdentifier)->bool{
        self.buffer.contains_key(pageid)
    }
}

#[cfg(test)]
mod tests {
    use crate::buffermanager::Buffermanager;
    use crate::buffermanager::diskreader::DiskReader;
    use crate::{BUFFER_SIZE, PAGE_SIZE} ;

    #[test]
    fn test_default(){
        let dr = DiskReader::default();
        let data = dr.read_classic(0);
        dbg!(data);
    }

    #[test]
    fn test_buffermanager_overflow() {
        let mut  bm  = Buffermanager::new();
        for i in 0..BUFFER_SIZE+1{
            bm.buf_insert(i,vec![2 as u8;10]);
        }
    }
    #[test]
    fn test_buffermanager() {
        // let mut bm = Buffermanager::new();
        let mut  bm  = Buffermanager::new();
        bm.get_page(0);
        assert!(bm.buf_contains(&0));
        assert!(!bm.get_page(0).is_empty());
    }
}
