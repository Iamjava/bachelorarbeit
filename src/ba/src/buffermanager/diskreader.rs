use std::{fs, path};
use std::os::unix::fs::FileExt;
use crate::{PAGE_SIZE,Page,DEFAULT_FILENAME};

#[derive(Debug)]
pub struct DiskReader {
    ring: rio::Rio,
    file: fs::File,
}


impl Default for DiskReader{
    fn default() -> Self{
        let ring = rio::new().expect("couldnt initialize io_uring");
        let dr = DiskReader::new(ring,DEFAULT_FILENAME);
        dr
    }
}

impl DiskReader {
    fn new(ring: rio::Rio, file_path: impl AsRef<path::Path>) -> DiskReader {
        DiskReader {
            ring: ring,
            file: fs::File::open(file_path.as_ref()).expect("ERROR: File not found, cant initialize DiskReader"),
        }
    }

    pub fn read(&self, data: Vec<u8>, at: u64) -> Page {
        dbg!(&data.len());
        let completion = self.ring.read_at(&self.file, &data, at);
        completion.wait().unwrap();
        dbg!(data)
    }

    //strange behavior if calling read
    pub fn read_classic(&self, at: u64) -> Page {
        let mut data = [0;10];
        let _read = self.file.read_at(&mut data,at).unwrap();
        data.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use crate::buffermanager::diskreader::DiskReader;
    use crate::{DEFAULT_FILENAME, };

    #[test]
    fn test_diskreader_multithread() {
        let data = vec![0; 40];
        let data2 = vec![0; 40];
        //dr.read(data,10,);

        let a = thread::spawn(move || {
            let dr = DiskReader::default();
            let data = dr.read(data,0);
            dbg!(&dr.file);
            assert!(data.len()>0);
            println!("a read'{}'", String::from_utf8_lossy(&data))
        });

        let b = thread::spawn(move || {
            let ring = rio::new().expect("couldnt initialize io_uring");
            let dr = DiskReader::new(ring,DEFAULT_FILENAME);
            let data = dr.read(data2, 0);
            assert!(data.len()>0);
            println!("b read'{}'", String::from_utf8_lossy(&data))
        });

        let _adata = a.join().unwrap();
        let _bdata = b.join().unwrap();
    }

    #[test]
    fn test_default(){
        let b = thread::spawn(move || {
            let dr = DiskReader::default();
            let data = dr.read_classic(0);
            assert!(!data.is_empty())
        });
        let _ = b.join();
    }

    #[test]
    fn test_thread(){
        let data = vec![0; 40];
        let a = thread::spawn(move || {
            let dr = DiskReader::default();
            let data = dr.read(data,0);
            dbg!(&dr.file);
            assert!(data.len()>0);
            println!("a read'{}'", String::from_utf8_lossy(&data))
        });
        let _ = a.join();
    }
}
