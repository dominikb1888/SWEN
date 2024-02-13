# Rust Ownership by Example

    Based on "Rust Ownership by Example" by Richard L. Apodaca (Updated 2021-05-01T15:50:00Z)

[Rust](https://www.rust-lang.org) is a safe systems programming language. Although C and C++ are systems languages, they're not safe. Specifically, Rust is a "[type safe language](http://www.pl-enthusiast.net/2014/08/05/type-safety/)", meaning that the compiler ensures that every program has well-defined behavior. Although other languages make the same guarantee, Rust does so without a garbage collector, runtime, or manual memory management.

Key to Rust's safety guarantee, and its most unusual feature, is the concept of _ownership_. When new Rust programmers run into trouble, the problem often is ownership. Master ownership, and Rust will make a lot of sense. Ignore ownership, and you'll fight the compiler to accomplish even the simplest tasks.

This beginner Rust tutorial, unlike most others, features ownership front-and-center. Short examples highlight the practical consequences of ownership. I'm no Rust expert. However, I have tried to choose examples that speak for themselves, and descriptions that jibe with reputable sources when possible.

## Getting Started

Running sample code and experimentation are helpful for learning Rust. The easiest way to do that is to copy and paste the examples here into the [Rust Playground](https://play.rust-lang.org) web application. If you'd rather compile and run code on your own machine, the easiest way to do that is [rustup](https://rustup.rs).

## Ownership Begins with Assignment

Like other languages, Rust uses the equals symbol (`=`) to assign a value to a variable. The variable then serves as a handle for later use.

The following program assigns the literal value `42` to the variable `x`, then prints the value of `x`.

```{rust}
    fn main() {
        let x = 42;

        println!("x: {}", x);
    }
```

Rust takes assignment one step further. The assignee (`x`) becomes the value's sole owner. There are no exceptions. The tight coupling between assignment and ownership sets the stage for many of Rust's unique capabilities.

## Ownership Ends with Scope

When a variable goes out of scope, its associated value, if any, is _dropped_. A dropped value can never be used again because the resources it uses are immediately freed.

This rule makes it easy to reason about the liveness of values. For as long as a variable remains in scope, the value it owns will never be dropped. By the time the owner goes out of scope, its value will have been dropped. A value can be dropped before the end of a scope if the compiler determines that the owner is no longer used within the scope.

We can see this rule in action by assigning a variable within an anonymous scope created by the left and right curly brace characters (`{` and `}`):

```{rust}
    fn main() {
        {
            let x = 42;

            println!("x: {}", x);
        }

        println!("x: {}", x); // ERROR: x not in scope
    }
```

The compiler rejects this code, informing us that the variable used in the second `println!` (`x`) is not in scope.

    8 |     println!("x: {}", x);
      |                       ^ not found in this scope


Most languages would not allow you to use `x` outside of its local scope, either. But in Rust, this limitation goes further. When the anonymous scope ends, the value owned by `x` (`42`) is dropped.

A slightly more advanced example proves the order of events. Some explanation might be useful. We define a custom type `DropMe` that implements the [`Drop` trait](https://doc.rust-lang.org/std/ops/trait.Drop.html) and its associated `drop` method. `drop` is called before an instance is dropped, printing a farewell message before digital oblivion. The remaining syntax will be explained later.

```{rust}
    #[derive(Debug)]
    struct DropMe;

    impl Drop for DropMe {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    fn main() {
        println!("Begin outer scope.");

        {
            println!("Begin inner scope.");

            let x = DropMe;

            println!("x: {:?}", x);
        }

        println!("End outer scope.");
    }
```

The program output demonstrates that the value owned by `x` is dropped when the variable goes out of scope:

    Begin outer scope.
    Begin inner scope.
    x: DropMe
    Dropping!
    End outer scope.


## Reassignment Moves Ownership

If assignment creates an ownership relationship, what about _reassignment_? Imagine we want to reassign the value owned by `a` to a new variable, `b`:

```{rust}
    fn main() {
        let a = vec![1, 2, 3]; // a growable array literal
        let b = a;             // move: `a` can no longer be used

        println!("b: {:?}", b);
    }
```

[Vectors](https://doc.rust-lang.org/std/vec/struct.Vec.html) (aka `Vec`) are Rust's growable array type. Vector literals are created with the `vec!` macro.

The above code compiles and runs, printing the result `b: [1, 2, 3]`. Following the rule that assignment creates an ownership relationship, we expect `b` to be the new owner. Given that a value can have only one owner, we further expect `a` to be uninitialized and therefore unusable. Both expectations are correct.

Reassignment of ownership (as in `let b = a`) is known as a _move_. A move causes the former assignee to become uninitialized and therefore not usable in the future.

We can confirm this by compiling:

```{rust}
    fn main() {
        let a = vec![1, 2, 3];
        let b = a;

        println!("a: {:?}\nb: {:?}", a, b); // error: borrow of moved value: `a`
    }
```

The compiler detects our attempt to reuse the now uninitialized `a` and complains:


    2 |     let a = vec![1,2,3];
      |         - move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    3 |     let b = a;
      |             - value moved here
    4 |
    5 |     println!("a: {:?}\nb: {:?}", a, b);
      |                                  ^ value borrowed here after move


This error message jumps the gun a bit, anticipating what we want to do and suggesting a way to proceed. We'll get to "borrowing" and the `Copy` trait later. For now, note the error on line 5, which was triggered because we tried to access `a` after a move.

It can sometimes be hard to spot a move. Consider what happens when we pass an argument to a function:

```{rust}
    fn sum(vector: Vec<i32>) -> i32 {
        let mut sum = 0; // mutability, more on this later

        for item in vector {
            sum = sum + item
        }

        sum
    }

    fn main() {
        let v = vec![1,2,3];
        let s = sum(v); // watch out, v was MOVED!

        println!("sum: {}", s);
    }
```

This code compiles and prints the result `sum: 6` as expected, However, it's easy to ignore the implicit move that takes place when calling `sum`. Specifically, the value owned by `v` is _moved_ to the `vector` parameter of the `sum` function.

If we were to use `v` after this move, the compiler would complain:

```{rust}
    fn sum(vector: Vec<i32>) -> i32 {
        let mut sum = 0;

        for item in vector {
            sum = sum + item
        }

        sum
    }

    fn main() {
        let v = vec![1,2,3];
        let s = sum(v);

        println!("sum of {:?}: {}", v, s); // ERROR: v was MOVED!
    }
```

In fact, we get essentially the same error (and cheerfully helpful suggestion) as with the more obvious reassignment.

Yet another form of reassignment occurs when returning a value from a function:

```{rust}
    fn create_series(x: i32) -> Vec<i32> {
        let result = vec![x, x+1, x+2];

        result
    }

    fn main() {
        let series = create_series(42);

        println!("series: {:?}", series);
    }
```

Fortunately, this form of reassignment doesn't cause the same problems as the other two because when a function exits, its corresponding scope ends. There's no way to later access the old scope or its local variables. We do, however, retain access to return values.

## Copy

What if we wanted to re-use a variable after a reassignment? The previous section showed what happens when we try the following.

```{rust}
    fn sum(vector: Vec<i32>) -> i32 {
        let mut sum = 0;

        for item in vector {
            sum = sum + item
        }

        sum
    }

    fn main() {
        let v = vec![1,2,3];
        let s = sum(v);

        println!("sum of {:?}: {}", v, s); // ERROR: v was MOVED!
    }
```

However, the following example compiles and runs just fine. Why can we use both `a` and `b` even though they've been passed as arguments to `sum`?

```{rust}
    fn sum(left: i32, right: i32) -> i32 {
        left + right
    }

    fn main() {
        let a = 42;
        let b = 1;
        let s = sum(a, b);

        println!("this sum of {} and {} is {}", a, b, s); // no error!
    }
```

Instead of moving the values owned by `a` and `b` to the parameters of `sum`, the values are _copied_. A copy creates an exact duplicate of a value that implements the [`Copy` trait](https://doc.rust-lang.org/std/marker/trait.Copy.html). Numerical values and several other inexpensive built-in Rust types support copy. Vectors do not.

The example with `Vec` fails to compile because `Vec` does not implement the `Copy` trait. The example with `i32` succeeds because this type supports copy.

This difference becomes obvious with user-defined types. Such types are created from a struct. For example, we can define a `Person` type:

```{rust}
    #[derive(Debug)]
    struct Person {
        age: i8
    }

    fn main() {
        let alice = Person { age: 42 };

        println!("alice: {:?}", alice);
    }
```

At the top of the `Person` definition sits a [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html). Its purpose is to automatically generate code. In the case of `#[derive(Debug)]` the generated code makes it possible to use `Person` in `println!` output.

Structs do not implement `Copy` by default. Reassignment of a `Person` value leads to a move, not a copy, as can be seen below:

```{rust}
    #[derive(Debug)]
    struct Person {
        age: i8
    }

    fn main() {
        let alice = Person { age: 42 };
        let bob = alice;

        println!("alice: {:?}\nbob: {:?}", alice, bob); // ERROR: alice moved
    }
```

Once again the compiler throws the familiar error: "value borrowed here after move."

We can, however, transform `Person` into a type that implements `Copy`. To do so, we can automatically derive the `Copy` trait just like the `Debug` trait was derived. For [reasons](https://doc.rust-lang.org/std/marker/trait.Copy.html#whats-the-difference-between-copy-and-clone) beyond the scope of this tutorial, `Clone` must also be derived.

```{rust}
    #[derive(Debug,Clone,Copy)]
    struct Person {
        age: i8
    }

    fn main() {
        let alice = Person { age: 42 };
        let bob = alice;

        println!("alice: {:?}\nbob: {:?}", alice, bob);
    }
```

Compiling and running this code yields the expected output:

    alice: Person { age: 42 }
    bob: Person { age: 42 }


This `Copy` fix works for values like `Person` that can be efficiently copied, but what about expensive values?

## Borrow

Many resources are too expensive in terms of time or memory be copied for every reassignment. In these cases, Rust offers the option to _borrow_.

Previously, we saw that a non-copyable value couldn't be reassigned. We can solve this problem by borrowing the value instead. To do so, we precede the assignee variable with the ampersand (`&`) character.

```{rust}
    #[derive(Debug)] // no more copy
    struct Person {
        age: u8
    }

    fn main() {
        let alice = Person { age: 8 };
        let bob = &alice; // bob borrows alice

        println!("alice: {:?}\nbob: {:?}", alice, bob);
    }
```

Despite the lack of a `Copy` trait on `Person`, the code above compiles and gives the same output as before:

    alice: Person { age: 42 }
    bob: Person { age: 42 }


Similarly, non-copyable value can be passed as an argument to a function if it is borrowed. Notice the use of borrow notation (`&`) in the signature for `sum`:

```{rust}
    fn sum(vector: &Vec<i32>) -> i32 { // borrow signature
        let mut sum = 0;

        for item in vector {
            sum = sum + item
        }

        sum
    }

    fn main() {
        let v = vec![1,2,3];
        let v_ref = &v;  // v_ref borrows v
        let s = sum(v_ref);

        println!("sum of {:?}: {}", v_ref, s); // no error
    }
```

The code above produces the expected result: `sum of [1, 2, 3]: 6`.

If assignment _always_ creates an ownership relationship, it may be surprising that the above code works. After all, `v_ref`, a reference value, is not passed by reference, yet it can still be accessed within `println!`. The answer is that with one notable exception [references themselves implement `Copy`](https://stackoverflow.com/questions/41413336). Although it may seem odd, the references in the examples so far are passed by value.

## Passing by Reference or Value

The preceding sections show how Rust lets us pass a value to a function either by reference or value. Here's a recap:

1.  If a value implements `Copy` and is not borrowed, it will be passed by value.
2.  If a value implements `Copy` and is borrowed, it will be passed by reference.
3.  If a value does not implement `Copy`, it must be borrowed and so will be passed by reference.
4.  References implement `Copy` and so are passed by value. There is one exception, which is described later.

Summarizing these rules in example form:

```{rust}
    fn pass_number_by_reference(number: &i8) -> bool {
        number.is_negative()
    }

    fn pass_number_by_value(number: i8) -> bool {
        number.is_negative()
    }

    fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
        vec.is_empty()
    }

    fn main() {
        // numbers implement Copy, and so can be passed by value or reference
        let number = 42;

        // does not move number because of borrow
        let is_negative_by_ref = pass_number_by_reference(&number);

        // moves number, which can never be used again
        let is_negative_by_value = pass_number_by_value(number);

        // copy not implemented - must be passed by reference
        let vec = vec![];

        // does not move vec
        let is_empty = pass_vec_by_reference(&vec);

        println!("is_negative_by_value: {}", is_negative_by_value);
        println!("is_negative_by_ref: {}", is_negative_by_ref);
        println!("vec {:?} is_empty: {}", vec, is_empty);
    }
```

## Borrowing and String Literals

String manipulation is an important capability in any language. In Rust, string literals are borrowed references. For example, consider:

```{rust}
    fn byte_length(string: &str) -> usize {
        string.bytes().len()
    }

    fn main() {
        let string = "🦀";
        let length = byte_length(string);

        println!("Bytes in \"{}\": {}", string, length);
    }
```

The compiler recognizes the value owned by `string` as the borrowed reference type `&str` (reference to `str`). Using `string` after calling `byte_length` in `println!` is allowed because the reference itself is copied into the `string` parameter of `byte_length`.

## Returning a Borrowed Value

Sometimes we'll want a function to return a borrowed value. For example, we might want to return the longer of two strings in terms of byte length. We might try the following.

```{rust}
    // Errors!
    fn longest(x: &str, y: &str) -> &str {
        if x.bytes().len() > y.bytes().len() {
            x
        } else {
            y
        }
    }

    fn main() {
        let alice = "Alice";
        let bob = "Bob";

        println!("{}", longest(alice, bob));
    }
```

However, we'd be greeted with a cryptic looking error referring to something called a "lifetime".

    1 | fn longest(x: &str, y: &str) -> &str {
      |                                 ^ expected lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`


## Lifetimes

A _lifetime_ is the scope within which a borrowed reference is valid. The Rust compiler is smart enough to infer lifetimes in many cases, meaning that we don't need to explicitly write them. This cuts both ways, though. When the compiler requires a lifetime, the concept seems alien.

Let's rewrite a previous example with an explicit, but unnecessary lifetime. This is accomplished by adding a _lifetime parameter_. A lifetime parameter can be added anywhere a borrowed reference appears. Like type parameters (aka "generics"), a lifetime parameter must be brought into scope before it can be used. We do this by placing the parameter into the angle brackets (`<` and `>`) following the name of the function. This is also where type parameter declarations go.

```{rust}
    fn byte_length<'a>(string: &'a str) -> usize { // unnecessary lifetime
        string.bytes().len()
    }

    fn main() {
        let string = "🦀";
        let length = byte_length(string);

        println!("Bytes in \"{}\": {}", string, length);
    }
```

This example compiles and runs just like before. There are only two differences: (1) the lifetime parameter declaration `<'a>` after `byte_length`; and (2) the lifetime parameter `'a` immediately following the ampersand in the parameter `string`'s type definition. The name of a lifetime parameter (e.g., `'a`) begins with an apostrophe symbol (`'`) and ends with one or more characters - typically just one. The content within angle brackets brings the lifetime parameter `'a` into scope.

The previous section presented this failing example:

```{rust}
    fn longest(x: &str, y: &str) -> &str {     // error: expected lifetime parameter
        if x.bytes().len() > y.bytes().len() {
            x
        } else {
            y
        }
    }

    fn main() {
        let alice = "Alice";
        let bob = "Bob";

        println!("{}", longest(alice, bob));
    }
```

This won't compile because we have not yet constrained the lifetime of the return value. Without bounds on this value, there is no way to exclude cases in which the `longest` function returns a reference to a dropped value. Adding a constraint in the form of a lifetime parameter excludes those cases. The idea is analogous to a type parameter that excludes cases in which a variable holds a value of incompatible type.

With this idea in mind, we update the example:

```{rust}
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.bytes().len() > y.bytes().len() {
            x
        } else {
            y
        }
    }

    fn main() {
        let alice = "Alice";
        let bob = "Bob";

        println!("{}", longest(alice, bob));
    }
```

The change allows the compiler to determine that the lifetime (valid scope) of the value whose borrowed reference it returns matches the lifetime of the parameters `x` and `y`. In other words, there is no way for the `longest` function to return a reference to a dropped value.

It's tempting to think of lifetime parameters as a way to make a reference "live longer." A better approach is to consider the similarity between type parameters and lifetime parameters. A type parameter constrains the range of possible types for a value owned by a variable. Likewise, a lifetime parameter constrains the range of possible lifetimes (valid scopes) for a value owned by a variable.

## Lifetimes and Structs

Function parameters aren't the only context in which lifetimes appear. When a struct declares a member with a reference type, the lifetime of that member must also be declared. The example below won't compile.

```{rust}
    #[derive(Debug)]
    struct Person {
        name: &str // error: expected lifetime parameter
    }

    fn main() {
        let alice = Person { name: "Alice" };

        println!("alice: {:?}", alice);
    }
```

Here, the compiler can't rule out cases in which the `name` member is dropped before its enclosing `Person` instance. We can correct this problem by adding a lifetime parameter `'a`. In doing so, we notify the compiler that `name` will live at least as long as its parent.

```{rust}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str
    }

    fn main() {
        let alice = Person { name: "Alice" };

        println!("alice: {:?}", alice);
    }
```

## Mutability

So far we've only considered values that never change. But writing real software requires state changes — mainly through changes to the state of values. This capability of changing the state of a value is known as _mutability_.

Mutability may seem unrelated to ownership, but the two concepts are tightly coupled. In particular, mutability constrains your ability to borrow references.

In Rust, variables own immutable values by default. We can override this behavior by preceding a variable with the `mut` keyword.

For example, we're unable to add members to a `Vec` by default:
```{rust}
    fn main() {
        let numbers = vec![1, 2, 3];

        numbers.push(4); // ERROR: cannot borrow as mutable

        println!("numbers: {:?}", numbers);
    }
```

Making the owner mutable solves this problem.

```{rust}
    fn main() {
        let mut numbers = vec![1, 2, 3];

        numbers.push(4);  // mutable Vec supports push

        println!("numbers: {:?}", numbers);  // numbers: [1, 2, 3, 4]
    }
```

## MARSAW: Multiple Active Readers or Single Active Writer

Mutability constrains our ability to borrow references. The book _[Programming Rust](https://www.oreilly.com/library/view/programming-rust/9781491927274/)_ refers to the high-level concept as _multiple readers or single writer_.

_[The Rust Programming Language](https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html#the-rules)_ describes the same principle like so:

> … you may have one or the other of these two kinds of borrows, but not both at the same time:
>
> * one or more references (`&T`) to a resource,
> * exactly one mutable reference (`&mut T`).

Neither description, however, quite hits the mark. First, the rule applies to both borrowed references and _owners_. Second, the rule only applies when _active_ readers and writers are involved. As such, it might be more instructive to recast the rule as: "multiple active readers or single active writer" (MARSAW).

The term "active" deserves some explanation. Until a writer's mutable API is used, it's inactive. After mutation, the writer stays active for the duration of its lifetime. A reader borrowed before the activation of a writer will become active on the first use of the owner's immutable API.

Let's look at some examples to make these ideas more concrete.

One _inactive_ writer and one _inactive_ reader can co-exist within the same scope, as demonstrated below. You'll notice warnings about unused variables, but no errors.
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];   // inactive
        let reader = &writer;           // inactive
    }
```

We can read without triggering an error, although we'll receive an additional warning about unneeded mutability on `writer`:
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];
        let reader = &writer;

        println!("len: {}", reader.len()); // no error, inactive writer
    }
```

Likewise, we write without triggering a compiler error:
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];
        let reader = &writer;

        writer.push(4);  // no error, inactive reader
    }
```

We can take this further with sequential read-write:
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];
        let reader = &writer;

        println!("len: {}", reader.len()); // no error, inactive writer

        writer.push(4);                    // no error, inactive reader
    }
```

We can exercise `writer`'s immutable API, then read without producing an error:
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];
        let reader = &writer;

        writer.len();

        println!("len: {}", reader.len()); // no error: inactive reader and writer
    }
```

What we can't do is activate `writer` then use `reader`. The following code produces an error because the `println!` statement generates an active reader, which when paired with the active writer is not allowed:
```{rust}
    fn main() {
        let mut writer = vec![1,2,3];
        let reader = &writer;

        writer.push(4);                    // error: cannot borrow `writer` as mutable because it is also borrowed as immutable

        println!("len: {}", reader.len());
    }
```


The error message explains the situation:

    3 |     let reader = &writer;
      |                  ------- immutable borrow occurs here
    4 |
    5 |     writer.push(4);                    // active writer, inactive reader
      |     ^^^^^^^^^^^^^^ mutable borrow occurs here
    6 |
    7 |     println!("len: {}", reader.len()); // error: cannot borrow `writer` as mutable because it is also borrowed as immutable
      |                         ------ immutable borrow later used here


We can eliminate the error by pulling the borrow down after the call to `push`.

```{rust}
    fn main() {
        let mut writer = vec![1,2,3];

        writer.push(4);

        let reader = &writer;

        println!("len: {}", reader.len()); // no error, reader is not active because it was borrowed _after_ last writer mutation
    }
```


Even though a variable might be declared as `mut`, it can nevertheless be used to read. Note that `writer.iter` triggers an implicit immutable borrow. This does not violate MARSAW, however, because the borrow occurs after the last mutation

```{rust}
    fn main() {
        let mut writer = vec![1,2,3];

        writer.push(4);

        for number in writer.iter() {
            println!("number: {}", number); // no error, implicit borrow occurs after writer mutation
        }
    }
```

Implicit borrowing can nevertheless lead to a MARSAW violation if coupled to a write. The following code won't compile because the `iter` method implicitly borrows an immutable reference, creating a simultaneously active reader and writer:

```{rust}
    fn main() {
        let mut writer = vec![1,2,3];

        for number in writer.iter() {
            writer.push(number + 2); // ERROR: cannot borrow `writer` as mutable because it is also borrowed as immutable
        }
    }
```

The error (below) lets us know where borrowing occurs. Used together with the MARSAW principle, we can identify the problem's source as an _active_ reader and writer operating simultaneously.

    4 |     for number in writer.iter() {
      |                   -------------
      |                   |
      |                   immutable borrow occurs here
      |                   immutable borrow later used here
    5 |         writer.push(number + 2); // ERROR: cannot borrow `writer` as mutable because it is also borrowed as immutable
      |         ^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here


## Conclusion

Ownership permeates Rust, so it's critical to understand it early in your study of the language. This guide offers some simple examples illustrating how ownership works. Key points can be summarized as:

1. Assignment always binds a value to a variable, which becomes the value's sole owner.
2. Passing and returning by value both count as assignment.
3. A value will always be dropped by the time its owner goes out of scope.
4. Reassignment of a value results in a move, or change of ownership.
5. After a move, the former assignee can never be used again.
6. A reference can be borrowed through reassignment by preceding its owner with the ampersand symbol (`&`).
7. A borrowed reference may not live longer than the underlying value.
8. A lifetime parameter, written with an apostrophe character (`'`) followed by at least one letter, represents the scope of a borrowed reference.
9. A lifetime parameter identifies two references as having the same minimum scope.
10. The MARSAW principle allows for either multiple readers or a single writer to be active at the same time, but not both. Owners and references both count.

