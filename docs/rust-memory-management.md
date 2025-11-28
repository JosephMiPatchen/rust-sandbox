# Rust Memory Management: Ownership, Borrowing, and Best Practices

Rust's memory management is unique—it guarantees memory safety without a garbage collector. This guide explains how to manage memory correctly in Rust.

## The Three Core Concepts

1. **Ownership** - Every value has one owner
2. **Borrowing** - Temporarily access values without taking ownership
3. **Lifetimes** - How long references are valid

## Ownership Rules

Rust's ownership system has three fundamental rules:

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped (freed)
3. You can transfer ownership (move) or lend it out (borrow)

```rust
fn main() {
    let s = String::from("hello");  // s owns the String
    // When s goes out of scope here, String is dropped
}
```

## When to Move (Transfer Ownership)

**Move** when you want to transfer ownership permanently.

### Use Cases:
- Passing data to a function that consumes it
- Returning values from functions
- Storing values in collections
- When you won't need the original value anymore

```rust
fn process_string(s: String) {
    println!("{}", s);
    // s is dropped here
}

fn main() {
    let text = String::from("hello");
    process_string(text);  // Ownership moved to function
    // println!("{}", text);  // ❌ ERROR: text no longer valid
}
```

### Example: Moving into a Collection

```rust
let mut names = Vec::new();
let name = String::from("Alice");
names.push(name);  // Ownership moved into vector
// println!("{}", name);  // ❌ ERROR: name was moved
```

## When to Borrow (Immutable Reference `&`)

**Borrow immutably** when you need to read data without modifying it.

### Use Cases:
- Reading values without consuming them
- Passing data to functions that only need to read
- Multiple parts of code need to read the same data simultaneously
- Most function parameters should be immutable borrows

```rust
fn print_string(s: &String) {
    println!("{}", s);
    // s is just borrowed, not owned
}

fn main() {
    let text = String::from("hello");
    print_string(&text);  // Borrow text
    print_string(&text);  // Can borrow multiple times
    println!("{}", text); // ✅ Still valid!
}
```

### Multiple Immutable Borrows

You can have **unlimited** immutable borrows at the same time:

```rust
let data = String::from("hello");
let ref1 = &data;
let ref2 = &data;
let ref3 = &data;
// All valid simultaneously!
println!("{} {} {}", ref1, ref2, ref3);
```

## When to Borrow Mutably (`&mut`)

**Borrow mutably** when you need to modify data without taking ownership.

### Use Cases:
- Modifying a value temporarily
- Updating struct fields
- In-place modifications
- When the caller needs the value back after modification

```rust
fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut text = String::from("hello");
    add_exclamation(&mut text);  // Borrow mutably
    println!("{}", text);         // "hello!"
}
```

### The Mutable Borrow Rule

**Critical:** You can have **only ONE** mutable borrow at a time, and **no immutable borrows** can exist simultaneously.

```rust
let mut data = String::from("hello");

let ref1 = &mut data;
// let ref2 = &mut data;  // ❌ ERROR: Can't have two mutable borrows
// let ref3 = &data;      // ❌ ERROR: Can't mix mutable and immutable

ref1.push_str(" world");
println!("{}", ref1);
```

### Why This Rule Exists

Prevents data races at compile time:

```rust
let mut numbers = vec![1, 2, 3];

// ❌ This would be dangerous (prevented by Rust):
// let first = &numbers[0];        // Immutable borrow
// numbers.push(4);                // Mutable operation
// println!("{}", first);          // first might be invalid!
```

## Choosing the Right Approach: Decision Tree

```
Do you need to modify the value?
│
├─ NO → Do you need the value after the function call?
│       │
│       ├─ YES → Use immutable borrow (&T)
│       │        Example: fn read(data: &String)
│       │
│       └─ NO → Move or borrow (usually borrow is better)
│                Example: fn consume(data: String) or fn read(data: &String)
│
└─ YES → Does the caller need the value back?
         │
         ├─ YES → Use mutable borrow (&mut T)
         │        Example: fn modify(data: &mut String)
         │
         └─ NO → Move and return new value, or use mutable borrow
                  Example: fn transform(data: String) -> String
```

