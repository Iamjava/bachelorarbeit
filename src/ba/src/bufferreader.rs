use std::io::{Read, Seek, SeekFrom};
use rand::Rng;
use std::{
    fs::OpenOptions,
    io::{Result},
    os::unix::fs::OpenOptionsExt,
};
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use iou::registrar::{RegisteredFd};

async fn test_async_rio_oo(read_positions:impl std::iter::IntoIterator<Item=u64>, file_name: &'static str, _worker_fn: fn(&Vec<u8>)) {
    let ring = Arc::new(rio::new().expect("create uring"));
    let data:[u8;4096] = [0;4096];
    let  _results = read_positions.into_iter().map(|x|(x,ring.clone(),data.clone())).map(|(x,ring,mut data)| async move{
        let n = x.clone() as u64;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .custom_flags(0x4000) // equivalent to libc::O_DIRECT
            .open(file_name)
            .expect("open file");
        //let mut data: &mut [u8] = &mut [0; 4096];
        let completion = ring.read_at(&file, &mut data, n);
        completion.await
        // if using async
    });
}

#[cfg(test)]
mod tests {
    use crate::bufferreader::*;
    static FILENAME: &str = "src/hello.txt";


    #[test]
    fn async_test_rio_uring_odirect() {
        let mut rng = rand::thread_rng();
        let tests: Vec<u64> = (0..100000).map(|_| rng.gen_range(0..2022 / 8)).collect();
        test_async_rio_oo(tests, FILENAME, |x|print!("a"));
    }
}
