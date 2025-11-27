# Rust Operators Explained

Operators in Rust are symbols that perform operations on values. This guide covers all the main operators and what they do.

## Arithmetic Operators

Used for mathematical calculations.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `+` | Addition | Adds two values | `5 + 3` → `8` |
| `-` | Subtraction | Subtracts right from left | `5 - 3` → `2` |
| `*` | Multiplication | Multiplies two values | `5 * 3` → `15` |
| `/` | Division | Divides left by right | `6 / 3` → `2` |
| `%` | Modulo/Remainder | Returns remainder of division | `5 % 3` → `2` |

```rust
let sum = 10 + 5;        // 15
let diff = 10 - 5;       // 5
let product = 10 * 5;    // 50
let quotient = 10 / 5;   // 2
let remainder = 10 % 3;  // 1
```

### Important Notes:
- Integer division truncates: `5 / 2` → `2` (not `2.5`)
- Division by zero causes a panic at runtime
- Use floating-point types for decimal results: `5.0 / 2.0` → `2.5`

## Comparison Operators

Used to compare values. Always return a `bool` (`true` or `false`).

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `==` | Equal | Checks if values are equal | `5 == 5` → `true` |
| `!=` | Not equal | Checks if values are different | `5 != 3` → `true` |
| `<` | Less than | Left is smaller than right | `3 < 5` → `true` |
| `>` | Greater than | Left is larger than right | `5 > 3` → `true` |
| `<=` | Less than or equal | Left is smaller or equal | `3 <= 3` → `true` |
| `>=` | Greater than or equal | Left is larger or equal | `5 >= 5` → `true` |

```rust
let is_equal = 5 == 5;           // true
let is_different = 5 != 3;       // true
let is_less = 3 < 5;             // true
let is_greater = 5 > 3;          // true
let is_less_or_equal = 3 <= 5;   // true
let is_greater_or_equal = 5 >= 3; // true
```

## Logical Operators

Used to combine boolean expressions.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `&&` | Logical AND | True if both are true | `true && true` → `true` |
| `\|\|` | Logical OR | True if either is true | `true \|\| false` → `true` |
| `!` | Logical NOT | Inverts the boolean | `!true` → `false` |

```rust
let and_result = true && false;   // false
let or_result = true || false;    // true
let not_result = !true;           // false

// Practical example
let age = 25;
let has_license = true;
let can_drive = age >= 18 && has_license;  // true
```

### Short-Circuit Evaluation:
- `&&` stops evaluating if the first value is `false`
- `||` stops evaluating if the first value is `true`

```rust
let x = 5;
// Second part never runs because first is false
let result = (x < 0) && (expensive_function());
```

## Bitwise Operators

Operate on individual bits of integers.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `&` | Bitwise AND | 1 if both bits are 1 | `5 & 3` → `1` |
| `\|` | Bitwise OR | 1 if either bit is 1 | `5 \| 3` → `7` |
| `^` | Bitwise XOR | 1 if bits are different | `5 ^ 3` → `6` |
| `!` | Bitwise NOT | Inverts all bits | `!5` → `-6` |
| `<<` | Left shift | Shifts bits left | `5 << 1` → `10` |
| `>>` | Right shift | Shifts bits right | `5 >> 1` → `2` |

```rust
let a = 0b1010;  // 10 in binary
let b = 0b1100;  // 12 in binary

let and = a & b;   // 0b1000 = 8
let or = a | b;    // 0b1110 = 14
let xor = a ^ b;   // 0b0110 = 6
let not = !a;      // Inverts all bits
let left = a << 1; // 0b10100 = 20
let right = a >> 1; // 0b0101 = 5
```

### Use Cases:
- Flags and permissions
- Low-level programming
- Performance optimization
- Cryptography

## Assignment Operators

Assign values to variables.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `=` | Assignment | Assigns right to left | `x = 5` |
| `+=` | Add and assign | `x = x + y` | `x += 5` |
| `-=` | Subtract and assign | `x = x - y` | `x -= 5` |
| `*=` | Multiply and assign | `x = x * y` | `x *= 5` |
| `/=` | Divide and assign | `x = x / y` | `x /= 5` |
| `%=` | Modulo and assign | `x = x % y` | `x %= 5` |
| `&=` | Bitwise AND and assign | `x = x & y` | `x &= 5` |
| `\|=` | Bitwise OR and assign | `x = x \| y` | `x \|= 5` |
| `^=` | Bitwise XOR and assign | `x = x ^ y` | `x ^= 5` |
| `<<=` | Left shift and assign | `x = x << y` | `x <<= 1` |
| `>>=` | Right shift and assign | `x = x >> y` | `x >>= 1` |

```rust
let mut x = 10;

x += 5;   // x is now 15
x -= 3;   // x is now 12
x *= 2;   // x is now 24
x /= 4;   // x is now 6
x %= 4;   // x is now 2
```

## Range Operators

