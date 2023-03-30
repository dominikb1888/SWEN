# Software Engineering - From REPL to SYSTEM

## Readings and Resources:

### CLASS CONTENT (1 of 2):
  - Foundations of Computer Science: https://www.teach.cs.toronto.edu/~csc110y/fall/notes/
  - Basic Python: https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218 (also in the md folder in this Repo)

### EXERCISES:
  - [Python](././python_assignments.md)
  - [Rust](././rust_assignments.md)
  - Algorithms and Data Structures: https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=4946360&query=algorithms (I will refer to this while discussing solutions to exercises)

### THEORY (optional): - Essential Computer Science: https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815

## Agenda

- 60-90min | Review and joint development of selected Homework Exercises
- 20-30min | Brief theoretical input on computational thinking with Algorithms and Data Structures.
- 60-90min | Practical Exercises and Coding Examples with Python

## Deliverables (ungraded)

Each session below comes with a set of exercises. We will do the first batch of exercises for Session 1 together. From Session 2 onwards you are encouraged to prepare all exercises associated to a certain session. I will provide feedback and guidance on the side. For each session, I will pick some difficult or important exercises and discuss solutions with the class. The exercises are plenty and you will hardly be able to complete all of them. Make sure to definitely work on all Exercises from Session 1 and 2.

## Exam (graded)

The Exam will be written and online. It will ask about theory and require you to work on several coding exercise.


## Sessions

### 1 - The Urge to Compute and Automate

#### Theory: How do we compute and automate and why?

- What would you like to automate?
- What do you think about DAOs?

    The ideal of a decentralized autonomous organization is easy to describe: it is an entity that lives on the internet and exists autonomously, but also heavily relies on hiring individuals to perform certain tasks that the automaton itself cannot do.

    - Vitalik Buterin, Ethereum Foundation


#### Further Reading:

- https://subpixel.space/entries/the-desire-for-full-automation/
- https://blog.ethereum.org/2014/05/06/daos-dacs-das-and-more-an-incomplete-terminology-guide/


#### Practice:

- From REPL commands to simple scripts | [Chapter 1, p.1-36](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=26)


### 2 - The Shoulders of Turtles

Theory: What are the layers of abstraction by creating different Versions of Hello World in Assembler, C, Python | [Chapter 1, p. 1-28](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=21), [Chapter 2, p.29-38](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=49)

#### Building Blocks/Layers of Programming Languages:

- Assembly (NASM): https://github.com/netwide-assembler/nasm (Example: https://en.wikipedia.org/wiki/Assembly_language#:~:text=A%20program%20written%20in%20assembly,ops)%2C%20comments%20and%20data.)
- C Compiler: https://github.com/mortdeus/legacy-cc
- CPython: https://github.com/python/cpython


#### Further Reading:

- https://archive.org/details/programsforelect00wilk/mode/2up
- https://www.nand2tetris.org/course
- https://github.com/Schweigi/assembler-simulator (https://www.mschweighauser.com/make-your-own-assembler-simulator-in-javascript-part1/)
- https://www.youtube.com/watch?v=tpIctyqH29Q&list=PL8dPuuaLjXtNlUrzyH5r6jN9ulIgZBpdo (Videos 1-10 explain really neatly all the abstraction levels between logic gates and programs)


#### Practice:

Python Built-in Data Types | [Chapter 2, pp. 37-82](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=49)


### 3 - Basic Elements of Programming Languages

#### Theory: Algorithm and Data Structure | [Chapter 3, pp. 53-81](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=72)
Understand your Algorithms: https://www.bigocheatsheet.com/


#### Practice: Conditionals and Iteration | [Chapter 3, pp. 83-114](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=140)

- Problem-solving with Data Structures and Algorithms
- Choosing the right data structure is key
- Solve problems iteratively:
  1. Understand the Problem  at Hand (Read Excercise Description carefully)
  2. Build a model (Paraphrase in your own words, Visualize, Translate into "Python")
  3. Validate the model (Look at test cases, especially at input and output data types)
  4. Get a quick win (Solve the problem as concrete as possible or only partially first, e.g. by copying test case)
  5. Test, Fail, and Repeat towards higher abstraction (Try to find more abstract approaches (variables, conditions, iterations) for parts of your code)
  6. Clean your code (Once all tests pass, challenge the current state of your Code and avoid repetitive patterns (DRY Principle))
  7. Celebrate!


### 4 - Programming Paradigms and Functional Programming

#### Theory: Programming Paradigms | [Chapter 3, pp.38-51](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=58)

#### Practice: Functions, the Building Blocks of Code [Chapter 4, pp. 115-156](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=140)

- https://maryrosecook.com/blog/post/a-practical-introduction-to-functional-programming


### 5 - Clean Code

#### Theory: Clean Code - Principles

#### Practice: Comprehensions and Generators | [Chapter 5, pp. 157-194](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=182)



### 6 - Understanding Data Bindings: OOP and FP

- Theory: Functions, data, classes and objects - [Chapter 2, pp.56-103](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=4946360&ppg=131)
- Practice: OOP, Decorators, and Classes - [Chapter 6, pp.195-238](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=220)

Additional Material:
- https://www.youtube.com/watch?v=8moWQ1561FY&t=3666s
- https://www.youtube.com/watch?v=HTLu2DFOdTg


### 7 - Error Handling, Exceptions and Context

- Theory: Operating Systems (1/2) | [Chapter 4, pp.81-130](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=58)

- Practice: Exceptions and Context Managers | [Chapter 7, pp. 239-256](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=264)


### 8 - Operating Systems

#### Theory: Operating System (2/2) | [Chapter 4, pp.81-131](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=99)

#### Practice: Files and Data Persistence | [Chapter 8, pp. 257-294](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=282)



### 9 - Mid-Term Exercise Review and Q&A

#### Theory: NONE

#### Practice: Collective Session on current issues and status of learning

### 10 - Testing

#### Theory: Software Development Lifecycle | [Chapter A, 241-247](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=258)
- [The testing Goat - obey!](https://www.obeythetestinggoat.com/pages/book.html#toc)


#### Practice: Testing | [Chapter 10, pp.315-342](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=340)



### 11 - Debugging and Profiling

#### Theory: Software Engineering Practices | [Chapter B, pp. 249-267](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=265)

#### Practice: Debugging and Profiling | [Chapter 11, pp. 343-364](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=368)

- https://martinheinz.dev/blog/64
- https://jakevdp.github.io/PythonDataScienceHandbook/01.07-timing-and-profiling.html
- https://stackoverflow.com/questions/17579357/time-time-vs-timeit-timeit

### 12 - Computer Networks: Servers and Clients

#### Theory: Computer Networks | [Chapter 6](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=150)

- Python requests and requests-cache: https://realpython.com/caching-external-api-requests/
- httpx for async requests

#### Practice: GUIs and Scripting | [Chapter 12, pp. 365-392](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=390)


### 13 - Computer Networks: Data Exchange

#### Theory: Computer Security | [Chapter 6, pp.165-194](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=58)

#### Practice: Cryptography and Tokes | [Chapter 9, pp. 295-314](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6797218&ppg=320)


### 14 - Computer Networks: Authentication and APIs

#### Theory: Cloud Computing [Chapter 7, 195-240](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6640815&ppg=182)
- https://realpython.com/api-integration-in-python
- https://betterprogramming.pub/asynchronous-programming-in-python-for-making-more-api-calls-faster-419a1d2ee058


### 15 - Final Exercise Review and Prep

#### Theory: NONE

#### Practice: Collective Session on current issues and status of learning


## Additional Material

Advanced Python: https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=5353672

