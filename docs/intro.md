# Rust Learning Guide for Python/C#/Java Developers

## Overview
This guide is tailored for developers with strong Python, C#, and Java experience learning Rust. We'll leverage your existing knowledge while focusing on Rust's unique concepts.

## Table of Contents
1. [Getting Started](#getting-started)
2. [Core Concepts Comparison](#core-concepts-comparison)
3. [The Big Three: Ownership, Borrowing, References](#the-big-three)
4. [Week-by-Week Learning Path](#learning-path)
5. [Common Pitfalls & Solutions](#common-pitfalls)
6. [Practice Projects](#practice-projects)
7. [Resources](#resources)

---

## Getting Started

### Installation
```bash
# Install Rust via rustup (the official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Your First Program
```rust
fn main() {
    println!("Hello, Rust!");
}
```

Compile and run:
```bash
rustc main.rs
./main

# Or use cargo (recommended)
cargo new hello_rust
cd hello_rust
cargo run
```

---

## Core Concepts Comparison

### Variables & Mutability

**Coming from Python/C#/Java:**
```python
# Python - variables are mutable by default
x = 5
x = 10  # Fine

# Java
int x = 5;
x = 10;  // Fine
```

**In Rust:**
```rust
// Immutable by default (like 'val' in Kotlin or 'const' in JS)
let x = 5;
// x = 10;  // ERROR! Can't reassign

// Explicitly mutable
let mut y = 5;
y = 10;  // OK

// Type annotations (optional when compiler can infer)
let z: i32 = 5;
```

**Why?** Rust encourages immutability for safety and concurrency. You opt-in to mutation.

### Functions

**Comparison:**
```python
# Python
def add(a, b):
    return a + b
```

```rust
// Rust - explicit types required
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value (expression)
    // OR: return a + b;  (statement)
}

fn main() {
    let result = add(5, 10);
    println!("Result: {}", result);
}
```

### Types

| Python | C#/Java | Rust |
|--------|---------|------|
| int | int, long | i8, i16, i32, i64, i128, isize |
| float | float, double | f32, f64 |
| str | string | String, &str |
| bool | bool | bool |
| list | List<T> | Vec<T> |
| dict | Dictionary<K,V> | HashMap<K,V> |
| None | null | Option<T> |

**Important Rust Difference:**
```rust
// Rust has NO null! Instead, we have Option<T>
fn find_user(id: i32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

// Pattern matching to handle
match find_user(1) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}
```

---

## The Big Three: Ownership, Borrowing, References

**This is THE concept that makes Rust unique.** Understanding this unlocks everything.

### 1. Ownership

**The Rules:**
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped (freed)
3. You can transfer ownership (move)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    
    // println!("{}", s1);  // ERROR! s1 no longer valid
    println!("{}", s2);     // OK
}
```

**Compare to languages you know:**
- **Python/Java:** Everything is a reference, garbage collector handles cleanup
- **C#:** Reference types vs value types, GC handles cleanup
- **Rust:** Ownership ensures memory is freed at compile time, no GC needed

### 2. Borrowing (References)

Instead of moving, you can borrow:

```rust
fn main() {
    let s1 = String::from("hello");
    
    // Immutable borrow
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);  // s1 still valid!
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the data (it doesn't own it)
```

**The Borrowing Rules:**
1. You can have unlimited immutable borrows (`&T`) OR one mutable borrow (`&mut T`)
2. But NOT both at the same time
3. References must always be valid (no dangling pointers)

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Mutable borrow
    change(&mut s);
    println!("{}", s);  // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**Why this matters:**
- Prevents data races at compile time
- No need for locks in many concurrent scenarios
- Memory safety without garbage collection

### 3. Lifetimes

Lifetimes ensure references are always valid. Most of the time, Rust infers them:

```rust
// Rust infers lifetimes here
fn first_word(s: &String) -> &str {
    &s[..5]
}

// Sometimes you need to be explicit
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

**Think of it as:** "The returned reference is valid as long as the shortest-lived input reference."

---

## Week-by-Week Learning Path

### Week 1: Foundations
**Goal:** Get comfortable with Rust syntax and tooling

**Topics:**
- Variables, mutability, types
- Functions, control flow (if, loop, while, for)
- Ownership basics (move semantics)
- println! and basic macros

**Practice:**
```rust
// Exercise: FizzBuzz
fn main() {
    for n in 1..=100 {
        match (n % 3, n % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", n),
        }
    }
}
```

**Resources:**
- Read Rust Book Chapters 1-4
- Do Rustlings exercises: variables, functions, if

### Week 2: Ownership & Borrowing
**Goal:** Master the core concept that makes Rust unique

**Topics:**
- Deep dive into ownership
- References and borrowing
- Slices

**Practice:**
```rust
// Exercise: Implement a function that takes a string
// and returns the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

**Resources:**
- Rust Book Chapter 4 (read twice!)
- Rustlings: move_semantics, primitive_types

### Week 3: Structs, Enums & Pattern Matching
**Goal:** Learn Rust's powerful data modeling

**Topics:**
- Structs and methods
- Enums (more powerful than Java enums)
- Pattern matching with `match` and `if let`
- Option<T> and Result<T, E>

**Practice:**
```rust
// Exercise: Build a simple user system
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn new(name: String, email: String, age: u32) -> Self {
        User { name, email, age }
    }
    
    fn can_vote(&self) -> bool {
        self.age >= 18
    }
}

enum LoginResult {
    Success(User),
    InvalidCredentials,
    AccountLocked,
}

fn attempt_login(email: &str) -> LoginResult {
    // Implementation here
    LoginResult::InvalidCredentials
}
```

**Resources:**
- Rust Book Chapters 5-6
- Rustlings: structs, enums, error_handling

### Week 4: Collections & Error Handling
**Goal:** Work with common data structures and handle errors properly

**Topics:**
- Vec<T>, HashMap<K,V>, HashSet<T>
- String vs &str (crucial distinction)
- Error handling with Result<T, E>
- The `?` operator

**Practice:**
```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

// Exercise: Word frequency counter
fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    
    map
}

// Exercise: Read file with error handling
fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**Resources:**
- Rust Book Chapters 8-9
- Rustlings: collections, error_handling, conversions

### Week 5: Traits & Generics
**Goal:** Understand Rust's approach to polymorphism and abstraction

**Topics:**
- Traits (similar to interfaces)
- Trait bounds
- Generics
- Common traits: Display, Debug, Clone, Copy

**Practice:**
```rust
// Exercise: Create a generic summarizable trait
trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_verbose(&self) -> String {
        format!("Summary: {}", self.summarize())
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {} chars", self.title, self.content.len())
    }
}

// Generic function with trait bounds
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

**Resources:**
- Rust Book Chapter 10
- Rustlings: traits, generics

### Week 6: Modules, Crates & Testing
**Goal:** Structure larger projects and write tests

**Topics:**
- Module system (mod, pub, use)
- Cargo workspaces
- Unit tests and integration tests
- Documentation tests

**Practice:**
```rust
// src/lib.rs
pub mod user {
    pub struct User {
        name: String,
    }
    
    impl User {
        pub fn new(name: String) -> Self {
            User { name }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = user::User::new("Alice".to_string());
        assert_eq!(user.name, "Alice");
    }
}
```

**Resources:**
- Rust Book Chapters 7, 11
- Create a multi-module project

### Week 7-8: Advanced Topics
**Goal:** Level up with more powerful Rust features

**Topics:**
- Iterators and closures
- Smart pointers (Box, Rc, Arc)
- Interior mutability (RefCell)
- Concurrency (threads, channels)
- Async/await basics

**Practice:**
```rust
// Exercise: Parallel processing
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}

// Exercise: Use iterators
fn main() {
    let numbers: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    
    println!("{:?}", numbers);
}
```

**Resources:**
- Rust Book Chapters 13, 15-16
- Rustlings: iterators, threads, smart_pointers

---

## Common Pitfalls & Solutions

### Pitfall 1: Fighting the Borrow Checker
**Problem:**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);  // ERROR: can't mutate while borrowed
    println!("{}", first);
}
```

**Solution:**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v[0];  // Copy the value instead of borrowing
    v.push(4);
    println!("{}", first);
}
```

### Pitfall 2: String vs &str Confusion
**Problem:** Strings are tricky coming from other languages

**Solution:**
- `String`: Owned, growable, heap-allocated (like Java's String)
- `&str`: Borrowed string slice, view into string data (like Java's String but read-only)
- Use `&str` for function parameters, `String` when you need ownership

```rust
// Good: accepts any string-like type
fn process(s: &str) {
    println!("{}", s);
}

fn main() {
    let owned = String::from("hello");
    let slice = "world";
    
    process(&owned);  // &String coerces to &str
    process(slice);
}
```

### Pitfall 3: Lifetime Annotation Confusion
**Problem:** Compiler asks for lifetime annotations

**Solution:** Most of the time, you don't need them. When you do:
```rust
// The function returns a reference with the same lifetime as input
fn first<'a>(x: &'a [i32]) -> &'a i32 {
    &x[0]
}
```

### Pitfall 4: Clone vs Copy
**Problem:** Confusion about when data is copied

**Solution:**
- `Copy`: Implicit, cheap bitwise copy (integers, bools, etc.)
- `Clone`: Explicit, potentially expensive deep copy

```rust
let x = 5;
let y = x;  // Copy (x still valid)

let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit clone (both valid)
let s3 = s1;  // Move (s1 no longer valid)
```

---

## Practice Projects

### Beginner Projects
1. **CLI Todo App**: Practice structs, vectors, file I/O
2. **Text Parser**: Work with strings, iterators, error handling
3. **Simple HTTP Client**: Use the `reqwest` crate, async/await

### Intermediate Projects
1. **REST API Server**: Use `actix-web` or `axum`
2. **Database-backed App**: Practice with `sqlx` or `diesel`
3. **Concurrent Web Scraper**: Threads, channels, async

### Advanced Projects
1. **Custom Allocator**: Deep dive into unsafe Rust
2. **Blockchain Implementation**: Complex ownership scenarios
3. **Embedded System**: Use Rust for IoT/embedded devices

---

## Resources

### Essential Reading
1. **The Rust Programming Language** ("The Book"): https://doc.rust-lang.org/book/
   - Official, comprehensive, start here
2. **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
   - Learn by working through examples
3. **Rustlings**: https://github.com/rust-lang/rustlings
   - Interactive exercises

### Video Courses
- **Rust Programming Course** (freeCodeCamp): Excellent intro
- **Jon Gjengset's YouTube**: Advanced Rust concepts

### Practice & Community
- **Exercism Rust Track**: Great exercises with mentoring
- **Advent of Code** in Rust: Annual coding challenges
- **r/rust** on Reddit: Active, helpful community
- **Rust Users Forum**: Official forum

### Quick References
- **Rust Cheat Sheet**: https://cheats.rs/
- **Rust Standard Library Docs**: https://doc.rust-lang.org/std/

---

## Key Mindset Shifts

Coming from Python/C#/Java, embrace these differences:

1. **Immutability by Default**: Opt-in to mutation, not out
2. **No Null**: Use `Option<T>` and handle both cases
3. **Explicit Error Handling**: No exceptions, use `Result<T, E>`
4. **Compiler is Your Friend**: Error messages are incredibly helpful
5. **Zero-Cost Abstractions**: High-level code compiles to fast machine code
6. **Memory Safety Without GC**: Ownership makes this possible

---

## Next Steps

1. **Week 1-2**: Complete Rust Book Chapters 1-4, do basic Rustlings
2. **Week 3-4**: Build a small CLI app (todo list, calculator)
3. **Week 5-6**: Complete Rust Book Chapters 5-10
4. **Week 7-8**: Start an intermediate project (REST API, web scraper)
5. **Ongoing**: Contribute to open source Rust projects

Remember: **The borrow checker is frustrating at first, but it's teaching you to write better code.** Stick with it!

Good luck, and welcome to Rust! 🦀