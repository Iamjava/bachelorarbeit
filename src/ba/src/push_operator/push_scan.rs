use crate::{Chunk, Operator, DynTuple, CHUNK_SIZE,  BufferOperator, DynValue};
use crate::operator::buffer_mock::BufferMock;

#[derive(Clone,Debug)]
pub struct Scan<O: BufferOperator>{
    sources: Vec<O>,
    current_loaded_data: Vec<Vec<DynValue>>,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl<O: BufferOperator + Default + Clone> Scan<O> {
    fn default_sized(n: usize)->Self{
        Scan {
            sources: vec![O::default(); n],
            state: 0,
            current_loaded_data: Vec::with_capacity(CHUNK_SIZE),
        }
    }
}

impl Default for Scan<BufferMock>{
    fn default() -> Self {
        let m1 = BufferMock::default();
        let m2  = BufferMock::default();
        let m3  = BufferMock::default();

        Scan{
            sources: vec![m1,m2,m3],
            state: 0,
            current_loaded_data: Vec::with_capacity(CHUNK_SIZE),
        }
    }
}

impl<O: BufferOperator> Operator for Scan<O>{
    fn open() -> Self where Self: Sized{
        todo!()
    }

    fn next(&mut self) -> Option<Chunk<DynTuple>> {
        // Multithreading
        let mut count  = 0;
        self.current_loaded_data.clear();
        for s in &mut self.sources{
            count = count +1;
            let n= s.next()?;
            //println!("Column {count} {:?}",n);
            let vecs = n;
            self.current_loaded_data.push(vecs)
        }
        let mut transposed = Vec::with_capacity(CHUNK_SIZE);

        self.state = &self.state + CHUNK_SIZE;
        for v in 0..self.current_loaded_data[0].len(){
            let mut one_tuple = Vec::with_capacity(self.sources.len());
            for col in &self.current_loaded_data{
                one_tuple.push(col[v].clone());
            }
            //println!("one_tuple: {:?}",one_tuple);
            transposed.push(one_tuple);
        }
        //println!("{:?}",transposed);
        Some(transposed)
    }


    fn close(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_open_sized(){
        let mut open = Scan::<BufferMock>::default_sized(1);
        assert_eq!(&open.state, &0);
        let _n = open.next().unwrap();
        let _n = open.next().unwrap();
    }
    #[test]
    fn test_open(){
        let mut open = Scan::<BufferMock>::default_sized(3);
        assert_eq!(&open.state, &0);
        let _n = open.next().unwrap();
        let _n = open.next().unwrap();
    }
}
