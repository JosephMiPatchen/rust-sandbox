# Rust Standard Types Reference

A comprehensive overview of Rust's standard library types.

## Primitive Types

### Integer Types (Signed)
- `i8` - 8-bit signed integer (-128 to 127)
- `i16` - 16-bit signed integer (-32,768 to 32,767)
- `i32` - 32-bit signed integer (default integer type)
- `i64` - 64-bit signed integer
- `i128` - 128-bit signed integer
- `isize` - pointer-sized signed integer (32 or 64 bits depending on architecture)

### Integer Types (Unsigned)
- `u8` - 8-bit unsigned integer (0 to 255)
- `u16` - 16-bit unsigned integer (0 to 65,535)
- `u32` - 32-bit unsigned integer
- `u64` - 64-bit unsigned integer
- `u128` - 128-bit unsigned integer
- `usize` - pointer-sized unsigned integer (used for indexing)

### Floating Point Types
- `f32` - 32-bit floating point (single precision)
- `f64` - 64-bit floating point (double precision, default float type)

### Boolean Type
- `bool` - true or false

### Character Type
- `char` - Unicode scalar value (4 bytes, can represent any Unicode character)

### Unit Type
- `()` - empty tuple, represents "no value" or "void"

## String Types

### String Types
- `String` - growable, heap-allocated UTF-8 string (owned)
- `&str` - string slice, immutable view into string data (borrowed)
- `OsString` / `&OsStr` - platform-native strings (for OS interactions)
- `CString` / `&CStr` - null-terminated C-compatible strings

## Collection Types

### Sequences
- `Vec<T>` - growable array (dynamic array/list)
- `VecDeque<T>` - double-ended queue
- `LinkedList<T>` - doubly-linked list

### Maps
- `HashMap<K, V>` - hash table (unordered key-value pairs)
- `BTreeMap<K, V>` - ordered map (sorted by keys)

### Sets
- `HashSet<T>` - hash set (unordered unique values)
- `BTreeSet<T>` - ordered set (sorted unique values)

### Other Collections
- `BinaryHeap<T>` - priority queue

## Smart Pointer Types

### Ownership & Borrowing
- `Box<T>` - heap-allocated value (owned pointer)
- `Rc<T>` - reference-counted pointer (shared ownership, single-threaded)
- `Arc<T>` - atomic reference-counted pointer (shared ownership, thread-safe)
- `Cow<'a, T>` - clone-on-write smart pointer

### Interior Mutability
- `Cell<T>` - mutable memory location (for Copy types)
- `RefCell<T>` - mutable memory location with runtime borrow checking
- `Mutex<T>` - mutual exclusion primitive (thread-safe interior mutability)
- `RwLock<T>` - reader-writer lock (multiple readers or one writer)

### References
- `&T` - immutable reference (borrow)
- `&mut T` - mutable reference (exclusive borrow)

## Option and Result Types

### Option Type
```rust
enum Option<T> {
    Some(T),
    None,
}
```
Represents an optional value (may or may not exist).

### Result Type
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Represents success (`Ok`) or failure (`Err`) with error handling.

## Tuple Types
- `(T1, T2, ...)` - fixed-size collection of values of different types
- Examples: `(i32, f64)`, `(String, bool, u8)`

## Array and Slice Types

### Arrays
- `[T; N]` - fixed-size array of N elements of type T
- Example: `[i32; 5]` - array of 5 integers

### Slices
- `&[T]` - immutable view into a sequence
- `&mut [T]` - mutable view into a sequence

## Function Types

### Function Pointers
- `fn(T1, T2) -> R` - function pointer type

### Closures
- `Fn(T1, T2) -> R` - immutable closure
- `FnMut(T1, T2) -> R` - mutable closure
- `FnOnce(T1, T2) -> R` - closure that can be called once

## Range Types
- `Range<T>` - `a..b` (excludes end)
- `RangeInclusive<T>` - `a..=b` (includes end)
- `RangeFrom<T>` - `a..` (unbounded end)
- `RangeTo<T>` - `..b` (unbounded start)
- `RangeFull` - `..` (unbounded both ends)

## Path Types
- `Path` - immutable file system path
- `PathBuf` - owned, mutable file system path

## Time Types
- `Duration` - span of time
- `Instant` - monotonic clock measurement
- `SystemTime` - system clock measurement

## Thread Types
- `Thread` - handle to a thread
- `JoinHandle<T>` - handle to join a spawned thread

## I/O Types
- `File` - file handle
- `Stdin` / `Stdout` / `Stderr` - standard I/O streams
- `BufReader<R>` / `BufWriter<W>` - buffered I/O

## Error Types
- `Error` - trait for error types
- `Box<dyn Error>` - type-erased error (common return type)

## Marker Types
- `PhantomData<T>` - zero-sized type for generic parameters
- `!` - never type (function that never returns)

## Atomic Types
- `AtomicBool` - atomic boolean
- `AtomicI8`, `AtomicI16`, `AtomicI32`, `AtomicI64`, `AtomicIsize`
- `AtomicU8`, `AtomicU16`, `AtomicU32`, `AtomicU64`, `AtomicUsize`
- `AtomicPtr<T>` - atomic raw pointer

## Raw Pointer Types
- `*const T` - raw immutable pointer (unsafe)
- `*mut T` - raw mutable pointer (unsafe)

## Common Type Aliases
- `Result<T>` - often aliased as `Result<T, Box<dyn Error>>`
- `io::Result<T>` - `Result<T, io::Error>`

## Usage in Your Proof-of-Work Code

From your `block.rs`:
```rust
pub struct Block {
    pub index: u64,              // unsigned 64-bit integer
    pub transactions: Vec<Transaction>,  // vector (growable array)
    pub nonce: u64,              // unsigned 64-bit integer
    pub prev_hash: String,       // owned string
    pub is_valid: bool,          // boolean
}
```

## Quick Reference Table

| Category | Common Types | Use Case |
|----------|-------------|----------|
| Integers | `i32`, `u64`, `usize` | Numbers, indices |
| Floats | `f64` | Decimal numbers |
| Text | `String`, `&str` | Text data |
| Collections | `Vec<T>`, `HashMap<K,V>` | Multiple values |
| Optional | `Option<T>` | May or may not exist |
| Errors | `Result<T, E>` | Success or failure |
| Smart Pointers | `Box<T>`, `Rc<T>`, `Arc<T>` | Heap allocation, sharing |

---

**Pro Tip**: When choosing a type, consider:
1. **Ownership** - Do you own the data or borrow it?
2. **Mutability** - Does it need to change?
3. **Size** - How much memory do you need?
4. **Thread-safety** - Will it be shared across threads?
