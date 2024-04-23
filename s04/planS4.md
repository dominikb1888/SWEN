---
# Object-orientation and Pattern Matching


---


---
# Structs and Object-Orientation

- Generic Structs, and Traits
- Smart Pointers and Interior Mutability (Advanced)


---
### Review Structs in Clack

---
### Named-Field Structs, Tuple-Like Structs, Unit-Like Structs

2. Named field structs (Pub structs, pub fields)
3. Tuple-Like Structs
4. Unit Like Structs

---
### Structs in Memory

- No promise on field order
- Fields are stored in Struct's block though
- Heap allocated types remain own block
- #[repr(C)] creates C/C++ compatible Layout


---
### Defining Methods with impl (1/2)

- Associated Functions (aka methods) are defined within an impl block
- Free Functions are defined outside of an impl block
- Self as first argument to all methods, Shorthand form possible
- Self can be referenced and mutable (mut self, &mut self)

---
### Defining Methods with impl (2/2)
- Smart Pointers (Box, Rc, Arc) can be used on self
- Type-Associated functions do not need to take self
- new() as convention for constructor function
- Associated Constants can be defined outside a function scope

---
### Generic Structs

- Sometimes our own types need to be able to consume different types (Queue Example)
- <T> allows writing type unspecific Types and methods
- This tells the compiler to allow all different types (In Python this is the default)
- References in Generic Structs need defined Lifetimes Parameters (Sometimes tricky, immutability to the rescue)
- Generic Structs can take constant-value Paramters

---
### Using common traits with #[derive()]

First exmaples (later more):
- Copy
- Clone
- Debug
- Partial Eq

---

### Interior Mutability (optional)

- Lots of mutability causes problems, a little is a good thing
- Interior Mutabilit: "Little bit of mutability inside otherwise immutable type"
- Use of Rc, Cell<T>, RefCell<T>


---
# Enums and Pattern Matching


---
### Building Enums

- Allows building a type with associated values called variants
- Some kind of logic and structure between the values
- match statement will check of completeness when using enums (MECE principle)


---
# Enum Example

´´´{rust}

enum HttpStatus {
    Ok = 200
    NotModified = 304
    Not Found = 404
}

´´´

---
### Match Example

´´´
match n {
    200 => Some(HttpStatus::Ok),
    304 => Some(HttpStatus::NotModified),
    404 => Some(HttpStatus::NotFound),
    _ => None,
}
´´´

---
### Enums in Memory

- Visual on Page 236
- No promises on Enum Layout
- Fields are stored as "small integer tag"

---
### Rich Data Structues using Enums

- A JSON Example


---
### Generic Enums

- Binary Tree Example


---
### Patterns

- Table on Page 243
- Literals, Variables, and Wildcards
- Tuple and Struct Patterns
- Array and Slice Patterns
- Reference Patterns
- Match Guards
- Matching Multiple
- Binding with @





