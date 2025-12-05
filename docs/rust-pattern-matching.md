# Rust Pattern Matching: A Powerful Control Flow Tool

Pattern matching in Rust is one of the language's most powerful features. It allows you to destructure data, handle different cases, and write expressive, safe code.

## What is Pattern Matching?

Pattern matching lets you compare a value against a series of patterns and execute code based on which pattern matches. It's like a supercharged `switch` statement that can also destructure data.

## The `match` Expression

The most common way to pattern match is with the `match` keyword.

### Basic Match

```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),  // _ is the catch-all pattern
}
```

### Match Returns a Value

Unlike `switch` in other languages, `match` is an expression that returns a value:

```rust
let number = 2;

let description = match number {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "many",
};

println!("{}", description);  // "two"
```

### Match Must Be Exhaustive

Rust requires you to handle all possible cases:

```rust
let x = 5;

// ❌ ERROR: non-exhaustive patterns
// match x {
//     1 => println!("one"),
//     2 => println!("two"),
// }

// ✅ OK: all cases covered
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),  // Catch-all
}
```

## Matching Enums

Pattern matching shines when working with enums:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quitting");
        }
        Message::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Writing: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

## Matching `Option<T>`

Pattern matching is the idiomatic way to handle `Option`:

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

let result = divide(10, 2);

match result {
    Some(value) => println!("Result: {}", value),
    None => println!("Cannot divide by zero"),
}
```

### Extracting Values from `Option`

```rust
let some_number = Some(5);

match some_number {
    Some(n) => println!("The number is {}", n),  // n = 5
    None => println!("No number"),
}
```

## Matching `Result<T, E>`

Handle errors elegantly with pattern matching:

```rust
use std::fs::File;

match File::open("config.txt") {
    Ok(file) => {
        println!("File opened successfully");
        // Use file
    }
    Err(error) => {
        println!("Failed to open file: {}", error);
    }
}
```

### Matching Specific Errors

```rust
use std::io::ErrorKind;

match File::open("config.txt") {
    Ok(file) => println!("Success"),
    Err(error) => match error.kind() {
        ErrorKind::NotFound => println!("File not found"),
        ErrorKind::PermissionDenied => println!("Permission denied"),
        _ => println!("Other error: {}", error),
    },
}
```

## Pattern Matching Features

### 1. Multiple Patterns with `|`

```rust
let number = 2;

match number {
    1 | 2 => println!("One or two"),
    3 | 4 | 5 => println!("Three, four, or five"),
    _ => println!("Something else"),
}
```

### 2. Range Patterns with `..=`

```rust
let number = 42;

match number {
    1..=10 => println!("Between 1 and 10"),
    11..=50 => println!("Between 11 and 50"),
    51..=100 => println!("Between 51 and 100"),
    _ => println!("Outside range"),
}
```

Works with characters too:

```rust
let letter = 'c';

match letter {
    'a'..='z' => println!("Lowercase letter"),
    'A'..='Z' => println!("Uppercase letter"),
    _ => println!("Not a letter"),
}
```

### 3. Destructuring Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 0, y: 7 };

match point {
    Point { x: 0, y: 0 } => println!("Origin"),
    Point { x: 0, y } => println!("On Y-axis at {}", y),
    Point { x, y: 0 } => println!("On X-axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

### 4. Destructuring Tuples

```rust
let tuple = (1, 2, 3);

match tuple {
    (0, _, _) => println!("First is zero"),
    (_, 0, _) => println!("Second is zero"),
    (1, 2, 3) => println!("Exact match"),
    _ => println!("Something else"),
}
```

### 5. Ignoring Values with `_`

```rust
let numbers = (1, 2, 3, 4);

match numbers {
    (first, _, _, last) => {
        println!("First: {}, Last: {}", first, last);
        // Middle values ignored
    }
}
```

### 6. Ignoring Remaining Parts with `..`

```rust
let numbers = (1, 2, 3, 4, 5);

match numbers {
    (first, .., last) => {
        println!("First: {}, Last: {}", first, last);
    }
}

// Works with structs too
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

let point = Point3D { x: 1, y: 2, z: 3 };

match point {
    Point3D { x, .. } => println!("x = {}", x),  // Ignore y and z
}
```

### 7. Match Guards with `if`

Add extra conditions to patterns:

```rust
let number = Some(4);

