# Object-Oriented Concepts in Rust

Rust doesn't have traditional classes, interfaces, or inheritance like Java/C++. Instead, it uses **structs**, **traits**, and **composition**. This guide explains Rust's approach to OOP concepts.

## Quick Comparison: Traditional OOP vs Rust

| Traditional OOP | Rust Equivalent | Purpose |
|----------------|-----------------|---------|
| Class | `struct` + `impl` | Data + methods |
| Interface | `trait` | Shared behavior contract |
| Inheritance | Composition + Traits | Code reuse |
| Abstract Class | `trait` with default methods | Partial implementation |
| Function Pointer | `fn` pointer or `Fn` trait | Callbacks, higher-order functions |

## 1. Structs (Rust's "Classes")

Rust uses **structs** to define custom data types. Unlike classes, structs only hold data—methods are defined separately.

### Defining a Struct

```rust
// Define a struct (like a class without methods)
struct Player {
    name: String,
    health: u32,
    score: i32,
}
```

### Creating Instances

```rust
fn main() {
    // Create an instance
    let player = Player {
        name: String::from("Alice"),
        health: 100,
        score: 0,
    };
    
    // Access fields
    println!("{} has {} health", player.name, player.health);
}
```

### Adding Methods with `impl`

```rust
struct Player {
    name: String,
    health: u32,
    score: i32,
}

// Implementation block - where methods live
impl Player {
    // Associated function (like a static method or constructor)
    fn new(name: String) -> Player {
        Player {
            name,
            health: 100,
            score: 0,
        }
    }
    
    // Method (takes &self, like instance methods)
    fn take_damage(&mut self, damage: u32) {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }
    
    // Method that doesn't modify self
    fn is_alive(&self) -> bool {
        self.health > 0
    }
    
    // Method that consumes self (takes ownership)
    fn game_over(self) -> String {
        format!("{} finished with {} points", self.name, self.score)
    }
}

fn main() {
    let mut player = Player::new(String::from("Bob"));
    player.take_damage(30);
    
    if player.is_alive() {
        println!("{} is still alive with {} health", player.name, player.health);
    }
}
```

### Method Types

```rust
impl Player {
    // 1. Associated function (no self) - like static methods
    fn new(name: String) -> Player { /* ... */ }
    
    // 2. Immutable method (&self) - read-only access
    fn is_alive(&self) -> bool { /* ... */ }
    
    // 3. Mutable method (&mut self) - can modify
    fn take_damage(&mut self, damage: u32) { /* ... */ }
    
    // 4. Consuming method (self) - takes ownership
    fn game_over(self) -> String { /* ... */ }
}
```

## 2. Traits (Rust's "Interfaces")

**Traits** define shared behavior. They're like interfaces in Java or abstract classes in C++.

### Defining a Trait

```rust
// Define a trait (interface)
trait Drawable {
    // Method signature (must be implemented)
    fn draw(&self);
    
    // Default implementation (optional)
    fn hide(&self) {
        println!("Hidden");
    }
}
```

### Implementing a Trait

```rust
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// Implement Drawable for Circle
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

// Implement Drawable for Rectangle
impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
    
    // Override default implementation
    fn hide(&self) {
        println!("Rectangle is now hidden");
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 10.0, height: 20.0 };
    
    circle.draw();  // "Drawing a circle with radius 5"
    rect.draw();    // "Drawing a rectangle 10x20"
    
    circle.hide();  // "Hidden" (default implementation)
    rect.hide();    // "Rectangle is now hidden" (overridden)
}
```

### Trait Bounds (Generic Constraints)

```rust
// Function that works with any Drawable type
fn render<T: Drawable>(item: &T) {
    item.draw();
}

// Alternative syntax
fn render_alt(item: &impl Drawable) {
    item.draw();
}

// Multiple trait bounds
fn process<T: Drawable + Clone>(item: T) {
    item.draw();
    let copy = item.clone();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    render(&circle);
}
```

### Common Standard Traits

```rust
// Clone - ability to duplicate
#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Debug - ability to print with {:?}
#[derive(Debug)]
struct Player {
    name: String,
}

// Multiple traits
#[derive(Debug, Clone, PartialEq)]
struct Item {
    id: u32,
    name: String,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();  // Clone trait
    
    let player = Player { name: String::from("Alice") };
    println!("{:?}", player);  // Debug trait
    
    let item1 = Item { id: 1, name: String::from("Sword") };
    let item2 = item1.clone();
    if item1 == item2 {  // PartialEq trait
        println!("Items are equal");
    }
}
```

