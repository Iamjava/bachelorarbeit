
mod bufferreader;

pub fn a()->i32{
    return 1;
}

struct Unary<T> {
    inner: Vec<Vec<T>>, // read pages as list of lists (vector at a time)
}

impl<u8: std::cmp::PartialOrd+ Clone> Unary<u8>
{
    fn new(inner: Vec<Vec<u8>>){
        Unary {inner};
    }
    fn get(&self, f: fn(&u8)->bool)->Vec<u8>{
        let mut next_page_vecs :Vec<u8> = Vec::with_capacity(4096);
        for a in &self.inner {
            for b in a {
                if f(b) {
                    next_page_vecs.push((*b).clone());
                }
            }
        }
        next_page_vecs
    }
}

pub fn filter(){
    let next_page_vecs = vec![1;4096];

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter() {

    }
}
