use crate::{Operator, Page};
use crate::buffermanager::Buffermanager;


#[derive(Debug)]
pub struct Scan {
    _bm: Buffermanager,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl Operator<Page> for Scan{
    fn open() -> Self where Self: Sized{
        Scan {
            _bm: Buffermanager::new(),
            state: 0,
        }
    }

    fn next(&mut self) -> Option<Page> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::scan::Scan;
    use crate::Operator;
    #[test]
    fn test_open(){
        let open = Scan::open();
        assert_eq!(open.state, 0);
    }
}