## 3. Composition Over Inheritance

Rust **doesn't have inheritance**. Instead, it uses **composition** and **traits**.

### Traditional Inheritance (Java/C++)

```java
// Java example (NOT Rust!)
class Animal {
    void eat() { /* ... */ }
}

class Dog extends Animal {
    void bark() { /* ... */ }
}
```

### Rust Approach: Composition

```rust
// Instead of inheritance, use composition
struct Animal {
    name: String,
    age: u32,
}

impl Animal {
    fn eat(&self) {
        println!("{} is eating", self.name);
    }
}

// Dog contains an Animal (composition)
struct Dog {
    animal: Animal,  // Composition!
    breed: String,
}

impl Dog {
    fn bark(&self) {
        println!("{} says woof!", self.animal.name);
    }
    
    // Delegate to Animal's method
    fn eat(&self) {
        self.animal.eat();
    }
}

fn main() {
    let dog = Dog {
        animal: Animal {
            name: String::from("Buddy"),
            age: 3,
        },
        breed: String::from("Golden Retriever"),
    };
    
    dog.eat();   // Delegates to animal.eat()
    dog.bark();  // Dog's own method
}
```

### Composition with Traits

```rust
// Define common behavior with traits
trait Eater {
    fn eat(&self);
}

trait Sleeper {
    fn sleep(&self);
}

// Different types can implement the same traits
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Eater for Dog {
    fn eat(&self) {
        println!("{} eats dog food", self.name);
    }
}

impl Eater for Cat {
    fn eat(&self) {
        println!("{} eats cat food", self.name);
    }
}

impl Sleeper for Dog {
    fn sleep(&self) {
        println!("{} sleeps in a dog bed", self.name);
    }
}

impl Sleeper for Cat {
    fn sleep(&self) {
        println!("{} sleeps anywhere", self.name);
    }
}

// Function that works with any Eater
fn feed_animal(animal: &impl Eater) {
    animal.eat();
}

fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };
    
    feed_animal(&dog);
    feed_animal(&cat);
}
```

## 4. Function Pointers and Closures

Rust has multiple ways to work with functions as values.

### Function Pointers (`fn`)

```rust
// Define a function
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Function that takes a function pointer
fn apply_operation(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

fn main() {
    let result1 = apply_operation(5, 3, add);       // 8
    let result2 = apply_operation(5, 3, multiply);  // 15
    
    println!("Add: {}, Multiply: {}", result1, result2);
}
```

### Closures (Anonymous Functions)

```rust
fn main() {
    // Closure - anonymous function
    let add = |a, b| a + b;
    let result = add(5, 3);  // 8
    
    // Closure with type annotations
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    
    // Closure that captures environment
    let x = 10;
    let add_x = |y| x + y;  // Captures 'x' from environment
    println!("{}", add_x(5));  // 15
}
```

### Closure Traits: `Fn`, `FnMut`, `FnOnce`

```rust
// Fn - can be called multiple times, doesn't modify captured variables
fn call_twice<F>(f: F) where F: Fn() {
    f();
    f();
}

// FnMut - can be called multiple times, can modify captured variables
fn call_and_modify<F>(mut f: F) where F: FnMut() {
    f();
    f();
}

// FnOnce - can only be called once, consumes captured variables
fn call_once<F>(f: F) where F: FnOnce() {
    f();
}

fn main() {
    // Fn example
    let x = 5;
    let print_x = || println!("x = {}", x);
    call_twice(print_x);
    
    // FnMut example
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    call_and_modify(&mut increment);
    
    // FnOnce example
    let s = String::from("hello");
    let consume = || {
        let _owned = s;  // Takes ownership
    };
    call_once(consume);
    // consume(); // Error! Can't call again
}
```

### Storing Functions in Structs

```rust
struct Calculator {
    operation: fn(i32, i32) -> i32,
}

impl Calculator {
    fn new(op: fn(i32, i32) -> i32) -> Calculator {
        Calculator { operation: op }
    }
    
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}

fn main() {
    let add = |a, b| a + b;
    let calc = Calculator::new(add);
    println!("Result: {}", calc.calculate(5, 3));  // 8
}
```

