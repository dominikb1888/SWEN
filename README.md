# Software Development - Computational Thinking and Software Engineering

## Readings and Resources:


### Required Reading:

Lecture:
- Book: [Algorithms and Data Structures with Python](https://link.springer.com/book/10.1007/978-3-031-42209-6)
- [Currently adapting to Rust](https://github.com/dominikb1888/ad-rust) (private repo, Ask for Access if necessary)

Exercise:
- [Guide to Competitive Programming](https://link.springer.com/book/10.1007/978-3-031-61794-2)

Rust Reference:
- [Programming Rust](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6643397)


### Additional Resources

- [https://rust-exercises.com/](https://rust-exercises.com/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- Rust Book, Brown University with great quizzes: [](https://rust-book.cs.brown.edu/)
- Rust by Example: https://doc.rust-lang.org/rust-by-example/index.html
- [Algorithms and Data Structures in Rust (Second half)](https://www.packtpub.com/product/hands-on-data-structures-and-algorithms-with-rust/9781788995528)



### Competitive Programming Deliverables

In each exercise we will attempt a challenging algorithmic problem from the CSES Problemset: https://cses.fi/problemset/

Use this template for Rust Code: https://github.com/dominikb1888/cses_template

You can also use https://github.com/csesfi/cses-cli/ to download the exercises and test cases.


## Class Design - Flipped Classroom

The course is taught in a Flipped Classroom style. You are required to prepare coding exercises and will be asked to present them in front of class on a random selection basis.

- 30 min | Review and joint development of selected Homework Exercises
- 60 min | Theoretical input on computational thinking and software engineering.
- 90 min | Applying theoretical input in a hands-on


## Exam (graded)

The Exam will be an oral exam on-site counting 60% of your grade. The 10 deliverables count 40%. The exam will ask about a solution you submitted as part of the 10 deliverables. Access to the oral exam requires complete submission of all 10 deliverables.


## Agenda

1. Algorithms Analysis, Design and Evaluation (Chapter 2)
2. Recursion
3. Sequences
4. Sets and Maps
5. Trees
6. Graphs
7. Membership Structures
8. Heaps
9. Balanced Binary Search Trees
10. B-Trees
11. Heuristic Search


## Deliverables

| Prepare before Session | Exercise | Topic |
|---|---|---|
| 2 |   |   |
| 3 | [Tower of Hanoi](https://cses.fi/ckvo8q5wh/task/2165)| Recursion |
| 4 | [Increasing Array](https://cses.fi/ckvo8q5wh/task/1094) | Dynamic Arrays |
| 5 | [Two Sets](https://cses.fi/ckvo8q5wh/task/1092) | Sets |
| 6 | [Subordinates](https://cses.fi/ckvo8q5wh/task/1674) | Trees |
| 7 | [Counting Rooms](https://cses.fi/ckvo8q5wh/task/1192) | Graphs |
| 8 | [Finding Borders](https://cses.fi/ckvo8q5wh/task/1732) | Membership Structures |
| 9 | [Another Game](https://cses.fi/ckvo8q5wh/task/2208) | Heaps |
| 10 | Free Choice | Balanced Binary Search Trees |
| 11 | Free Choice | B-Trees |
| 12 | Free Choice | Heuristic Search |


In total you have to submit 10 Deliverables during the semester. The deliverables will count 40% of your final grade. You can submit these via ilearn ().


## Sessions

### 1. Algorithms & Data Structures: Computational Complexity and Real-world Performance

Lecture:
- Introduction to Data Structures: [Simple Linked List](https://rust-unofficial.github.io/too-many-lists/first.html)
- Introduction to the class, exam, and deliverables
- Introduction to Estimating Algorithm Runtime


In-class Exercise:
- [Weird Algorithm](https://cses.fi/ckvo8q5wh/task/1068)

Required Reading:
- https://en.algorithmica.org/hpc/complexity/

Optional Reading
- https://nnethercote.github.io/perf-book/introduction.html
- https://towardsdatascience.com/benchmarking-rust-compiler-settings-with-criterion-62db50cd62fb
- https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html
- Last resort - Assembly Output: https://rust.godbolt.org/, https://darkcoding.net/software/underrust-rust-assembly-output/


### 2. Problem Solving Strategies: Recursion and others

Exercise Review:
- [Weird Algorithm](https://cses.fi/ckvo8q5wh/task/1068)

Theory:
- Recap: Memory Hierarchy and Processor Architecture
- Rust Arrays and Vectors in Memory
- Options, Results, and Smart Pointers in Rust

Interactive:
- Building a better Linked List (https://rust-unofficial.github.io/too-many-lists/second.html)



### 3. Sequences: Lists, Lists, and more Lists (and Smart Pointers)

Theory:
- Benchmarking and Profiling (Criterion, Cachegrind, Heaptrack)

Exercise Review:
- [Recap of Tower of Hanoi Excercise](https://youtu.be/rf6uf3jNjbo?si=z00L6RDw9yvRO63e)


Data Structures:
- Linked Lists: Improving on our Exercise
- Doubly Linked Lists
- Skip Lists
- Dynamic Arrays (Vec<T>):

Problem Solving Strategies (https://link.springer.com/chapter/10.1007/978-3-031-61794-2_2, https://www.designgurus.io/blog/grokking-the-coding-interview-patterns
): 
- I/O in Rust Reading and Writing from and to String, &str, &[u8]
- Bit Manipulation
- Recursion: [Understanding Recursion and its memory footprint](https://youtu.be/_JtPhF8MshA?si=k2bHPfkmPk-Y-m5N)- The Stack, the Heap, Pointers and Recursion:
    - https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html
    - https://www.varonis.com/blog/stack-memory-3
- Backtracking 


Links:
- as_ref and as_deref: https://academy.fpblock.com/blog/rust-asref-asderef/



### 4. Sequences continued: The performance implications of Lists, Arrays, and Vectors

Exercise Review:

Interactive:
- Rust Memory Allocation, Cache Lines, Registers

Algorithms for Sorting and Ordering:
- Bubble Sort
- Merge Sort (Divide and Conquer)
- Quicksort

Reading:
- https://developerlife.com/2025/05/19/rust-mem-latency/
- https://www.danielokoronkwo.com/post/memory-caches-cpus-a-practical-mental-model/
- Detailed Registers and Cache Visualization tool: https://ripes.me/


### 4. Sets and Maps:

Algorithms:
- Hashing

Data Structures:
- Maps
- Sets

Reading:
- https://medium.com/better-programming/implementing-a-hashmap-in-rust-35d055b5ac2b
- https://edgl.dev/blog/rust-hashmap/
- https://nnethercote.github.io/perf-book/hashing.html#:~:text=The%20default%20hashing%20algorithm%20is,short%20keys%20such%20as%20integers
- https://en.wikipedia.org/wiki/Collision_attack
- https://link.springer.com/chapter/10.1007/978-3-642-34931-7_28
- https://www.reddit.com/r/rust/comments/52grcl/comment/d7kcei2/
- https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
- https://louisabraham.github.io/articles/exact-cover
- https://www.geeksforgeeks.org/solving-sudoku-using-bitwise-algorithm/
- https://medium.com/@dt_emmy/sudoku-solver-using-rust-8a4e83d921fd


### 5. Trees

Data Structure:
- Heap
- Binary Search Tree
- Red-Black Tree
- Trie
- B-Tree

Algorithms
- Heap Sort


### 6. Graphs

- Graph Representation: Adjacency Matrix

### 7. Membership Structures

- Bloom Filters

### 8. Heaps

### 9. Balanced Binary Search Trees

### 10. B-Trees

### 11. Heuristic Search

### 12. Practice and Mock Exam


## Further Reading

- [The Rules of Programming](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=30290181)
