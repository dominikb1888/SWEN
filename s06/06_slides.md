---
title: Session 6 - Functional Programming in Rust: Closures, Iterators, and Collections

---

---
# Agenda

1. Closures
    - Capturing Variables with closures
    - Using closures as arguments
2. Iterators and Higher-order Functions
    - Iterator and IntoIterator traits
    - Iterators and Higher-order Functions
    - Implementing iterators for your types
        - LinkedList example
        - Binary Tree example
3. Collections
    - Brief overview on collection types

---
### Capturing Variables with Closures

-  Borrowing
-  Stealing

---
### Function and Closure types

```{rust}
fn(&City) -> bool // fn type (functions only)
Fn(&City) -> bool // Fn trait (both functions and closures)
```

---
### Example (1/3)

Type Error. Why?

```{rust}
/// Given a list of cities and a test function,
/// return how many cities pass the test.
fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
           count += 1;
        }
    }
    count
}

/// An example of a test function. Note that the type of
/// this function is `fn(&City) -> bool`, the same as
/// the `test_fn` argument to `count_selected_cities`.
fn has_monster_attacks(city: &City) -> bool {
    city.monster_attack_risk > 0.0
}

// How many cities are at risk for monster attack?
let n = count_selected_cities(&my_cities, has_monster_attacks);
```
---
### Example (2/3)

Changed type signature.
```{rust}
fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize where F: Fn(&City) -> bool {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}
```

---
### Example (3/3)

We have changed only the type signature of count_selected_cities, not the body. The new version is generic. It takes a test_fn of any type F as long as F implements the special trait Fn(&City) -> bool. This trait is automatically implemented by all functions and most closures that take a single &City as an argument and return a Boolean value:
```{rust}
fn(&City) -> bool // fn type (functions only)
Fn(&City) -> bool // Fn trait (both functions and closures)
```
This special syntax is built into the language. The -> and return type are optional; if omitted, the return type is ().


---
### Closure Performance

- Usually closures are heap allocated, dynamically dispatched, and garbage collected (e.g. in Python). No inlining possible

In Rust:
- Rust does not have any performance drawbacks when using closures
- Only heap allocated if inside Box or Vec
- Closure as distinct type allows inlining

Discuss Figure 14-1, page 335

---
### Closures and Safety


Mutability:
- Fn is the family of closures and functions that you can call multiple times without restriction. This highest category also includes all fn functions.
- FnMut is the family of closures that can be called multiple times if the closure itself is declared mut.
- FnOnce is the family of closures that can be called once, if the caller owns the closure.

These are subtraits: FnOnce() > FnMut() > Fn()

Copy and Clone:
- All non-move closure are Copy and Clone automatically
- A non-move closure that does not mutate holds references which are Copy and Clone, which makes the whole closure Copy and Clone

---
###  Callbacks

Callback: Function provided by the user for a library to call later



---
### Example - Regular Functions
```{rust}
App::new()
        .route("/", web::get().to(get_index))
        .route("/gcd", web::post().to(post_gcd))
```

---
### Example - Closures:

```{rust}
App::new()
    .route("/", web::get().to(|| {
            HttpResponse::Ok()
                .content_type("text/html")
                .body("<title>GCD Calculator</title>...")
        }))
    .route("/gcd", web::post().to(|form: web::Form<GcdParameters>| {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("The GCD of {} and {} is {}.",
                             form.n, form.m, gcd(form.n, form.m)))
        }))
```

---
### Iterators and Higher-order Functions

- A iterator is a value that produces a series of values
- Iterators are used to traverse collections
- Iterators can also work on streaming data (lines of text, server connections)
- You can use loops (imperative style) or higher-order functions (functional style) on them


---
### Imperative vs functional

```{rust}
/// imperative
fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}
/// functional
fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}
```

Let's discuss!

---
### Iterator() and IntoIterator()



---
### Iterator lingo

- As we’ve said, an iterator is any type that implements Iterator.
- An iterable is any type that implements IntoIterator: you can get an iterator over it by calling its into_iter method. The vector reference &v is the iterable in this case.
- An iterator produces values.
- The values an iterator produces are items.
- The code that receives the items an iterator produces is the consumer.

---
### Creating Iterators

- for ... in (a variant of IntoIterator)
- Iterator | iter()
- IntoIterator | into_iter()
- Closure |  from_fn
- drain()

---
### Iterator Sources

- Range
- RangeFrom
- RangeInclusive
- Option
- Result
- Vec
- String
- ...

See Table 15-1, Page 356-358

---
### Iterators and Higher-order Functions

Higher-order Functions: Function that takes another function as argument

- Iterator Adapters: Higher order Functions (map and filter, filter_map and flat_map, flatten, take and take_while, skip and skip_while, peekable,chain, enumerate, zip, by_ref, cycle)
- Consuming iterators (count, sum, product, max, min, max_by, min_by, max_by_key, min_by_key)
- Comparing item sequences (any, all, position, rposition, reduce, fold, rfold,  try_fold, try_rfold, nth, nth_back, last, find, rfind, find_map)
- Building collections (collect, from_iterator, extend, partition, for_each)

---
### Collection Types

See Table 16-1, Page 394
