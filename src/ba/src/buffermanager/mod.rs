use std::cell::RefCell;
use std::collections;
use crate::{ Page, PageIdentifier};
use std::rc::Rc;
use crate::BUFFER_SIZE;

mod diskreader;

#[derive(Debug)]
struct Buffermanager{
    buffer: collections::HashMap<PageIdentifier,Rc<RefCell<Page>>>,
    disk_reader: diskreader::DiskReader,
}

impl Buffermanager{
    fn new()->Self{
        Buffermanager{
            buffer: collections::HashMap::with_capacity(BUFFER_SIZE.try_into().unwrap()),
            disk_reader: diskreader::DiskReader::default(),
        }
    }

    fn get_page(&mut self, pageid: PageIdentifier)->Page{
        if self.buffer.contains_key(&pageid){
            self.buffer.get(&pageid).unwrap().take()  //safe since key is part of buffer
        }else {
            let read_page = self.disk_reader.allocate_and_read(pageid);
            self.buffer.insert(pageid,Rc::new(RefCell::new(read_page.clone())));
            dbg!(read_page)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffermanager::Buffermanager;
    use crate::buffermanager::diskreader::DiskReader;

    #[test]
    fn test_default(){
        let dr = DiskReader::default();
        let data = dr.allocate_and_read(0);
        dbg!(data);
    }
    #[test]
    fn test_buffermanager() {
        // let mut bm = Buffermanager::new();
        let data = vec![0;100];
        let dr = DiskReader::default();
        let page = dr.read(data,0);
        //let page = bm.get_page(0);
        assert!(!page.is_empty());
    }
}