### Callbacks Example

```rust
struct Button {
    label: String,
    on_click: Option<fn()>,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: label.to_string(),
            on_click: None,
        }
    }
    
    fn set_callback(&mut self, callback: fn()) {
        self.on_click = Some(callback);
    }
    
    fn click(&self) {
        println!("Button '{}' clicked!", self.label);
        if let Some(callback) = self.on_click {
            callback();
        }
    }
}

fn handle_click() {
    println!("Callback executed!");
}

fn main() {
    let mut button = Button::new("Submit");
    button.set_callback(handle_click);
    button.click();
}
```

## 5. Trait Objects (Dynamic Dispatch)

Sometimes you need polymorphism at runtime. Use **trait objects**.

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

// Vector of different types implementing Animal
fn main() {
    // Use trait objects with Box<dyn Trait>
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Dog),
    ];
    
    // Polymorphic call
    for animal in animals.iter() {
        animal.make_sound();
    }
}
```

### `dyn` Keyword

```rust
// Static dispatch (compile-time, faster)
fn feed_static<T: Animal>(animal: &T) {
    animal.make_sound();
}

// Dynamic dispatch (runtime, more flexible)
fn feed_dynamic(animal: &dyn Animal) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    
    feed_static(&dog);    // Monomorphized at compile time
    feed_dynamic(&cat);   // Resolved at runtime
}
```

## Real-World Example: From Our Hangman Game

```rust
use serde::{Deserialize, Serialize};

// Struct (like a class)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    word: String,
    guessed_letters: Vec<char>,
    wrong_guesses: u32,
    max_wrong_guesses: u32,
    game_over: bool,
    won: bool,
}

// Implementation (methods)
impl GameState {
    // Associated function (constructor)
    fn new(word: String) -> Self {
        GameState {
            word: word.to_uppercase(),
            guessed_letters: Vec::new(),
            wrong_guesses: 0,
            max_wrong_guesses: 6,
            game_over: false,
            won: false,
        }
    }
    
    // Method
    fn check_win(&self) -> bool {
        self.word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }
}

// Traits are derived (Debug, Clone, Serialize, Deserialize)
// These provide automatic implementations
```

## Summary Table

| Concept | Rust Solution | Example |
|---------|---------------|---------|
| **Class** | `struct` + `impl` | `struct Player { ... } impl Player { ... }` |
| **Interface** | `trait` | `trait Drawable { fn draw(&self); }` |
| **Inheritance** | Composition + Traits | Embed structs, implement traits |
| **Polymorphism** | Trait objects (`dyn`) | `Box<dyn Animal>` |
| **Function Pointer** | `fn` type | `fn(i32) -> i32` |
| **Lambda/Closure** | Closures | `\|x\| x + 1` |
| **Abstract Method** | Trait method | `fn draw(&self);` in trait |
| **Default Method** | Trait default impl | `fn hide(&self) { ... }` in trait |

## Key Differences from Traditional OOP

1. **No inheritance** - Use composition and traits instead
2. **No classes** - Use structs with impl blocks
3. **Traits are more flexible** - Can implement traits for types you don't own
4. **No method overloading** - Use different names or generic parameters
5. **Explicit self** - Methods must specify `self`, `&self`, or `&mut self`
6. **Ownership matters** - Methods can consume, borrow, or mutably borrow

## Best Practices

✅ **DO:**
- Use composition over inheritance
- Implement traits for shared behavior
- Use `#[derive(...)]` for common traits
- Prefer static dispatch (`impl Trait`) over dynamic (`dyn Trait`)
- Use closures for callbacks and short functions

❌ **DON'T:**
- Try to replicate class hierarchies from other languages
- Overuse trait objects (they have runtime cost)
- Create deep composition chains (keep it simple)

## Resources

- **Rust Book - Structs**: https://doc.rust-lang.org/book/ch05-00-structs.html
- **Rust Book - Traits**: https://doc.rust-lang.org/book/ch10-02-traits.html
- **Rust Book - OOP**: https://doc.rust-lang.org/book/ch17-00-oop.html
- **Rust Book - Closures**: https://doc.rust-lang.org/book/ch13-01-closures.html
