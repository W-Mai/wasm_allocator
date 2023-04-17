#![cfg(target_arch = "wasm32")]

#[cfg(test)]
mod tests {
    use wasm_allocator::Heap;
    use super::*;

    #[global_allocator]
    static ALLOCATOR: Heap = Heap;

    fn main() {
        let mut v = Vec::new();
        for i in 0..100 {
            v.push(i);
        }
    }
}
