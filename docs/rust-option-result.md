# Rust Option and Result: Handling Absence and Errors

`Some`, `None`, `Ok`, and `Err` are not operators—they're **enum variants** used to represent optional values and errors in Rust.

## `Option<T>` - Handling Optional Values

`Option` represents a value that might or might not exist. It replaces `null` from other languages.

### The `Option` Enum

```rust
enum Option<T> {
    Some(T),  // Contains a value
    None,     // No value
}
```

### Creating Options

```rust
let some_number = Some(5);           // Has a value
let some_string = Some("hello");     // Has a value
let no_value: Option<i32> = None;    // No value
```

### Why Use `Option`?

In other languages, `null` can cause crashes. Rust forces you to handle the "no value" case:

```rust
// ❌ Other languages (can crash)
// let name = user.name;  // What if name is null?

// ✅ Rust (must handle None)
let name: Option<String> = get_user_name();
match name {
    Some(n) => println!("Name: {}", n),
    None => println!("No name found"),
}
```

### Common `Option` Methods

```rust
let x = Some(5);

// Check if it has a value
x.is_some();  // true
x.is_none();  // false

// Get the value (panics if None)
x.unwrap();  // 5 (dangerous!)

// Get the value or a default
x.unwrap_or(0);  // 5
None.unwrap_or(0);  // 0

// Get the value or compute a default
x.unwrap_or_else(|| 0);

// Transform the value if it exists
x.map(|n| n * 2);  // Some(10)
None.map(|n| n * 2);  // None

// Use with ? operator
fn get_first(list: &[i32]) -> Option<i32> {
    let first = list.get(0)?;  // Returns None if empty
    Some(*first)
}
```

## `Result<T, E>` - Handling Errors

`Result` represents an operation that can succeed or fail.

### The `Result` Enum

```rust
enum Result<T, E> {
    Ok(T),   // Success with value
    Err(E),  // Failure with error
}
```

### Creating Results

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

let success = divide(10, 2);  // Ok(5)
let failure = divide(10, 0);  // Err("Cannot divide by zero")
```

### Handling Results

```rust
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

### Common `Result` Methods

```rust
let result: Result<i32, String> = Ok(5);

// Check if it's Ok or Err
result.is_ok();   // true
result.is_err();  // false

// Get the value (panics if Err)
result.unwrap();  // 5 (dangerous!)

// Get the value or a default
result.unwrap_or(0);  // 5
Err("error").unwrap_or(0);  // 0

// Get the value or panic with custom message
result.expect("Should have a value");

// Transform the Ok value
result.map(|n| n * 2);  // Ok(10)

// Transform the Err value
result.map_err(|e| format!("Error: {}", e));

// Use with ? operator (propagate errors)
fn read_file() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("file.txt")?;
    Ok(contents)
}
```

## The `?` Operator

The `?` operator is shorthand for propagating `None` or `Err`:

### With `Option`

```rust
// Without ?
fn get_first_char(text: Option<String>) -> Option<char> {
    match text {
        Some(s) => s.chars().next(),
        None => None,
    }
}

// With ?
fn get_first_char(text: Option<String>) -> Option<char> {
    let s = text?;  // Returns None if text is None
    s.chars().next()
}
```

### With `Result`

```rust
// Without ?
fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    let contents = match std::fs::read_to_string("number.txt") {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };
    
    let number = match contents.trim().parse() {
        Ok(n) => n,
        Err(e) => return Err(Box::new(e)),
    };
    
    Ok(number)
}

// With ?
fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("number.txt")?;
    let number = contents.trim().parse()?;
    Ok(number)
}
```

## Converting Between `Option` and `Result`

```rust
// Option to Result
let opt = Some(5);
let res: Result<i32, &str> = opt.ok_or("No value");  // Ok(5)

let opt: Option<i32> = None;
let res: Result<i32, &str> = opt.ok_or("No value");  // Err("No value")

// Result to Option
let res: Result<i32, String> = Ok(5);
let opt = res.ok();  // Some(5)

let res: Result<i32, String> = Err("error".to_string());
let opt = res.ok();  // None
```

## Other Common Enum Constructors

### `Ordering` - Comparison Results

```rust
use std::cmp::Ordering;

let result = 5.cmp(&10);

match result {
    Ordering::Less => println!("5 < 10"),
    Ordering::Equal => println!("Equal"),
    Ordering::Greater => println!("5 > 10"),
}
```

### `bool` - True or False

```rust
// bool is actually an enum!
enum bool {
    false,
    true,
}

let is_valid = true;
let is_empty = false;
```

### Custom Enums

You can create your own:

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

let current = Status::Active;

match current {
    Status::Active => println!("Active"),
    Status::Inactive => println!("Inactive"),
    Status::Pending => println!("Pending"),
}
```

## Quick Reference

| Variant | Type | Meaning | Example |
|---------|------|---------|---------|
| `Some(value)` | `Option<T>` | Has a value | `Some(5)` |
| `None` | `Option<T>` | No value | `None` |
| `Ok(value)` | `Result<T, E>` | Success | `Ok(42)` |
| `Err(error)` | `Result<T, E>` | Failure | `Err("error")` |

## Best Practices

1. **Use `Option` for optional values** - Never use `null` or sentinel values
2. **Use `Result` for operations that can fail** - Don't panic, return errors
3. **Prefer `?` over `.unwrap()`** - Propagate errors instead of panicking
4. **Use `.expect()` with good messages** - If you must unwrap, explain why
5. **Use combinators** - `.map()`, `.and_then()`, etc. are more idiomatic than `match`

## Examples

### Example 1: Finding in a List

```rust
fn find_user(id: u32) -> Option<String> {
    let users = vec![
        (1, "Alice"),
        (2, "Bob"),
        (3, "Charlie"),
    ];
    
    users.iter()
        .find(|(user_id, _)| *user_id == id)
        .map(|(_, name)| name.to_string())
}

match find_user(2) {
    Some(name) => println!("Found: {}", name),
    None => println!("User not found"),
}
```

### Example 2: Chaining Operations

```rust
fn process(input: Option<i32>) -> Option<i32> {
    input
        .map(|n| n * 2)           // Double it
        .filter(|n| *n > 10)      // Keep only if > 10
        .map(|n| n + 5)           // Add 5
}

let result = process(Some(3));  // Some(3) -> Some(6) -> None
let result = process(Some(6));  // Some(6) -> Some(12) -> Some(17)
```

### Example 3: Error Handling Chain

```rust
use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

## Summary

- **`Some(value)` and `None`** - Variants of `Option<T>` for optional values
- **`Ok(value)` and `Err(error)`** - Variants of `Result<T, E>` for fallible operations
- **Not operators** - They're enum constructors (like calling a function)
- **Type safe** - Compiler forces you to handle all cases
- **Use `?` operator** - Clean error propagation
- **Avoid `.unwrap()`** - Prefer pattern matching or `?`

These enums are fundamental to Rust's safety guarantees—no null pointer exceptions, no uncaught exceptions!
