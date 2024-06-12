# Software Development - Clean Code, Data Structures, and Algorithms

## Readings and Resources:


### Required Reading:

Lecture:
- [Introduction to Algorithms](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6925615)

Exercise:
- [Programming Rust (First half)](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6643397)
- [Algorithms and Data Structures in Rust (Second half)](https://www.packtpub.com/product/hands-on-data-structures-and-algorithms-with-rust/9781788995528)
### Additional Resources

- Rust Book, Brown University with great quizzes: https://rust-book.cs.brown.edu/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/index.html

### Required Preparation before 1st Day of Class

- Walk through the rustlings tutorial: https://github.com/rust-lang/rustlings
- Finish all [Exercises](././rust_assignments.md) for Session 1 in Rust


### Mandatory Exercises:

Prepare the exercises in the following list according to the sessions they are assigned to. Each exercise should take about 30min to complete, if approached right. There are 2 times 42 exercises (what a surprise). The first 42 are to warm up and get comfortable with Rust. The second 42 are a little more complex and require either a clever algorithm or a good design. Make use of any tool at your disposal and do work in groups to reflect upon the different solutions possible:

- [Rust Exercises](././rust_assignments.md)

Credit: The exercises are taken from [Exercism](http://www.exercism.org), you can find many other languages to practice there and a helpful community. You can also find solutions to each exercise. In you own interest, only look at these solutions after spending at least 30min on an honest attempt to each exercise.


## Class Design - Flipped Classroom

The course is taught in a Flipped Classroom style. You are required to prepare coding exercises and will be asked to present them in front of class on a random selection basis.

- 30 min | Review and joint development of selected Homework Exercises
- 60 min | Theoretical input on computational thinking with Algorithms and Data Structures.
- 90 min | Applying theoretical input in a hands-on class project: Building the Game of Clack (https://github.com/dominikb1888/clap).


## Exam (graded)

The Exam will be written and online. It will be a code exercise just like the ones you practice with tests provided.


## Agenda

### Programming in Rust

1. Types: Data Types and Rust Tooling ([Chapter 1-3](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=23)) | Exercises R01

2. Memory Management: Mutability and Ownership ([Chapter 3-6](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=101)) | Exercises R02

3. System Design: Error Handling, Crates and Modules ([Chapter 7-8](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=179)) | Exercises R03

4. Object-Oriented-Design: Structs, Enums and Patterns [Chapter 9-10](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=231)) | Exercises R04

5. Encapsulation and Abstraction: Traits and Generics, Operator Overloading ([Chapter 11-13](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=277)) | Exercises R05

6. Functional Programming: Closures, Iterators and Collections ([Chapter 14-16](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=349)) | Exercises R06

7. Input and Output: Strings, Text and IO ([Chapter 17-18](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6643397&ppg=451)) | Exercises R07

### Algorithms and Data Structures in Rust

8. Algorithms Analysis, Design and Evaluation ([Chapter 1-4](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6925615&ppg=21)) | Exercises R08 or Project

9. Lists ([Chapter 7-10](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6925615&ppg=139)) | Exercises R09 or Project

10. Maps and Sets ([Chapter ](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6925615&ppg=214)) | Exercises R10 or Project

11. Trees ([Chapter 6, 11-13](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6925615&ppg=261)) | Exercises R11 or Project


12. Graphs ([Chapter 20-22](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6925615&ppg=443)) | Exercises R12 or Project
13. Review and Mock Exam | Project



## Sessions

### 1. Types: Rust Data Types and Tooling

Learning Goals:
- Understand why are types important for programming in General
- Understand how can types help support efficiency, safety, and security
- Know basic types in Rust and understand how they compare to Python
- Understand the differences between scalar types, compound types, and structs
- Apply Numeric Types, Strings, and Pointers in your programs.
- Apply Tuples, Arrays, Vectors, and Hashmaps in your programs.

1. Exercise Review:

2. Lecture: Why do we need types and what can we do with them?

3. Project: Creating preliminary Data Structure and Logic for our Game of Clack


### 2. Memory Management: Mutability and Ownership

Learning Goals:
- Understand head and stack and their usage and implications for structuring programs
- Understand the need for and pitfalls of pointers
- Understand the difference between mutable and immutable types
- Understand how C, Python and Rust handle memory
- Understand Ownership and Borrowing and Rust for both mutable and immutable types
- Analyse and resolve issues arising from Ownership and Borrowing in your own code


Additional Reading:
- https://depth-first.com/articles/2020/01/27/rust-ownership-by-example/
- https://fasterthanli.me/articles/declarative-memory-management

The detailed content of each Session will be added here during the term.

### 3. System Design

Learning Goals:
- Understand and apply important principles of systems design
- Know and apply the Separation of Concerns principle
- Know and apply the Don't Repeat Yourself Principle
- Know and apply the KISS Principle
- Know and apply the SOLID Principle
- Know and apply the YAGNI Principle
- Avoid premature optimization

1. Exercise Review

2. Lecture
- Programming Principles

3. Project
- Create Basic Data Structure and Simple Frontend for Clack


### 4. Object-Oriented-Design

Learning Goals:
- Understand the basics of object-orientation
- Know and understand how to build a Struct in Rust
- Develop a basic understanding of Traits and Generics
- Get to know Enums and understand when to use them
- Understand Generics and Enums
- Apply Pattern Matching

1. Exercise Review

2. Lecture:

- Object Orientation and Generic Types in Rust
- Enums and Pattern Matching


3. Project:

- Building Game, Turn and Clacking Function


### 5. Encapsulation and Abstraction

Learning Goals:
- Get an Advanced understanding of Abstract Types
- Understand the differences between Trait Object and Generics
- Know how to write own Traits Object and Generics
- Know how to use Generic Functions and Type Parameters
- Understand Polymorphism, Apply Generic Traits and Operator Overloading
- Know and apply Utility Traits

### 6. Functional Programming

Learning Goals:
- Understand the differences between object-oriented Programming and functional Programming
- Know and apply higher-order functions in Rust (aka Iterator Adapters)
- Understand and apply closures in Rust
- Using and apply functional patterns to collection types in Rust


### 7. Strings and IO

UTF-8:
- Encoding: https://en.wikipedia.org/wiki/UTF-8
- BOM: https://en.wikipedia.org/wiki/Byte_order_mark
- Grapheme Clusters: https://hsivonen.fi/string-length/

Strings in Rust:
- https://www.simonwenkel.com/notes/programming_languages/rust/strings_with_rust.html
- https://dev.to/dsysd_dev/string-vs-str-in-rust-understanding-the-fundamental-differences-for-efficient-programming-4og8
- https://betterprogramming.pub/strings-in-rust-28c08a2d3130
- https://www.linkedin.com/pulse/understanding-string-str-utf-8-byte-arrays-inrust-luis-soares-m-sc--dnwcf/
- https://fasterthanli.me/articles/working-with-strings-in-rust
- String Conversion: https://stackoverflow.com/questions/41034635/how-do-i-convert-between-string-str-vecu8-and-u8
- Byte Strings: https://blog.burntsushi.net/bstr/
- Grapheme Clusters: https://hsivonen.fi/string-length/

### 8. Algorithms: Analysis, Design, and Evaluation

Reading:
- https://nnethercote.github.io/perf-book/introduction.html
- https://towardsdatascience.com/benchmarking-rust-compiler-settings-with-criterion-62db50cd62fb
- https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html
- Last resort - Assembly Output: https://rust.godbolt.org/, https://darkcoding.net/software/underrust-rust-assembly-output/


### 9. Lists

Recap:
- Rust Memory Allocation and Pointers

Data Structures:
- Linked Lists: Improving on our Exercise
- Doubly Linked Lists
- Skip Lists
- Dynamic Arrays (Vec<T>):

Algorithms for Sorting and Ordering:
- Bubble Sort
- Merge Sort (Divide and Conquer)
- Quicksort

Further Reading:
- https://rust-unofficial.github.io/too-many-lists/


### 10. Maps and Sets

Algorithms:
- Hashing

Data Structures:
- Maps
- Sets

Reading:
- https://nnethercote.github.io/perf-book/hashing.html#:~:text=The%20default%20hashing%20algorithm%20is,short%20keys%20such%20as%20integers
- https://en.wikipedia.org/wiki/Collision_attack
- https://link.springer.com/chapter/10.1007/978-3-642-34931-7_28
- https://www.reddit.com/r/rust/comments/52grcl/comment/d7kcei2/
- https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b

### 11. Trees

Data Structure:
- Heap
- Binary Search Tree
- Red-Black Tree
- Trie
- B-Tree

Algorithms
- Heap Sort


### 12. Graphs

- Graph Representation: Adjacency Matrix


## Further Reading

- [The Rules of Programming](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=30290181)