## Common Patterns

### Pattern 1: Read-Only Access

```rust
fn get_length(s: &String) -> usize {
    s.len()  // Just reading, not modifying
}

let text = String::from("hello");
let len = get_length(&text);  // Borrow
println!("{} has length {}", text, len);  // text still valid
```

### Pattern 2: Modify in Place

```rust
fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase();
}

let mut text = String::from("hello");
make_uppercase(&mut text);
println!("{}", text);  // "HELLO"
```

### Pattern 3: Transform and Return

```rust
fn add_prefix(s: String) -> String {
    format!("PREFIX: {}", s)
}

let text = String::from("hello");
let new_text = add_prefix(text);  // text moved, new value returned
println!("{}", new_text);  // "PREFIX: hello"
```

### Pattern 4: Builder Pattern (Consuming Self)

```rust
struct Config {
    name: String,
    value: i32,
}

impl Config {
    fn new() -> Self {
        Config {
            name: String::new(),
            value: 0,
        }
    }
    
    fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self  // Return ownership
    }
    
    fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }
}

// Usage
let config = Config::new()
    .with_name("test".to_string())
    .with_value(42);
```

## Clone: When You Need a Copy

Use `.clone()` when you need to keep the original and create a new owned copy.

### When to Clone:
- You need the value in multiple places with independent ownership
- Storing the same data in multiple collections
- When borrowing rules make your code too complex

```rust
let original = String::from("hello");
let copy = original.clone();  // Explicit copy

println!("{}", original);  // ✅ Still valid
println!("{}", copy);      // ✅ Independent copy
```

### Clone is Expensive

Cloning creates a deep copy, which can be slow for large data:

```rust
let big_vec = vec![1; 1_000_000];
let copy = big_vec.clone();  // Copies 1 million elements!
```

**Rule of thumb:** Avoid cloning in hot paths (frequently called code). Prefer borrowing.

## Copy Types: Automatic Copying

Some types implement the `Copy` trait and are copied automatically (no move):

### Copy Types:
- All integers: `i32`, `u64`, etc.
- Floating point: `f32`, `f64`
- Booleans: `bool`
- Characters: `char`
- Tuples of Copy types: `(i32, i32)`
- Arrays of Copy types: `[i32; 3]`

```rust
let x = 5;
let y = x;  // x is copied, not moved
println!("{} {}", x, y);  // ✅ Both valid

let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, not copied
// println!("{}", s1);  // ❌ ERROR: s1 was moved
```

## Reference Lifetimes

References must always be valid. Rust uses lifetimes to ensure this.

### Basic Lifetime Rules:
1. A reference cannot outlive the value it points to
2. References must always point to valid data

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

let string1 = String::from("long string");
let string2 = String::from("short");
let result = longest(&string1, &string2);
println!("{}", result);
```

### Dangling References (Prevented by Rust)

```rust
fn dangle() -> &String {  // ❌ ERROR: Can't return reference to local
    let s = String::from("hello");
    &s  // s will be dropped, reference would be invalid
}

// Fix: Return owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Move ownership out
}
```

## Common Mistakes and Solutions

### Mistake 1: Trying to Use After Move

```rust
// ❌ Wrong
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // ERROR: s was moved

// ✅ Fix 1: Clone if you need both
let s = String::from("hello");
let s2 = s.clone();
println!("{} {}", s, s2);

// ✅ Fix 2: Borrow instead
let s = String::from("hello");
let s2 = &s;
println!("{} {}", s, s2);
```

### Mistake 2: Multiple Mutable Borrows

```rust
// ❌ Wrong
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // ERROR: Can't have two mutable borrows

