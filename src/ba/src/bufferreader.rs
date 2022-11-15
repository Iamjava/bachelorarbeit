use std::{fs, path};
use std::sync::Arc;
#[derive(Debug)]
struct DiskReader<'a> {
    ring: &'a rio::Rio,
    file: fs::File,
}

impl DiskReader<'_> {
    fn new<'a>(ring: &rio::Rio, file_path: impl AsRef<path::Path>) -> DiskReader<'_> {
        DiskReader {
            ring: ring,
            file: fs::File::open(file_path.as_ref()).expect("ERROR: File not found, cant initialize DiskReader"),
        }
    }

    fn read(&self, data: Vec<u8>, at: u64) -> Vec<u8> {
        let completion = self.ring.read_at(&self.file, &data, at);
        completion.wait().unwrap();
        //println!("{:?}",String::from_utf8_lossy(&data[0..100]));
        data
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use crate::bufferreader::*;

    static FILENAME: &str = "../../.gitignore";

    #[test]
    fn test_diskreader_multithread() {
        let data = vec![0; 10];
        let data2 = vec![0; 10];
        //dr.read(data,10,);

        let a = thread::spawn(move || {
            let ring = Arc::new(rio::new().expect("couldnt initialize io_uring"));
            let binding = ring.clone();
            let dr = DiskReader::new(&binding, FILENAME);
            let data = dr.read(data, 10);
            println!("a {}", String::from_utf8_lossy(&data))
        });

        let b = thread::spawn(move || {
            let ring = Arc::new(rio::new().expect("couldnt initialize io_uring"));
            let binding = ring.clone();
            let dr = DiskReader::new(&binding, FILENAME);
            let data = dr.read(data2, 11);
            println!("b {}", String::from_utf8_lossy(&data))
        });

        let _adata = a.join().unwrap();
        let _bdata = b.join().unwrap();
    }
}
