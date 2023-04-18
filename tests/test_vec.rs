extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use wasm_allocator::Heap;
    use super::*;

    #[global_allocator]
    static ALLOCATOR: Heap = Heap;

    #[test]
    fn test_vec() {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
    }
}
