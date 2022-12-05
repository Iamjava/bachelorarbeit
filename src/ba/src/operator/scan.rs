use crate::{FromBytes, Operator, };
use crate::buffermanager::Buffermanager;


pub struct Scan<F: FromBytes> {
    _bm: Buffermanager<F>,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl<F: FromBytes> Operator<F> for Scan<F>{
    fn open() -> Self where Self: Sized{
        Scan {
            _bm: Buffermanager::<F>::new(),
            state: 0,
        }
    }

    fn next(&mut self) -> Option<Vec<F>> {
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
        let open = Scan::<i32>::open();
        assert_eq!(open.state, 0);
    }
}
