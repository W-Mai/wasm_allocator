# wasm_allocator

### a simple wasm allocator for `rust`

It is too simple that memory grows linearly and lacks the function of releasing memory (which may be supported in the
future).

* Designed for wasm_unkown_unkown, and it's `no_std`
* Extremely small size, even being optimized by the compiler to eliminate related code
* Extremely simple code, only a few lines can be read through
* Very easy to use, just like the example below

### Example

The default heap size is `1KB`, if you want to modify the size of the heap, provide the `WASM_ALLOCATOR_HEAP_SIZE`
environment variable, such as `WASM_ALLOCATOR_HEAP_SIZE=4096`

Using in your `wasm` project:

```rust
use wasm_allocator::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap;
```

### LICENSE

MIT
