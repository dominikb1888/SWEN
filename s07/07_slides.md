

---
### Chars in Memory

- Computer memory is laid out to store bytes (8 bit ones) in a sequence, one after another in common computers today.
- Bytes can mean anything. We are the ones who happen to be in consensus in certain areas and give meaning to them. With this power, we can also interpret them as characters. People made tables to agree on which bytes should map to which characters in history. See ASCII or Unicode tables. The first one is a minimal table in which every different character can be expressed with a single byte but in the latter, it may even take 4 bytes to define a single character. Check this thread out. Unicode is significantly a larger table that covers a vast number of characters and still has empty room.
- Strings are sequences of bytes that we promise to interpret complying to a character table. We make this promise by using the type system of the programming language we use. We achieve this in Rust by simply calling a chunk of memory a String or str or &str or &String or Box<str> or Box<&str> or..

---
### Strings in C

Strings in C are Null-terminated: https://en.wikipedia.org/wiki/Null-terminated_string

```{C}
// Allocate an immutable array in stack
// which is 7 bytes in size and fill it with the chars of "banana"
// C inserts the character '\0' in the end.
char string[] = "banana";

// Allocate an immutable array in the stack
// which is 42 bytes in size and fill it with the chars of "banana"
// C inserts the character '\0' in the 6th position.
char long_mostly_empty_string[42] = "banana";

// Do the first example in an explicit way.
char just_a_different_initialization[] = {'b', 'a', 'n', 'a', 'n', 'a', '\0'};
view raw
```

---
### Memory allocation in C

```{c}
#include <stdlib.h>
#include <string.h>

int main() {
  // A pointer to an immutable string allocated on data section
  const char *string_literal = "banana";
  // A pointer to a mutable string allocated on heap
  char *heap_string = (char*) malloc(7*sizeof(char));

  // How would "strcpy" know the length of "banana" which is the pointee
  // of "const char *string_literal"?

  // It will start walking the memory from
  // where the pointer points to and count every step.
  // When it matches the '\0' character (a null byte) it will break the loop!

  // Check the implementation of strcpy after the example.
  strcpy(heap_string, string_literal);

  //..

  // Don't forget to free your heap allocations when you're done with them,
  // since you're writing C, you need to clean your own mess ;)
  free(heap_string);

  return 0;
}
```

---
### Strings in JavaScript

JS stores the length of the string as part of the Object: https://v8docs.nodesource.com/node-0.8/d2/db3/classv8_1_1_string.html

```{js}
let string = "banAna".toLowerCase();
let concatinatedString = "I want a cherry " + string;
let shout = concatinatedString.toUpperCase() + "!!";

let string = new String("banana");
```

Source: https://betterprogramming.pub/strings-in-rust-28c08a2d3130

---
### Wrap-up:

- C strings do not enforce any encoding internally. They are just a sequence of plain bytes waiting to be interpreted which have a null terminator.
- Java Script strings use UTF-16 encoding.
- Rust strings are UTF-8 encoded.


---
### String

The String type in Rust is a growable, heap-allocated, UTF-8 encoded string.

It is part of the Rust standard library and is widely used when you need a mutable and dynamic string that can be modified and resized at runtime.

You can create a new String by calling its associated function String::new() or by converting from a string slice using the to_string() method.

```{rust}
let mut my_string = String::new();
my_string.push_str("Hello, ");
my_string.push('R');
my_string.push('u');
my_string.push('s');
my_string.push('t');

println!("{}", my_string); // Output: "Hello, Rust"
```
---
### str (String Slice)

The str type in Rust is a borrowed string slice, also known as a string reference.

It represents a view into an existing UTF-8 encoded string data. String slices are immutable and have a fixed size that cannot be modified.

String slices are commonly used to pass string data without transferring ownership, reducing the need for unnecessary memory copies.

```{rust}
fn print_greeting(greeting: &str) {
    println!("{}", greeting);
}

let my_string = "Hello, Rust!";
print_greeting(my_string); // Output: "Hello, Rust!"
```
---
### Differences between "String" and "str" (String Slice)

Ownership:
- String owns the data it represents and is responsible for its memory allocation and deallocation.
- str (String Slice) is a borrowed reference to a portion of an existing string or string literal, and it does not own the data.

Mutability:
- String is mutable, meaning you can modify its content after creation.
- str (String Slice) is immutable, and you cannot modify its content.

Size:
- String can dynamically grow or shrink as needed and has a flexible size.
- str (String Slice) has a fixed size and represents a subset of a larger string data.

Allocation:
- String is heap-allocated, and its memory is managed by the Rust standard library.
- str (String Slice) does not require any heap allocation and is often used as borrowed references to string literals or other String types.

---
### But what is a view
When we say that a "str" (String Slice) represents a view into an existing UTF-8 encoded string data, it means that the "str" type does not own the actual string data.

Instead, it provides a read-only reference to a portion of an existing UTF-8 encoded string.

---
### Let's break it down
View
A "view" in this context means that the str type acts as a window or a lens through which you can observe a part of a larger string.

It allows you to look at and interact with that specific section of the string without actually copying or owning the entire string data.

---
### Existing UTF-8 Encoded String Data

The data referred to by a str must already exist somewhere in memory.

It could be a string literal (a sequence of characters enclosed in double quotes) or a part of a heap-allocated "String" type.

In either case, the string data must be UTF-8 encoded, as Rust enforces UTF-8 encoding for all its string types.

---
### UTF-8 Encoded

UTF-8 is a character encoding standard that represents characters in a way that ensures compatibility with the ASCII character set while also supporting a vast range of international characters.

Rust's str type enforces UTF-8 encoding to guarantee that strings can represent any valid Unicode text.

---
### UTF-8 Example

Here's an example to illustrate the concept:

```{rust}
fn main() {
    let my_string = String::from("Hello, Rust!"); // A heap-allocated String
    let my_slice: &str = &my_string[0..5]; // A string slice representing "Hello"

    println!("Original String: {}", my_string);
    println!("String Slice: {}", my_slice);
}
```

---
### UTF-8 Example Explanation

In this example, we have a heap-allocated String called my_string, containing the text "Hello, Rust!".

The line let my_slice: &str = &my_string[0..5]; creates a string slice my_slice that borrows a part of my_string, specifically the characters from index 0 to 4 ("Hello").

The string slice my_slice does not own the data; it simply provides a view into the existing string data held by my_string.

The use of string slices allows Rust to efficiently work with and manipulate parts of strings without incurring unnecessary memory copies, making it a memory-safe and high-performance language for string handling.
