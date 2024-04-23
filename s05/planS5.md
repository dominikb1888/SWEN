---
# Encapsulation and Abstraction: Traits and Generics, Operator Overloading

Goals:
- Understand how to build own traits
- Understand how to build own operator behavior
- Understand utility traits
- Finding ways to implement these language options in a real-life example


---
## Traits and Generics

- Trait Objects and Generics allow writing polymorphic code
- Types and Methods with generic processing
- Methods on Traits are called Trait Methods (closest resemblance are interfaces in Java, Some similarity with dunder-methods in python)

- "Traits are contracts between distinct parts of the code, they agree upon a list of functions that can be called."
---
### Structs, Traits, Methods

- A struct represents a concrete type. It's an Object.
- A function defined for a particular struct is a method. Concretely this is a function that takes a Self type.
- A trait in contrast is a category. It represents an abstraction that many types or structs can be members of. A trait may have a method defined for it, but that method is inherently abstract as traits are abstract. It's only when paired with a implementation for a particular struct that a trait becomes concrete.

- There are three distinct abstractions here, structs, traits, and trait implementations that bridge the two. Methods can be defined on all three, but can only be concrete (in fact must be concrete) on structs and trait implementations.

Source:
- https://www.reddit.com/r/rust/comments/n4ava8/difference_between_methods_and_traits/
- https://betterprogramming.pub/rust-basics-structs-methods-and-traits-bb4839cd57bd

---
### Trait examples

 -A value that implements std::io::Write can write out bytes.
- A value that implements std::iter::Iterator can produce a sequence of
values.
- A value that implements std::clone::Clone can make clones of itself in memory.
- A value that implements std::fmt::Debug can be printed using println!() with the {:?} format specifier.

---
### Trait implementation examples

Those four traits are all part of Rust’s standard library, and many standard types implement them. For example:
- std::fs::File implements the Write trait; it writes bytes to a local file. std::net::TcpStream writes to a network connection. Vec<u8> also implements Write. Each .write() call on a vector of bytes appends some data to the end.
- Range<i32> (the type of 0..10) implements the Iterator trait, as do some itera‐ tor types associated with slices, hash tables, and so on.
- Most standard library types implement Clone. The exceptions are mainly types like TcpStream that represent more than just data in memory.
- Likewise, most standard library types support Debug.

---
### Defining a Trait

´´´{rust}
/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen.

trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);
    /// Return true if clicking at (x, y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
    }
´´´

---
### Implementing a Trait

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1 .. self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

---
### Trait rules

Rule about trait methods: the trait itself must be in scope. Otherwise, all its methods are hidden:

Wrong:

´´´{rust}
let mut buf: Vec<u8> = vec![];
buf.write_all(b"hello")?; // error: no method named `write_all`
´´´

Right:
´´´{rust}
use std::io::Write;

let mut buf: Vec<u8> = vec![];
buf.write_all(b"hello")?; // ok
´´´

- Necessary, because Rust allows adding new methods to any type.
- Namespace conflict are rare (Fix: Fully qualified Method Syntax)
- Clone and Iterator are always in scope

---
### Trait Objects and Dynamic Dispatch

- a way to use dynamic dispatch in Rust. Dynamic dispatch means that the compiler does not know at compile time which implementation of a trait to use for a given value, but instead it determines it at run time based on the actual type of the value.
- created by using the dyn keyword before a trait name, such as dyn Display or dyn Iterator.
- can be stored in variables, passed as arguments, or returned from functions, as long as they are behind a pointer, such as a reference, a Box, or a Rc.
- useful when you need to work with heterogeneous collections of types that share a trait, or when you want to defer the decision of which type to use until run time.

---
### Generics

- a way to use static dispatch in Rust. Static dispatch means that the compiler knows at compile time which implementation of a trait to use for a given value, and it generates specialized code for each type that is used.
- created by using angle brackets after a type name, a function name, or a trait name, such as Vec<T>, fn foo<T: Display>(x: T), or impl<T: Display> Display for MyType<T>.
- can also be stored in variables, passed as arguments, or returned from functions, without any pointers.
- are useful when you want to write code that can work with any type that meets some constraints, or when you want to optimize the performance of your code by avoiding the overhead of dynamic dispatch.


---
### Trait Objects vs. Generics

When comparing trait objects and generics, there are different trade-offs to consider in terms of flexibility, performance, and complexity:

- Trait objects are more flexible than generics, as they allow the use of any type that implements the trait, regardless of what is known at compile time.
- However, generics are more performant than trait objects due to the avoidance of dynamic dispatch overhead.
- Generics also have a greater complexity than trait objects, as they require more syntax, concepts, and constraints. On the other hand, trait objects are simpler to use as they only require the dyn keyword, object safety rules, and pointer types.

Source: https://www.linkedin.com/advice/0/how-do-you-choose-between-trait-objects-generics#:~:text=Trait%20objects%20are%20more%20flexible,avoidance%20of%20dynamic%20dispatch%20overhead.

---
### When to use which?

- Trait objects should be used when working with heterogeneous collections of types that share a trait, or when wanting to defer the decision of which type to use until run time. Examples include plugin systems, dependency injection frameworks, or strategy patterns.

- Generics should be used when wanting to write code that can work with any type that meets some constraints, or when needing to optimize performance by avoiding dynamic dispatch. Examples include containers, iterators, or algorithms.

- If you want to balance flexibility and performance, or abstract over different kinds of traits, then you can use a combination of trait objects and generics. For example, you can use generics to implement a trait and then use trait objects to store or pass values of that trait.



---
## Operator Overloading



---
## Utility Traits