Create ranges of values.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `..` | Exclusive range | Start to end (excluding end) | `1..5` → `1,2,3,4` |
| `..=` | Inclusive range | Start to end (including end) | `1..=5` → `1,2,3,4,5` |
| `..` | Range from | From start to infinity | `5..` |
| `..` | Range to | From beginning to end | `..5` |
| `..=` | Range to inclusive | From beginning to end (inclusive) | `..=5` |

```rust
// Exclusive range (doesn't include 5)
for i in 1..5 {
    println!("{}", i);  // Prints: 1, 2, 3, 4
}

// Inclusive range (includes 5)
for i in 1..=5 {
    println!("{}", i);  // Prints: 1, 2, 3, 4, 5
}

// Slicing arrays
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4];  // [2, 3, 4]
```

## Reference and Dereference Operators

Work with references and pointers.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `&` | Reference | Creates a reference | `&x` |
| `&mut` | Mutable reference | Creates a mutable reference | `&mut x` |
| `*` | Dereference | Accesses value behind reference | `*x` |

```rust
let x = 5;
let reference = &x;        // Reference to x
let value = *reference;    // Dereference to get 5

let mut y = 10;
let mut_ref = &mut y;      // Mutable reference
*mut_ref += 5;             // y is now 15
```

### Key Points:
- `&` borrows a value (immutable)
- `&mut` borrows a value (mutable)
- `*` accesses the actual value from a reference

## Type Cast Operator

Convert between types.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `as` | Type cast | Converts one type to another | `x as f64` |

```rust
let integer = 5;
let float = integer as f64;     // 5.0

let large = 1000_i32;
let small = large as i8;        // Truncates to fit

let character = 65 as char;     // 'A'
```

### Important:
- Can lose data (e.g., `i32` to `i8`)
- Use for primitive types
- For complex conversions, use `From`/`Into` traits

## Question Mark Operator

Propagates errors in functions that return `Result` or `Option`.

| Operator | Name | Description |
|----------|------|-------------|
| `?` | Error propagation | Returns early if error/None |

```rust
use std::fs::File;
use std::io::Read;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("data.txt")?;  // Returns error if fails
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;     // Returns error if fails
    Ok(contents)                              // Returns Ok if successful
}
```

### What It Does:
- If `Result` is `Ok(value)`, unwraps to `value`
- If `Result` is `Err(e)`, returns `Err(e)` from function
- If `Option` is `Some(value)`, unwraps to `value`
- If `Option` is `None`, returns `None` from function

## Dot Operator

Access fields and methods.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `.` | Member access | Access struct fields/methods | `obj.field` |

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

let point = Point { x: 3, y: 4 };
let x_value = point.x;           // Access field
let dist = point.distance();     // Call method
```

## Double Colon Operator

Access associated functions and items.

| Operator | Name | Description | Example |
|----------|------|-------------|---------|
| `::` | Path separator | Access module items/associated functions | `Vec::new()` |

```rust
// Associated function
let v = Vec::new();

// Module path
use std::collections::HashMap;

// Enum variant
let opt = Option::Some(5);

// Type alias
let s = String::from("hello");
```

See the [rust-double-colon-operator.md](rust-double-colon-operator.md) doc for more details.

## Operator Precedence

Operators are evaluated in a specific order (highest to lowest):

1. Method calls (`.`)
2. Unary operators (`!`, `-`, `*`, `&`)
3. `as` casting
4. Multiplication, division, modulo (`*`, `/`, `%`)
5. Addition, subtraction (`+`, `-`)
6. Bit shifts (`<<`, `>>`)
7. Bitwise AND (`&`)
8. Bitwise XOR (`^`)
9. Bitwise OR (`|`)
10. Comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`)
11. Logical AND (`&&`)
12. Logical OR (`||`)
13. Range (`..`, `..=`)
14. Assignment (`=`, `+=`, etc.)

```rust
// Example showing precedence
let result = 2 + 3 * 4;  // 14, not 20 (multiplication first)

// Use parentheses to override
let result = (2 + 3) * 4;  // 20
```

## Special Operators

### Underscore `_`
Used as a placeholder or to ignore values.

```rust
let _ = some_function();  // Ignore return value

match value {
    0 => println!("zero"),
    _ => println!("other"),  // Catch-all pattern
}

let (x, _) = (5, 10);  // Ignore second value
```

### Turbofish `::<>`
Specify generic type parameters explicitly.

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect::<Vec<i32>>();  // Turbofish specifies type
```

## Summary

Rust operators fall into several categories:
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Logical**: `&&`, `||`, `!`
- **Bitwise**: `&`, `|`, `^`, `!`, `<<`, `>>`
- **Assignment**: `=`, `+=`, `-=`, etc.
- **Range**: `..`, `..=`
- **Reference**: `&`, `&mut`, `*`
- **Special**: `.`, `::`, `?`, `as`, `_`

Most operators work as you'd expect from other languages, but Rust's ownership system adds important considerations when working with references and dereferencing.