// ✅ Fix: Use one at a time
let mut v = vec![1, 2, 3];
{
    let r1 = &mut v;
    r1.push(4);
}  // r1 goes out of scope
let r2 = &mut v;
r2.push(5);
```

### Mistake 3: Mixing Mutable and Immutable Borrows

```rust
// ❌ Wrong
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ERROR: Can't borrow mutably while immutably borrowed
println!("{}", r1);

// ✅ Fix: Separate the borrows
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);  // r1 no longer used after this
let r2 = &mut s;     // Now we can borrow mutably
r2.push_str(" world");
```

### Mistake 4: Returning References to Local Variables

```rust
// ❌ Wrong
fn get_string() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s will be dropped
}

// ✅ Fix: Return owned value
fn get_string() -> String {
    String::from("hello")
}
```

## Smart Pointers: Advanced Memory Management

When ownership rules are too restrictive, use smart pointers:

### `Box<T>` - Heap Allocation

Use when you need heap allocation or recursive types:

```rust
let boxed = Box::new(5);
println!("{}", boxed);

// Recursive type (requires Box)
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### `Rc<T>` - Reference Counting

Use when you need multiple owners (single-threaded):

```rust
use std::rc::Rc;

let shared = Rc::new(String::from("hello"));
let ref1 = Rc::clone(&shared);
let ref2 = Rc::clone(&shared);
// All three can access the String
```

### `Arc<T>` - Atomic Reference Counting

Use for multiple owners across threads:

```rust
use std::sync::Arc;
use std::thread;

let shared = Arc::new(String::from("hello"));
let shared_clone = Arc::clone(&shared);

thread::spawn(move || {
    println!("{}", shared_clone);
});
```

### `RefCell<T>` - Interior Mutability

Use when you need to mutate data with immutable references:

```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;  // Mutate through immutable reference
println!("{}", data.borrow());
```

## Performance Considerations

### Stack vs Heap

- **Stack**: Fast, fixed size, automatic cleanup
- **Heap**: Slower, dynamic size, manual management (Rust automates this)

```rust
let x = 5;  // Stack allocated (Copy type)
let s = String::from("hello");  // Heap allocated
```

### Borrowing is Zero-Cost

Borrowing has no runtime overhead:

```rust
fn process(data: &Vec<i32>) {
    // No copying, just a pointer
}

let numbers = vec![1, 2, 3];
process(&numbers);  // Zero cost!
```

### When to Optimize

1. **Start with borrowing** - It's usually the right choice
2. **Clone when needed** - Don't prematurely optimize
3. **Profile first** - Measure before optimizing
4. **Use smart pointers** - When ownership rules are too restrictive

## Best Practices Summary

1. **Default to borrowing** - Use `&T` for read-only, `&mut T` for modifications
2. **Move when consuming** - If a function consumes a value, take ownership
3. **Clone sparingly** - Only when you truly need independent copies
4. **Keep borrows short** - Let references go out of scope quickly
5. **Use Copy types** - For small, simple data
6. **Return owned values** - Don't return references to local variables
7. **One mutable borrow** - Never have multiple mutable references
8. **Read the compiler** - Rust's error messages are excellent teachers

## Quick Reference

| Scenario | Use | Example |
|----------|-----|---------|
| Read only, keep original | `&T` | `fn read(s: &String)` |
| Modify, keep original | `&mut T` | `fn modify(s: &mut String)` |
| Consume value | `T` | `fn consume(s: String)` |
| Need multiple owners | `Rc<T>` or `Arc<T>` | `let shared = Rc::new(data)` |
| Need independent copy | `.clone()` | `let copy = original.clone()` |
| Small data | `Copy` types | `let x = 5; let y = x;` |

## Conclusion

Rust's memory management is about understanding three questions:
1. **Who owns this data?**
2. **Who needs to read it?**
3. **Who needs to modify it?**

Answer these questions, and the right approach becomes clear. The compiler will guide you if you make mistakes—trust it and learn from its messages!
