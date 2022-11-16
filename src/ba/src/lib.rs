mod buffermanager;

pub type Page = Vec<u8>;
pub type PageIdentifier = u64;
pub const PAGE_SIZE: usize = 6;
pub const DEFAULT_FILENAME: &str = "../../.gitignore";
pub const BUFFER_SIZE: u64 = 20;

pub fn a() -> i32 {
    return 1;
}

struct Unary {
    inner: Vec<Page>,
    // read pages as list of lists (vector at a time)
    state: usize,
}

impl Unary
{
    fn new(inner: Vec<Page>) -> Unary {
        Unary { inner, state: 0 }
    }

    fn get_next_2<P>(&mut self, p: P) -> Vec<Page> where P: Fn(&&u8) -> bool {
        // vielleicht über iter_array_chunks nachdenken.
        let ret = self.inner.iter()
            .flatten()
            .filter(p)
            .copied()
            .collect::<Vec<u8>>()
            .chunks(PAGE_SIZE)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<u8>>>();
        ret
    }

    fn get_next(&mut self, predicate: fn(&u8) -> bool) -> Page {
        let mut counter: usize = self.state;
        let mut last_item = 0;
        let mut next_page_vecs: Vec<u8> = Vec::with_capacity(PAGE_SIZE);

        let flat_inner: Vec<&u8> = self.inner.iter().flatten().collect();
        while next_page_vecs.len() < PAGE_SIZE {
            let item = flat_inner[counter];
            if predicate(item) {
                next_page_vecs.push((*item).clone());
                last_item = counter;
            }
            counter = counter + 1;
        }

        self.state = last_item + 1;
        next_page_vecs
    }
}


#[cfg(test)]
mod tests {
    use crate::{Page, Unary};
    use crate::PAGE_SIZE;

    #[test]
    fn test_filter_2() {
        let vecs: Vec<Page> = vec![vec![1; PAGE_SIZE], vec![5; PAGE_SIZE+3], vec![4; PAGE_SIZE+3], vec![6; PAGE_SIZE+1]];
        let mut unary = Unary::new(vecs);
        println!("{:?}", &unary.get_next_2(|a| a > &&3));
    }

    #[test]
    fn test_filter() {
        let vecs: Vec<Page> = vec![vec![1; PAGE_SIZE], vec![5; PAGE_SIZE], vec![4; PAGE_SIZE], vec![6; PAGE_SIZE]];

        let mut unary = Unary::new(vecs);
        println!("PAGES");
        println!("{:?}", &unary.get_next(|a| a > &3));
        println!("{:?}", &unary.get_next(|a| a > &3));
        println!("{:?}", &unary.get_next(|a| a > &3));
    }
}