match number {
    Some(n) if n < 5 => println!("Less than five: {}", n),
    Some(n) => println!("Greater than or equal to five: {}", n),
    None => println!("No number"),
}
```

Another example:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),  // Only matches if y is true
    _ => println!("no"),
}
```

### 8. `@` Bindings

Capture a value while also testing it:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable);
    }
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range");
        // Can't use the id value here
    }
    Message::Hello { id } => {
        println!("Found some other id: {}", id);
    }
}
```

## `if let` - Simplified Pattern Matching

When you only care about one pattern, use `if let`:

```rust
let some_value = Some(3);

// Instead of:
match some_value {
    Some(3) => println!("three"),
    _ => (),
}

// Use:
if let Some(3) = some_value {
    println!("three");
}
```

### With `else`

```rust
let number = Some(7);

if let Some(n) = number {
    println!("Matched: {}", n);
} else {
    println!("Didn't match");
}
```

## `while let` - Loop with Pattern Matching

Continue looping while a pattern matches:

```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
// Prints: 3, 2, 1
```

## `let` Statements are Patterns

Even regular `let` statements use pattern matching:

```rust
// Destructuring a tuple
let (x, y, z) = (1, 2, 3);

// Destructuring a struct
struct Point { x: i32, y: i32 }
let Point { x, y } = Point { x: 5, y: 10 };

// Ignoring values
let (a, _, c) = (1, 2, 3);
```

## Function Parameters are Patterns

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);
```

## Practical Examples

### Example 1: Parsing Commands

```rust
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Speak { message: String },
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => std::process::exit(0),
        Command::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
        }
        Command::Speak { message } => {
            println!("Saying: {}", message);
        }
    }
}
```

### Example 2: State Machine

```rust
enum State {
    Idle,
    Running { speed: u32 },
    Error { code: i32 },
}

fn handle_state(state: State) -> State {
    match state {
        State::Idle => {
            println!("Starting...");
            State::Running { speed: 10 }
        }
        State::Running { speed } if speed > 100 => {
            println!("Too fast! Error!");
            State::Error { code: 1 }
        }
        State::Running { speed } => {
            println!("Running at speed {}", speed);
            State::Running { speed: speed + 10 }
        }
        State::Error { code } => {
            println!("Error {}, resetting", code);
            State::Idle
        }
    }
}
```

### Example 3: Nested Matching

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: {}, {}, {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: {}, {}, {}", h, s, v);
        }
    }
}
```

## Best Practices

1. **Use `match` for exhaustive handling** - Compiler ensures all cases are covered
2. **Use `if let` for single patterns** - Cleaner than `match` with `_ => ()`
3. **Use match guards sparingly** - Complex conditions can make code hard to read
4. **Destructure in function parameters** - Makes intent clear
5. **Prefer pattern matching over `unwrap()`** - More explicit error handling
6. **Use `_` to ignore values** - Shows intent to ignore
7. **Use `..` to ignore multiple fields** - Cleaner than multiple `_`

## Common Patterns

### Unwrapping with Default

```rust
let value = Some(5);
let result = match value {
    Some(n) => n,
    None => 0,  // Default value
};

// Or use .unwrap_or()
let result = value.unwrap_or(0);
```

### Transforming Options

```rust
let value = Some(5);
let doubled = match value {
    Some(n) => Some(n * 2),
    None => None,
};

// Or use .map()
let doubled = value.map(|n| n * 2);
```

### Early Returns

```rust
fn process(value: Option<i32>) -> i32 {
    let n = match value {
        Some(n) => n,
        None => return 0,  // Early return
    };
    
    n * 2
}

// Or use the ? operator
fn process_with_option(value: Option<i32>) -> Option<i32> {
    let n = value?;  // Returns None if value is None
    Some(n * 2)
}
```

## Summary

Pattern matching in Rust is powerful because it:
- **Ensures exhaustiveness** - Compiler checks all cases are handled
- **Destructures data** - Extract values from complex types
- **Returns values** - `match` is an expression
- **Provides safety** - No null pointer exceptions
- **Enables expressive code** - Clear intent, readable logic

Key constructs:
- `match` - Full pattern matching
- `if let` - Single pattern matching
- `while let` - Loop with pattern matching
- Match guards - Add conditions with `if`
- `@` bindings - Capture and test simultaneously

Pattern matching is fundamental to idiomatic Rust. Master it, and you'll write safer, more expressive code!
