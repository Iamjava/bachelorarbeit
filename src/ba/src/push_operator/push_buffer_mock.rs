use crate::{ CHUNK_SIZE, BufferOperator,  Column, DynValue, DEFAULT_FILENAME};
use std::default::Default;
use crate::DynValue::*;
use tokio_uring::fs::File;

#[derive(Clone,Debug)]
pub struct PushBufferMock {}

impl PushBufferMock {
    pub fn new(_inner: Vec<Column<DynValue>>) -> Self{
        PushBufferMock {
        }
    }

    pub async fn test_read(){
            // Open a file
            let file = File::open(DEFAULT_FILENAME).await.unwrap();

            let buf = vec![0; 4096];
            let (res, buf) = file.read_at(buf, 0).await;
            let n = res.unwrap();
            println!("{:?}", &buf[..n]);
    }
}

impl Default for PushBufferMock{
    fn default() -> Self{
       PushBufferMock::new(vec![
           vec![TInt(1),TInt(2),TInt(3),TInt(4)],
           vec![TInt(1),TInt(2),TInt(3),TInt(4)],
           vec![TInt(1),TInt(2),TInt(3),TInt(4)],
           vec![TInt(1),TInt(2),TInt(3),TInt(4)],
           vec![TInt(2); CHUNK_SIZE+4]
       ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_next_tt(){
      tokio_uring::start(async {
        PushBufferMock::test_read().await
        })
    }
}
