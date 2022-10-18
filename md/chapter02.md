# Built-In Data Types

    "Data! Data! Data!" he cried impatiently. "I can't make bricks without clay."
    – Sherlock Holmes, in The Adventure of the Copper Beeches

Everything you do with a computer is managing data. Data comes in many different shapes and flavors. It's the music you listen to, the movies you stream, the PDFs you open. Even the source of the chapter you're reading at this very moment is just a file, which is data.

Data can be simple, whether it is an integer number to represent an age, or complex, like an order placed on a website. It can be about a single object or about a collection of them. Data can even be about data—that is, metadata. This is data that describes the design of other data structures, or data that describes application data or its context. In Python, objects are our abstraction for data, and Python has an amazing variety of data structures that you can use to represent data or combine them to create your own custom data.

In this chapter, we are going to cover the following:

---
## Agenda 

- Python objects' structures
- Mutability and immutability
- Built-in data types: [numbers](numbers), strings, dates and times, sequences, collections, and mapping types
- (The collections module & Enumerations)
- Discussion: Choosing Data Types 


---

## TOC

- [Built-In Data Types](#built-in-data-types)
  - [Agenda](#agenda)
  - [TOC](#toc)
  - [Everything is an object](#everything-is-an-object)
  - [Mutable or immutable? That is the question](#mutable-or-immutable-that-is-the-question)
  - [Numbers](#numbers)
    - [Integers](#integers)
    - [Booleans](#booleans)
    - [Real numbers (Floats)](#real-numbers-floats)
    - [Complex numbers](#complex-numbers)
    - [Fractions and decimals](#fractions-and-decimals)
  - [Immutable sequences](#immutable-sequences)
    - [Strings and bytes](#strings-and-bytes)
      - [Encoding and decoding strings](#encoding-and-decoding-strings)
      - [Indexing and slicing strings](#indexing-and-slicing-strings)
      - [String formatting](#string-formatting)
    - [Tuples](#tuples)
  - [Mutable sequences](#mutable-sequences)
    - [Lists](#lists)
    - [Bytearrays](#bytearrays)
  - [Set types](#set-types)
    - [Mapping types: dictionaries](#mapping-types-dictionaries)
  - [Data types](#data-types)
    - [The standard library](#the-standard-library)
    - [Third-party libraries](#third-party-libraries)
    - [The collections module](#the-collections-module)
      - [namedtuple](#namedtuple)
      - [defaultdict](#defaultdict)
      - [ChainMap](#chainmap)
    - [Enums](#enums)
  - [Final considerations](#final-considerations)
    - [Small value caching](#small-value-caching)
  - [How to choose data structures](#how-to-choose-data-structures)
  - [About indexing and slicing](#about-indexing-and-slicing)
  - [About names](#about-names)
  - [Summary](#summary)

## Everything is an object

Before we delve into the specifics, we want you to be very clear about objects in Python, so let's talk a little bit more about them. As we already said, everything in Python is an object. But what really happens when you type an instruction like `age = 42` in a Python module?

If you go to http://pythontutor.com/, you can type that instruction into a text box and get its visual representation. Keep this website in mind; it's very useful to consolidate your understanding of what goes on behind the scenes.

So, what happens is that an object is created. It gets an id, the type is set to int (integer number), and the value to 42. A name, age, is placed in the global namespace, pointing to that object. Therefore, whenever we are in the global namespace, after the execution of that line, we can retrieve that object by simply accessing it through its name: age.

If you were to move house, you would put all the knives, forks, and spoons in a box and label it cutlery. This is exactly the same concept. Here is a screenshot of what it may look like (you may have to tweak the settings to get to the same view):

So, for the rest of this chapter, whenever you read something such as `name = some_value`, think of a name placed in the namespace that is tied to the scope in which the instruction was written, with a nice arrow pointing to an object that has an id, a type, and a value. There is a little bit more to say about this mechanism, but it's much easier to talk about it using an example, so we'll come back to this later.


---
## Mutable or immutable? That is the question

The first fundamental distinction that Python makes on data is about whether or not the value of an object can change. If the value can change, the object is called mutable, whereas if the value cannot change, the object is called immutable. It is very important that you understand the distinction between mutable and immutable because it affects the code you write; take this example:

```python
>>> age = 42
>>> age
42
>>> age = 43  #A
>>> age
43
```

In the preceding code, on line #A, have we changed the value of age? Well, no. But now it's 43 (we hear what you are saying...). Yes, it's 43, but 42 was an integer number, of the type int, which is immutable. So, what happened is really that on the first line, age is a name that is set to point to an int object, whose value is 42. When we type age = 43, what happens is that another object is created, of the type int and value 43 (also, the id will be different), and the name age is set to point to it. So, in fact, we did not change that 42 to 43—we actually just pointed age to a different location, which is the new int object whose value is 43. Let's see the same code also printing the IDs:

```python
>>> age = 42
>>> id(age)
4377553168
>>> age = 43
>>> id(age)
4377553200
```

Notice that we print the IDs by calling the built-in id() function. As you can see, they are different, as expected. Bear in mind that age points to one object at a time: 42 first, then 43—never together.

If you reproduce these examples on your computer, you will notice that the IDs you get will be different. This is of course expected, as they are generated randomly by Python, and will be different every time.

Now, let's see the same example using a mutable object. For this example, let's just use a Person object, that has a property age (don't worry about the class declaration for now—it is there only for completeness):

```python
>>> class Person:
...     def __init__(self, age):
...         self.age = age
...
>>> fab = Person(age=42)
>>> fab.age
42
>>> id(fab)
4380878496
>>> id(fab.age)
4377553168
>>> fab.age = 25  # I wish!
>>> id(fab)  # will be the same
4380878496
>>> id(fab.age)  # will be different
4377552624
```

In this case, we set up an object fab whose type is Person (a custom class). On creation, the object is given the age of 42. We then print it, along with the object ID, and the ID of age as well. Notice that, even after we change age to be 25, the ID of fab stays the same (while the ID of age has changed, of course). Custom objects in Python are mutable (unless you code them not to be). Keep this concept in mind, as it's very important. We'll remind you about it throughout the rest of the chapter.

---
## Numbers

Let's start by exploring Python's built-in data types for numbers. Python was 
designed by a man with a master's degree in mathematics and computer science, so 
it's only logical that it has amazing support for numbers. 

    Numbers are immutable objects.

### Integers

Python integers have an unlimited range, subject only to the available virtual memory. This means that it doesn't really matter how big a number you want to store is—as long as it can fit in your computer's memory, Python will take care of it. 

Integer numbers can be positive, negative, or 0 (zero). They support all the basic mathematical operations, as shown in the following example:

```python
>>> a = 14
>>> b = 3
>>> a + b  # addition
17
>>> a - b  # subtraction
11
>>> a * b  # multiplication
42
>>> a / b  # true division
4.666666666666667
>>> a // b  # integer division
4
>>> a % b  # modulo operation (reminder of division)
2
>>> a ** b  # power operation
2744
```

The preceding code should be easy to understand. Just notice one important thing: Python has two division operators, one performs the so-called true division (/), which returns the quotient of the operands, and another one, the so-called integer division (//), which returns the floored quotient of the operands. 

It might be worth noting that in Python 2 the division operator / behaves differently than in Python 3.

Let's see how division behaves differently when we introduce negative numbers:

```python
>>> 7 / 4  # true division
1.75
>>> 7 // 4  # integer division, truncation returns 1
1
>>> -7 / 4  # true division again, result is opposite of previous
-1.75
>>> -7 // 4  # integer div., result not the opposite of previous
-2
```

This is an interesting example. If you were expecting a -1 on the last line, don't feel bad, it's just the way Python works. Integer division in Python is always rounded toward minus infinity. If, instead of flooring, you want to truncate a number to an integer, you can use the built-in int() function, as shown in the following example:

```python
>>> int(1.75)
1
>>> int(-1.75)
-1
```

Notice that the truncation is done toward 0.

The int() function can also return integer numbers from string 
representation in a given base:


```python
>>> int('10110', base=2)

22 
```

It's worth noting that the power operator, **, also has a built-in function counterpart, pow(), shown in the example below:

```python
>>> pow(10, 3)
1000.0  # result is float
>>> 10 ** 3
1000  # result is int
>>> pow(10, -3)
0.001
>>> 10 ** -3
0.001
```

There is also an operator to calculate the remainder of a division. It's called the modulo operator, and it's represented by a percentage symbol (%):

```python
>>> 10 % 3  # remainder of the division 10 // 3
1
>>> 10 % 4  # remainder of the division 10 // 4
2
```

The pow() function allows a third argument to perform modular exponentiation. The form with three arguments now accepts a negative exponent in the case where the base is relatively prime to the modulus. 

The result is the modular multiplicative inverse of the base (or a suitable power of that, when the exponent is negative, but not -1), modulo the third argument. Here's an example:

```python
>>> pow(123, 4)
228886641
>>> pow(123, 4, 100)
41  # notice: 228886641 % 100 == 41
>>> pow(37, -1, 43)  # modular inverse of 37 mod 43
7
>>> 7 * 37 % 43  # proof the above is correct
1
```

One nice feature introduced in Python 3.6 is the ability to add underscores within number literals (between digits or base specifiers, but not leading or trailing). The purpose is to help make some numbers more readable, such as 1_000_000_000:

```python
>>> n = 1_024
>>> n
1024
>>> hex_n = 0x_4_0_0  # 0x400 == 1024
>>> hex_n
1024
```


### Booleans

Boolean algebra is that subset of algebra in which the values of the variables are the truth values, true and false. In Python, True and False are two keywords that are used to represent truth values. Booleans are a subclass of integers, so True and False behave respectively like 1 and 0. The equivalent of the int class for Booleans is the bool class, which returns either True or False. Every built-in Python object has a value in the Boolean context, which means they basically evaluate to either True or False when fed to the bool function. We'll see all about this in Chapter 3, Conditionals and Iteration. Boolean values can be combined in Boolean expressions using the logical operators and, or, and not. Again, we'll see them in full in the next chapter, so for now let's just see a simple example:

```python
>>> int(True)  # True behaves like 1
1
>>> int(False)  # False behaves like 0
0
>>> bool(1)  # 1 evaluates to True in a Boolean context
True
>>> bool(-42)  # and so does every non-zero number
True
>>> bool(0)  # 0 evaluates to False
False
>>> # quick peek at the operators (and, or, not)
>>> not True
False
>>> not False
True
>>> True and True
True
>>> False or True
True
```

You can see that True and False are subclasses of integers when you try to add them. Python upcasts them to integers and performs the addition:

```python
>>> 1 + True
2
>>> False + 42
42
>>> 7 - True
6
```

Upcasting is a type conversion operation that goes from a subclass to its parent. In this example, True and False, which belong to a class derived from the integer class, are converted back to integers when needed. This topic is about inheritance and will be explained in detail in Chapter 6, OOP, Decorators, and Iterators.


### Real numbers (Floats)

Real numbers, or floating point numbers, are represented in Python according to the IEEE 754 double-precision binary floating point format, which is stored in 64 bits of information divided into three sections: sign, exponent, and mantissa.

Quench your thirst for knowledge about this format on Wikipedia: http://en.wikipedia.org/wiki/Double-precision_floating-point_format.

Several programming languages give coders two different formats: single and double precision. The former takes up 32 bits of memory, the latter 64. Python supports only the double format. Let's see a simple example:

```python
>>> pi = 3.1415926536  # how many digits of PI can you remember?
>>> radius = 4.5
>>> area = pi * (radius ** 2)
>>> area
63.617251235400005
```

In the calculation of the area, we wrapped the radius ** 2 within parentheses. Even though that wasn't necessary because the power operator has higher precedence than the multiplication one, we think the formula reads more easily like that. Moreover, should you get a slightly different result for the area, don't worry. It might depend on your OS, how Python was compiled, and so on. As long as the first few decimal digits are correct, you know it's a correct result. 

The sys.float_info sequence holds information about how floating point numbers will behave on your system. This is an example of what you might see:

```python
>>> import sys
>>> sys.float_info
sys.float_info(
    max=1.7976931348623157e+308, max_exp=1024, max_10_exp=308,
    min=2.2250738585072014e-308, min_exp=-1021, min_10_exp=-307,
    dig=15, mant_dig=53, epsilon=2.220446049250313e-16, radix=2,
    rounds=1
)
```

Let's make a few considerations here: we have 64 bits to represent floating point numbers. This means we can represent at most 264 (that is 18,446,744,073,709,551,616) distinct numbers. Take a look at the max and epsilon values for the float numbers, and you will realize that it's impossible to represent them all. There is just not enough space, so they are approximated to the closest representable number. You probably think that only extremely big or extremely small numbers suffer from this issue. Well, think again and try the following in your console:

```python
>>> 0.3 - 0.1 * 3  # this should be 0!!!
-5.551115123125783e-17
```

What does this tell you? It tells you that double precision numbers suffer from approximation issues even when it comes to simple numbers like 0.1 or 0.3. Why is this important? It can be a big problem if you are handling prices, or financial calculations, or any kind of data that need not to be approximated. Don't worry, Python gives you the Decimal type, which doesn't suffer from these issues; we'll see them in a moment.

### Complex numbers

Python gives you complex numbers support out of the box. If you don't know what complex numbers are, they are numbers that can be expressed in the form a + ib, where a and b are real numbers, and i (or j if you're an engineer) is the imaginary unit; that is, the square root of -1. a and b are called, respectively, the real and imaginary part of the number.

It is perhaps unlikely that you will use them, unless you're coding something scientific. Nevertheless, let's see a small example:

```python
>>> c = 3.14 + 2.73j
>>> c = complex(3.14, 2.73)  # same as above
>>> c.real  # real part
3.14
>>> c.imag  # imaginary part
2.73
>>> c.conjugate()  # conjugate of A + Bj is A - Bj
(3.14-2.73j)
>>> c * 2  # multiplication is allowed
(6.28+5.46j)
>>> c ** 2  # power operation as well
(2.4067000000000007+17.1444j)
>>> d = 1 + 1j  # addition and subtraction as well
>>> c - d
(2.14+1.73j)
```

### Fractions and decimals

Let's finish the tour of the number department with a look at fractions and decimals. Fractions hold a rational numerator and denominator in their lowest forms. Let's see a quick example:

```python
>>> from fractions import Fraction
>>> Fraction(10, 6)  # mad hatter?
Fraction(5, 3)  # notice it's been simplified
>>> Fraction(1, 3) + Fraction(2, 3)  # 1/3 + 2/3 == 3/3 == 1/1
Fraction(1, 1)
>>> f = Fraction(10, 6)
>>> f.numerator
5
>>> f.denominator
3
>>> f.as_integer_ratio()
(5, 3)
```

The as_integer_ratio() method has also been added to integers and Booleans. This is helpful, as it allows you to use it without needing to worry about what type of number is being worked with.

Although Fraction objects can be very useful at times, it's not that common to spot them in commercial software. Instead, it is much more common to see decimal numbers being used in all those contexts where precision is everything; for example, in scientific and financial calculations.

It's important to remember that arbitrary precision decimal numbers come at a price in terms of performance, of course. The amount of data to be stored for each number is greater than it is for Fractions or floats. The way they are handled also requires the Python interpreter to work harder behind the scenes. Another interesting thing to note is that you can get and set the precision by accessing decimal.getcontext().prec.

Let's see a quick example with decimal numbers:

```python
>>> from decimal import Decimal as D  # rename for brevity
>>> D(3.14)  # pi, from float, so approximation issues
Decimal('3.140000000000000124344978758017532527446746826171875')
>>> D('3.14')  # pi, from a string, so no approximation issues
Decimal('3.14')
>>> D(0.1) * D(3) - D(0.3)  # from float, we still have the issue
Decimal('2.775557561565156540423631668E-17')
>>> D('0.1') * D(3) - D('0.3')  # from string, all perfect
Decimal('0.0')
>>> D('1.4').as_integer_ratio()  # 7/5 = 1.4 (isn't this cool?!)
(7, 5)
```

Notice that when we construct a Decimal number from a float, it takes on all the approximation issues a float may come with. On the other hand, when we create a Decimal from an integer or a string representation of a number, then the Decimal will have no approximation issues, and therefore no quirky behavior. When it comes to currency or situations in which precision is of utmost importance, use decimals.

This concludes our introduction to built-in numeric types. Let's now look at sequences.

---
## Immutable sequences

Let's start with immutable sequences: strings, tuples, and bytes.

### Strings and bytes

Textual data in Python is handled with str objects, more commonly known as strings. They are immutable sequences of Unicode code points. Unicode code points can represent a character, but can also have other meanings, such as when formatting, for example. Python, unlike other languages, doesn't have a char type, so a single character is rendered simply by a string of length 1.

Unicode is an excellent way to handle data, and should be used for the internals of any application. When it comes to storing textual data though, or sending it on the network, you will likely want to encode it, using an appropriate encoding for the medium you are using. The result of an encoding produces a bytes object, whose syntax and behavior is similar to that of strings. String literals are written in Python using single, double, or triple quotes (both single or double). If built with triple quotes, a string can span multiple lines. An example will clarify this:

```python
>>> # 4 ways to make a string
>>> str1 = 'This is a string. We built it with single quotes.'
>>> str2 = "This is also a string, but built with double quotes."
>>> str3 = '''This is built using triple quotes,
... so it can span multiple lines.'''
>>> str4 = """This too
... is a multiline one
... built with triple double-quotes."""
>>> str4  #A
'This too\nis a multiline one\nbuilt with triple double-quotes.'
>>> print(str4)  #B
This too
is a multiline one
built with triple double-quotes.
```

In #A and #B, we print str4, first implicitly, and then explicitly, using the print() function. A good exercise would be to find out why they are different. Are you up to the challenge? (Hint: look up the str() and repr() functions.)

Strings, like any sequence, have a length. You can get this by calling the len() function:

```python
>>> len(str1)
49
```

Python 3.9 has introduced two new methods that deal with the prefixes and suffixes of strings. Here's an example that explains the way they work:

```python
>>> s = 'Hello There'
>>> s.removeprefix('Hell')
'o There'
>>> s.removesuffix('here')
'Hello T'
>>> s.removeprefix('Ooops')
'Hello There'
```

The nice thing about them is shown by the last instruction: when we attempt to remove a prefix or suffix which is not there, the method simply returns a copy of the original string. This means that these methods, behind the scenes, are checking if the prefix or suffix matches the argument of the call, and when that's the case, they remove it.


#### Encoding and decoding strings

Using the encode/decode methods, we can encode Unicode strings and decode bytes objects. UTF-8 is a variable-length character encoding, capable of encoding all possible Unicode code points. It is the most widely used encoding for the web. Notice also that by adding the literal b in front of a string declaration, we're creating a bytes object:

```python
>>> s = "This is üŋíc0de"  # unicode string: code points
>>> type(s)
<class 'str'>
>>> encoded_s = s.encode('utf-8')  # utf-8 encoded version of s
>>> encoded_s
b'This is \xc3\xbc\xc5\x8b\xc3\xadc0de'  # result: bytes object
>>> type(encoded_s)  # another way to verify it
<class 'bytes'>
>>> encoded_s.decode('utf-8')  # let's revert to the original
'This is üŋíc0de'
>>> bytes_obj = b"A bytes object"  # a bytes object
>>> type(bytes_obj)
<class 'bytes'>
```

#### Indexing and slicing strings

When manipulating sequences, it's very common to access them at one precise position (indexing), or to get a sub-sequence out of them (slicing). When dealing with immutable sequences, both operations are read-only.

While indexing comes in one form—zero-based access to any position within the sequence—slicing comes in different forms. When you get a slice of a sequence, you can specify the start and stop positions, along with the step. They are separated with a colon (:) like this: my_sequence[start:stop:step]. All the arguments are optional; start is inclusive, and stop is exclusive. It's probably better to see an example, rather than try to explain them any further with words:

```python
>>> s = "The trouble is you think you have time."
>>> s[0]  # indexing at position 0, which is the first char
'T'
>>> s[5]  # indexing at position 5, which is the sixth char
'r'
>>> s[:4]  # slicing, we specify only the stop position
'The '
>>> s[4:]  # slicing, we specify only the start position
'trouble is you think you have time.'
>>> s[2:14]  # slicing, both start and stop positions
'e trouble is'
>>> s[2:14:3]  # slicing, start, stop and step (every 3 chars)
'erb '
>>> s[:]  # quick way of making a copy
'The trouble is you think you have time.'
```

The last line is quite interesting. If you don't specify any of the parameters, Python will fill in the defaults for you. In this case, start will be the start of the string, stop will be the end of the string, and step will be the default: 1. This is an easy and quick way of obtaining a copy of the string s (the same value, but a different object). Can you think of a way to get the reversed copy of a string using slicing (don't look it up—find it for yourself)?


#### String formatting

One of the features strings have is the ability to be used as a template. There are several different ways of formatting a string, and for the full list of possibilities, we encourage you to look up the documentation. Here are some common examples:

```python
>>> greet_old = 'Hello %s!'
>>> greet_old % 'Fabrizio'
'Hello Fabrizio!'
>>> greet_positional = 'Hello {}!'
>>> greet_positional.format('Fabrizio')
'Hello Fabrizio!'
>>> greet_positional = 'Hello {} {}!'
>>> greet_positional.format('Fabrizio', 'Romano')
'Hello Fabrizio Romano!'
>>> greet_positional_idx = 'This is {0}! {1} loves {0}!'
>>> greet_positional_idx.format('Python', 'Heinrich')
'This is Python! Heinrich loves Python!'
>>> greet_positional_idx.format('Coffee', 'Fab')
'This is Coffee! Fab loves Coffee!'
>>> keyword = 'Hello, my name is {name} {last_name}'
>>> keyword.format(name='Fabrizio', last_name='Romano')
'Hello, my name is Fabrizio Romano'
```

In the previous example, you can see four different ways of formatting strings. The first one, which relies on the % operator, is deprecated and shouldn't be used anymore. The current, modern way to format a string is by using the format() string method. You can see, from the different examples, that a pair of curly braces acts as a placeholder within the string. When we call format(), we feed it data that replaces the placeholders. We can specify indexes (and much more) within the curly braces, and even names, which implies we'll have to call format() using keyword arguments instead of positional ones.

Notice how greet_positional_idx is rendered differently by feeding different data to the call to format.

One last feature we want to show you was added to Python in version 3.6, and it's called formatted string literals. This feature is quite cool (and it is faster than using the format() method): strings are prefixed with f, and contain replacement fields surrounded by curly braces. 

Replacement fields are expressions evaluated at runtime, and then formatted using the format protocol:

```python
>>> name = 'Fab'
>>> age = 42
>>> f"Hello! My name is {name} and I'm {age}"
"Hello! My name is Fab and I'm 42"
>>> from math import pi
>>> f"No arguing with {pi}, it's irrational..."
"No arguing with 3.141592653589793, it's irrational..."
```

An interesting addition to f-strings, which was introduced in Python 3.8, is the ability to add an equals sign specifier within the f-string clause; this causes the expression to expand to the text of the expression, an equals sign, then the representation of the evaluated expression. This is great for self-documenting and debugging purposes. Here's an example that shows the difference in behavior:

```python
>>> user = 'heinrich'
>>> password = 'super-secret'
>>> f"Log in with: {user} and {password}"
'Log in with: heinrich and super-secret'
>>> f"Log in with: {user=} and {password=}"
"Log in with: user='heinrich' and password='super-secret'"
```

Check out the official documentation to learn everything about string formatting and how truly powerful it can be.


### Tuples

The last immutable sequence type we are going to look at here is the tuple. A tuple is a sequence of arbitrary Python objects. In a tuple declaration, items are separated by commas. Tuples are used everywhere in Python. They allow for patterns that are quite hard to reproduce in other languages. Sometimes tuples are used implicitly; for example, to set up multiple variables on one line, or to allow a function to return multiple objects (in several languages, it is common for a function to return only one object), and in the Python console, tuples can be used implicitly to print multiple elements with one single instruction. We'll see examples for all these cases:

```python
>>> t = ()  # empty tuple
>>> type(t)
<class 'tuple'>
>>> one_element_tuple = (42, )  # you need the comma!
>>> three_elements_tuple = (1, 3, 5)  # braces are optional here
>>> a, b, c = 1, 2, 3  # tuple for multiple assignment
>>> a, b, c  # implicit tuple to print with one instruction
(1, 2, 3)
>>> 3 in three_elements_tuple  # membership test
True
```

Notice that the membership operator in can also be used with lists, strings, dictionaries, and, in general, with collection and sequence objects.

Notice that to create a tuple with one item, we need to put a comma after the item. The reason is that without the comma that item is wrapped in braces on its own, in what can be considered a redundant mathematical expression. Notice also that on assignment, braces are optional, so my_tuple = 1, 2, 3 is the same as my_tuple = (1, 2, 3).

One thing that tuple assignment allows us to do is one-line swaps, with no need for a third temporary variable. Let's first see the traditional way of doing it:


```python
>>> a, b = 1, 2
>>> c = a  # we need three lines and a temporary var c
>>> a = b
>>> b = c
>>> a, b  # a and b have been swapped
(2, 1)
Now let's see how we would do it in Python:
>>> a, b = 0, 1
>>> a, b = b, a  # this is the Pythonic way to do it
>>> a, b
(1, 0)
```

Take a look at the line that shows you the Pythonic way of swapping two values. Do you remember what we wrote in Chapter 1, A Gentle Introduction to Python? A Python program is typically one-fifth to one-third the size of equivalent Java or C++ code, and features like one-line swaps contribute to this. Python is elegant, where elegance in this context also means economy.

Because they are immutable, tuples can be used as keys for dictionaries (we'll see this shortly). To us, tuples are Python's built-in data that most closely represent a mathematical vector. This doesn't mean that this was the reason for which they were created, though. Tuples usually contain a heterogeneous sequence of elements while, on the other hand, lists are, most of the time, homogeneous. Moreover, tuples are normally accessed via unpacking or indexing, while lists are usually iterated over.

---
## Mutable sequences

Mutable sequences differ from their immutable counterparts in that they can be changed after creation. There are two mutable sequence types in Python: lists and byte arrays.

### Lists

Python lists are very similar to tuples, but they don't have the restrictions of immutability. Lists are commonly used for storing collections of homogeneous objects, but there is nothing preventing you from storing heterogeneous collections as well. Lists can be created in many different ways. Let's see an example:

```python
>>> []  # empty list
[]
>>> list()  # same as []
[]
>>> [1, 2, 3]  # as with tuples, items are comma separated
[1, 2, 3]
>>> [x + 5 for x in [2, 3, 4]]  # Python is magic
[7, 8, 9]
>>> list((1, 3, 5, 7, 9))  # list from a tuple
[1, 3, 5, 7, 9]
>>> list('hello')  # list from a string
['h', 'e', 'l', 'l', 'o']
```

In the previous example, we showed you how to create a list using various techniques. We would like you to take a good look at the line with the comment Python is magic, which we don't expect you to fully understand at this point— especially if you are unfamiliar with Python. That is called a list comprehension: a very powerful functional feature of Python, which we will see in detail in Chapter 5, Comprehensions and Generators. We just wanted to spark your curiosity at this point.

Creating lists is good, but the real fun begins when we use them, so let's see the main methods they gift us with:

```python
>>> a = [1, 2, 1, 3]
>>> a.append(13)  # we can append anything at the end
>>> a
[1, 2, 1, 3, 13]
>>> a.count(1)  # how many `1s` are there in the list?
2
>>> a.extend([5, 7])  # extend the list by another (or sequence)
>>> a
[1, 2, 1, 3, 13, 5, 7]
>>> a.index(13)  # position of `13` in the list (0-based indexing)
4
>>> a.insert(0, 17)  # insert `17` at position 0
>>> a
[17, 1, 2, 1, 3, 13, 5, 7]
>>> a.pop()  # pop (remove and return) last element
7
>>> a.pop(3)  # pop element at position 3
1
>>> a
[17, 1, 2, 3, 13, 5]
>>> a.remove(17)  # remove `17` from the list
>>> a
[1, 2, 3, 13, 5]
>>> a.reverse()  # reverse the order of the elements in the list
>>> a
[5, 13, 3, 2, 1]
>>> a.sort()  # sort the list
>>> a
[1, 2, 3, 5, 13]
>>> a.clear()  # remove all elements from the list
>>> a
[]
```

The preceding code gives you a roundup of a list's main methods. We want to show you how powerful they are, using the method extend() as an example. You can extend lists using any sequence type:

```python
>>> a = list('hello')  # makes a list from a string
>>> a
['h', 'e', 'l', 'l', 'o']
>>> a.append(100)  # append 100, heterogeneous type
>>> a
['h', 'e', 'l', 'l', 'o', 100]
>>> a.extend((1, 2, 3))  # extend using tuple
>>> a
['h', 'e', 'l', 'l', 'o', 100, 1, 2, 3]
>>> a.extend('...')  # extend using string
>>> a
['h', 'e', 'l', 'l', 'o', 100, 1, 2, 3, '.', '.', '.']
```

Now, let's see the most common operations you can do with lists:
```python
>>> a = [1, 3, 5, 7]
>>> min(a)  # minimum value in the list
1
>>> max(a)  # maximum value in the list
7
>>> sum(a)  # sum of all values in the list
16
>>> from math import prod
>>> prod(a)  # product of all values in the list
105
>>> len(a)  # number of elements in the list
4
>>> b = [6, 7, 8]
>>> a + b  # `+` with list means concatenation
[1, 3, 5, 7, 6, 7, 8]
>>> a * 2  # `*` has also a special meaning
[1, 3, 5, 7, 1, 3, 5, 7]
```

Notice how easily we can perform the sum and the product of all values in a list. The function prod(), from the math module, is just one of the many new additions introduced in Python 3.8. Even if you don't plan to use it that often, it's always a good idea to check out the math module and be familiar with its functions, as they can be quite helpful.

The last two lines in the preceding code are also quite interesting, as they introduce us to a concept called operator overloading. In short, this means that operators, such as +, -, *, %, and so on, may represent different operations according to the context they are used in. It doesn't make any sense to sum two lists, right? Therefore, the + sign is used to concatenate them. Hence, the * sign is used to concatenate the list to itself according to the right operand.

Now, let's take a step further and see something a little more interesting. We want to show you how powerful the sorted method can be and how easy it is in Python to achieve results that require a great deal of effort in other languages:

```python
>>> from operator import itemgetter
>>> a = [(5, 3), (1, 3), (1, 2), (2, -1), (4, 9)]
>>> sorted(a)
[(1, 2), (1, 3), (2, -1), (4, 9), (5, 3)]
>>> sorted(a, key=itemgetter(0))
[(1, 3), (1, 2), (2, -1), (4, 9), (5, 3)]
>>> sorted(a, key=itemgetter(0, 1))
[(1, 2), (1, 3), (2, -1), (4, 9), (5, 3)]
>>> sorted(a, key=itemgetter(1))
[(2, -1), (1, 2), (5, 3), (1, 3), (4, 9)]
>>> sorted(a, key=itemgetter(1), reverse=True)
[(4, 9), (5, 3), (1, 3), (1, 2), (2, -1)]
```

The preceding code deserves a little explanation. First of all, a is a list of tuples. This means each element in a is a tuple (a 2-tuple in this case). When we call sorted(my_ list), we get a sorted version of my_list. In this case, the sorting on a 2-tuple works by sorting them on the first item in the tuple, and on the second when the first one is the same. You can see this behavior in the result of sorted(a), which yields [(1, 2), (1, 3), ...]. Python also gives us the ability to control which element(s) of the tuple the sorting must be run against. Notice that when we instruct the sorted function, to work on the first element of each tuple (with key=itemgetter(0)), the result is different: [(1, 3), (1, 2), ...]. The sorting is done only on the first element of each tuple (which is the one at position 0). If we want to replicate the default behavior of a simple sorted(a) call, we need to use key=itemgetter(0, 1), which tells Python to sort first on the elements at position 0 within the tuples, and then on those at position 1. Compare the results and you will see that they match.

For completeness, we included an example of sorting only on the elements at position 1, and then again, with the same sorting but in reverse order. If you have ever seen sorting in other languages, you should be quite impressed at this moment.

The Python sorting algorithm is very powerful, and it was written by Tim Peters (we've already seen this name, can you recall when?). It is aptly named Timsort, and it is a blend between merge and insertion sort and has better time performances than most other algorithms used for mainstream programming languages. Timsort is a stable sorting algorithm, which means that when multiple records score the same in the comparison, their original order is preserved. We've seen this in the result of sorted(a, key=itemgetter(0)), which yielded [(1, 3), (1, 2), ...], in which the order of those two tuples had been preserved because they had the same value at position 0.

### Bytearrays

To conclude our overview of mutable sequence types, let's spend a moment on the bytearray type. Basically, they represent the mutable version of bytes objects. They expose most of the usual methods of mutable sequences as well as most of the methods of the bytes type. Items in a bytearray are integers in the range [0, 256).

When it comes to intervals, we are going to use the standard notation for open/closed ranges. A square bracket on one end means that the value is included, while a round bracket means that it is excluded. The granularity is usually inferred by the type of the edge elements so, for example, the interval [3, 7] means all integers between 3 and 7, inclusive. On the other hand, (3, 7) means all integers between 3 and 7, exclusive (4, 5, and 6). Items in a bytearray type are integers between 0 and 256; 0 is included, 256 is not. One reason that intervals are often expressed like this is to ease coding. If we break a range [a, b) into N consecutive ranges, we can easily represent the original one as a concatenation like this:

    [a,k1) + [k1,k2) + [k2,k3) + ... + [kN-1,b)

The middle points (ki) being excluded on one end, and included on the other end, allow for easy concatenation and splitting when intervals are handled in the code.

Let's see an example with the bytearray type:

```python
>>> bytearray()  # empty bytearray object
bytearray(b'')
>>> bytearray(10)  # zero-filled instance with given length
bytearray(b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00')
>>> bytearray(range(5))  # bytearray from iterable of integers
bytearray(b'\x00\x01\x02\x03\x04')
>>> name = bytearray(b'Lina')  #A - bytearray from bytes
>>> name.replace(b'L', b'l')
bytearray(b'lina')
>>> name.endswith(b'na')
True
>>> name.upper()
bytearray(b'LINA')
>>> name.count(b'L')
1
```

As you can see, there are a few ways to create a bytearray object. They can be useful in many situations; for example, when receiving data through a socket, they eliminate the need to concatenate data while polling, hence they can prove to be very handy. On line #A, we created a bytearray named as name from the bytes literal b'Lina' to show you how the bytearray object exposes methods from both sequences and strings, which is extremely handy. If you think about it, they can be considered as mutable strings.

### Set types

Python also provides two set types, set and frozenset. The set type is mutable, while frozenset is immutable. They are unordered collections of immutable objects. Hashability is a characteristic that allows an object to be used as a set member as well as a key for a dictionary, as we'll see very soon.

From the official documentation (https://docs.python. org/3.9/glossary.html): "An object is hashable if it has a hash value which never changes during its lifetime, and can be compared to other objects. […] Hashability makes an object usable as a dictionary key and a set member, because these data structures use the hash value internally. Most of Python's immutable built- in objects are hashable; mutable containers (such as lists or dictionaries) are not; immutable containers (such as tuples and frozensets) are only hashable if their elements are hashable. Objects which are instances of user-defined classes are hashable by default. They all compare unequal (except with themselves), and their hash value is derived from their id()."

Objects that compare equally must have the same hash value. Sets are very commonly used to test for membership; let's introduce the in operator in the following example:

```python
>>> small_primes = set()  # empty set
>>> small_primes.add(2)  # adding one element at a time
>>> small_primes.add(3)
>>> small_primes.add(5)
>>> small_primes
{2, 3, 5}
>>> small_primes.add(1)  # Look what I've done, 1 is not a prime!
>>> small_primes
{1, 2, 3, 5}
>>> small_primes.remove(1)  # so let's remove it
>>> 3 in small_primes  # membership test
True
>>> 4 in small_primes
False
>>> 4 not in small_primes  # negated membership test
True
>>> small_primes.add(3)  # trying to add 3 again
>>> small_primes
{2, 3, 5}  # no change, duplication is not allowed
>>> bigger_primes = set([5, 7, 11, 13])  # faster creation
>>> small_primes | bigger_primes  # union operator `|`
{2, 3, 5, 7, 11, 13}
>>> small_primes & bigger_primes  # intersection operator `&`
{5}
>>> small_primes - bigger_primes  # difference operator `-`
{2, 3}
```

In the preceding code, you can see two different ways to create a set. One creates an empty set and then adds elements one at a time. The other creates the set using a list of numbers as an argument to the constructor, which does all the work for us. Of course, you can create a set from a list or tuple (or any iterable) and then you can add and remove members from the set as you please.

We'll look at iterable objects and iteration in the next chapter. For now, just know that iterable objects are objects you can iterate on in a direction.

Another way of creating a set is by simply using the curly braces notation, like this:

```python
>>> small_primes = {2, 3, 5, 5, 3}
>>> small_primes
{2, 3, 5}
```

Notice we added some duplication to emphasize that the resulting set won't have any. Let's see an example using the immutable counterpart of the set type, frozenset:

```python
>>> small_primes = frozenset([2, 3, 5, 7])
>>> bigger_primes = frozenset([5, 7, 11])
>>> small_primes.add(11)  # we cannot add to a frozenset
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: 'frozenset' object has no attribute 'add'
>>> small_primes.remove(2)  # nor can we remove
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
AttributeError: 'frozenset' object has no attribute 'remove'
>>> small_primes & bigger_primes  # intersect, union, etc. allowed
frozenset({5, 7})
```

As you can see, frozenset objects are quite limited with respect to their mutable counterpart. They still prove very effective for membership test, union, intersection, and difference operations, and for performance reasons.

### Mapping types: dictionaries

Of all the built-in Python data types, the dictionary is easily the most interesting. It's the only standard mapping type, and it is the backbone of every Python object.

A dictionary maps keys to values. Keys need to be hashable (in Python equal to immutable types) objects, while values can be of any arbitrary type. Dictionaries are also mutable objects. There are quite a few different ways to create a dictionary, so let us give you a simple example of how to create a dictionary equal to {'A': 1, 'Z': -1} in five different ways:

```python
>>> a = dict(A=1, Z=-1)
>>> b = {'A': 1, 'Z': -1}
>>> c = dict(zip(['A', 'Z'], [1, -1]))
>>> d = dict([('A', 1), ('Z', -1)])
>>> e = dict({'Z': -1, 'A': 1})
>>> a == b == c == d == e  # are they all the same?
True  # They are indeed
```

Have you noticed those double equals? Assignment is done with one equal, while to check whether an object is the same as another one (or five in one go, in this case), we use double equals. There is also another way to compare objects, which involves the is operator, and checks whether the two objects are the same (that is, that they have the same ID, not just the same value), but unless you have a good reason to use it, you should use the double equals instead. In the preceding code, we also used one nice function: zip(). It is named after the real-life zip, which glues together two parts, taking one element from each part at a time. Let us show you an example:

```python
>>> list(zip(['h', 'e', 'l', 'l', 'o'], [1, 2, 3, 4, 5]))
[('h', 1), ('e', 2), ('l', 3), ('l', 4), ('o', 5)]
>>> list(zip('hello', range(1, 6)))  # equivalent, more pythonic
[('h', 1), ('e', 2), ('l', 3), ('l', 4), ('o', 5)] 
```

In the preceding example, we have created the same list in two different ways, one more explicit, and the other a little bit more Pythonic. Forget for a moment that we had to wrap the list() constructor around the zip() call (the reason is zip() returns an iterator, not a list, so if we want to see the result, we need to exhaust that iterator into something—a list in this case), and concentrate on the result. See how zip() has coupled the first elements of its two arguments together, then the second ones, then the third ones, and so on? 

Take a look at the zip of your suitcase, or a purse, or the cover of a pillow, and you will see it works exactly like the one in Python. But let's go back to dictionaries and see how many wonderful methods they expose for allowing us to manipulate them as we want. Let's start with the basic operations:

```python
>>> d = {}
>>> d['a'] = 1  # let's set a couple of (key, value) pairs
>>> d['b'] = 2
>>> len(d)  # how many pairs?
2
>>> d['a']  # what is the value of 'a'?
1
>>> d  # how does `d` look now?
{'a': 1, 'b': 2}
>>> del d['a']  # let's remove `a`
>>> d
{'b': 2}
>>> d['c'] = 3  # let's add 'c': 3
>>> 'c' in d  # membership is checked against the keys
True
>>> 3 in d  # not the values
False
>>> 'e' in d
False
>>> d.clear()  # let's clean everything from this dictionary
>>> d
{}
```

Notice how accessing keys of a dictionary, regardless of the type of operation we're performing, is done using square brackets. Do you remember strings, lists, and tuples? We were accessing elements at some position through square brackets as well, which is yet another example of Python's consistency.

Let's now look at three special objects called dictionary views: keys, values, and items. These objects provide a dynamic view of the dictionary entries and they change when the dictionary changes. keys() returns all the keys in the dictionary, values() returns all the values in the dictionary, and items() returns all the (key, value) pairs in the dictionary.

Enough with this chatter; let's put all this down into code:

```python
>>> d = dict(zip('hello', range(5)))
>>> d
{'h': 0, 'e': 1, 'l': 3, 'o': 4}
>>> d.keys()
dict_keys(['h', 'e', 'l', 'o'])
>>> d.values()
dict_values([0, 1, 3, 4])
>>> d.items()
dict_items([('h', 0), ('e', 1), ('l', 3), ('o', 4)])
>>> 3 in d.values()
True
>>> ('o', 4) in d.items()
True
```

There are a few things to note here. First, notice how we are creating a dictionary by iterating over the zipped version of the string 'hello' and the list [0, 1, 2, 3, 4]. The string 'hello' has two 'l' characters inside, and they are paired up with the values 2 and 3 by the zip() function. Notice how in the dictionary, the second occurrence of the 'l' key (the one with the value 3), overwrites the first one (the one with the value 2). Another thing to notice is that when asking for any view, the original order in which items were added is now preserved, while before version 3.6 there was no guarantee of that.

As of Python 3.6, the dict type has been reimplemented to use a more compact representation. This resulted in dictionaries using 20% to 25% less memory when compared to Python 3.5. Moreover, since Python 3.6, as a side effect, dictionaries preserve the order in which keys were inserted. This feature has received such a welcome from the community that in 3.7 it has become an official feature of the language rather than an implementation side effect. Since Python 3.8, dictionaries are also reversible!

We'll see how these views are fundamental tools when we talk about iterating over collections. Let's take a look now at some other methods exposed by Python's dictionaries—there's plenty of them and they are very useful:

```python
>>> d
{'h': 0, 'e': 1, 'l': 3, 'o': 4}
>>> d.popitem()  # removes a random item (useful in algorithms)
('o', 4)
>>> d
{'h': 0, 'e': 1, 'l': 3}
>>> d.pop('l')  # remove item with key `l`
3
>>> d.pop('not-a-key')  # remove a key not in dictionary: KeyError
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
KeyError: 'not-a-key'
>>> d.pop('not-a-key', 'default-value')  # with a default value?
'default-value'  # we get the default value
>>> d.update({'another': 'value'})  # we can update dict this way
>>> d.update(a=13)  # or this way (like a function call)
>>> d
{'h': 0, 'e': 1, 'another': 'value', 'a': 13}
>>> d.get('a')  # same as d['a'] but if key is missing no KeyError
13
>>> d.get('a', 177)  # default value used if key is missing
13
>>> d.get('b', 177)  # like in this case
177
>>> d.get('b')  # key is not there, so None is returned
```

All these methods are quite simple to understand, but it's worth talking about that None, for a moment. Every function in Python returns None, unless the return statement is explicitly used to return something else, but we'll see this when we explore functions. None is frequently used to represent the absence of a value, and it is quite commonly used as a default value for arguments in function declaration. Some inexperienced coders sometimes write code that returns either False or None. Both False and None evaluate to False in a Boolean context, so it may seem that there is not much difference between them. But actually, we would argue the contrary, that there is an important difference: False means that we have information, and the information we have is False. None means no information; no information is very different from information that is False. In layman's terms, if you ask your mechanic Is my car ready?, there is a big difference between the answer No, it's not (False) and I have no idea (None).

One last method we really like about dictionaries is setdefault(). It behaves like get(), but also sets the key with the given value if it is not there. Let's see an example:

```python
>>> d = {}
>>> d.setdefault('a', 1)  # 'a' is missing, we get default value
1
>>> d
{'a': 1}  # also, the key/value pair ('a', 1) has now been added
>>> d.setdefault('a', 5)  # let's try to override the value
1
>>> d
{'a': 1}  # no override, as expected
```

This brings us to the end of this tour of dictionaries. Test your knowledge about them by trying to foresee what d looks like after this line:

```python
>>> d = {}
>>> d.setdefault('a', {}).setdefault('b', []).append(1)
```

Don't worry if you don't get it immediately. We just want to encourage you to experiment with dictionaries.

Python 3.9 sports a brand-new union operator available for dict objects, which was introduced by PEP 584. When it comes to applying union to dict objects, we need to remember that union for them is not commutative. This becomes evident when the two dict objects we're merging have one or more keys in common. Check out this example:

```python
>>> d = {'a': 'A', 'b': 'B'}
>>> e = {'b': 8, 'c': 'C'}
>>> d | e
{'a': 'A', 'b': 8, 'c': 'C'}
>>> e | d
{'b': 'B', 'c': 'C', 'a': 'A'}
>>> {**d, **e}
{'a': 'A', 'b': 8, 'c': 'C'}
>>> {**e, **d}
{'b': 'B', 'c': 'C', 'a': 'A'}
>>> d |= e
>>> d
{'a': 'A', 'b': 8, 'c': 'C'}

```

Here, dict objects d and e have the key 'b' in common. For the dict object, d, the value associated with 'b' is 'B'; whereas, for dict e, it's the number 8. This means that when we merge them with e on the righthand side of the union operator, |, the value in e overrides the one in d. The opposite happens, of course, when we swap the positions of those objects in relation to the union operator.

In this example, you can also see how the union can be performed by using the ** operator to produce a dictionary unpacking. It's worth noting that union can also be performed as an augmented assignment operation (d |= e), which works in place. Please refer to PEP 584 for more information about this feature.

This concludes our tour of built-in data types. Before we discuss some considerations about what we've seen in this chapter, we briefly want to take a peek at data types.

---
## Data types

Python provides a variety of specialized data types, such as dates and times, container types, and enumerations. There is a whole section in the Python standard library titled Data Types, which deserves to be explored; it is filled with interesting and useful tools for each and every programmer's needs. You can find it here:

https://docs.python.org/3/library/datatypes.html

In this section, we are briefly going to take a look at dates and times, collections, and enumerations. Dates and times The Python standard library provides several data types that can be used to deal with dates and times. This realm may seem innocuous at first glance, but it's actually quite tricky: timezones, daylight saving time… There are a huge number of ways to format date and time information; calendar quirks, parsing, and localizing—these are just a few of the many difficulties we face when we deal with dates and times, and that's probably the reason why, in this particular context, it is very common for professional Python programmers to also rely on various third-party libraries that provide some much-needed extra power.

### The standard library

We will start with the standard library, and finish the session with a little overview of what's out there in terms of the third-party libraries you can use. From the standard library, the main modules that are used to handle dates and times are datetime, calendar, zoneinfo, and time. Let's start with the imports you'll need for this whole section:

```python
>>> from datetime import date, datetime, timedelta, timezone
>>> import time
>>> import calendar as cal
>>> from zoneinfo import ZoneInfo
```

The first example deals with dates. Let's see how they look:

```python
>>> today = date.today()
>>> today
datetime.date(2021, 3, 28)
>>> today.ctime()
'Sun Mar 28 00:00:00 2021'
>>> today.isoformat()
'2021-03-28'
>>> today.weekday()
6
>>> cal.day_name[today.weekday()]
'Sunday'
>>> today.day, today.month, today.year
(28, 3, 2021)
>>> today.timetuple()
time.struct_time(
    tm_year=2021, tm_mon=3, tm_mday=28,
    tm_hour=0, tm_min=0, tm_sec=0,
    tm_wday=6, tm_yday=87, tm_isdst=-1
)
```

We start by fetching the date for today. We can see that it's an instance of the datetime.date class. Then we get two different representations for it, following the C and the ISO 8601 format standards, respectively. After that, we ask what day of the week it is, and we get the number 6. Days are numbered 0 to 6 (representing Monday to Sunday), so we grab the value of the sixth element in calendar.day_name (notice in the code that we have substituted calendar with "cal" for brevity).

The last two instructions show how to get detailed information out of a date object. We can inspect its day, month, and year attributes, or call the timetuple() method and get a whole wealth of information. Since we're dealing with a date object, notice that all the information about time has been set to 0. 

Let's now play with time:

```python
>>> time.ctime()
'Sun Mar 28 15:23:17 2021'
>>> time.daylight
1
>>> time.gmtime()
time.struct_time(
    tm_year=2021, tm_mon=3, tm_mday=28,
    tm_hour=14, tm_min=23, tm_sec=34,
    tm_wday=6, tm_yday=87, tm_isdst=0
)
>>> time.gmtime(0)
time.struct_time(
    tm_year=1970, tm_mon=1, tm_mday=1,
    tm_hour=0, tm_min=0, tm_sec=0,
    tm_wday=3, tm_yday=1, tm_isdst=0
)
>>> time.localtime()
time.struct_time(
    tm_year=2021, tm_mon=3, tm_mday=28,
    tm_hour=15, tm_min=23, tm_sec=50,
    tm_wday=6, tm_yday=87, tm_isdst=1
)
>>> time.time()
1616941458.149149
```

This example is quite similar to the one before, only here, we are dealing with time. We can see how to get a printed representation of time according to C format standard, and then how to check if daylight saving time is in effect. The function gmtime converts a given number of seconds from the epoch to a struct_time object in UTC. If we don't feed it any number, it will use the current time.

The epoch is a date and time from which a computer system measures system time. You can see that on the machine used to run this code, the epoch is January 1st, 1970. This is the point in time used by both Unix and POSIX.

Coordinated Universal Time or UTC is the primary time standard by which the world regulates clocks and time.

We finish the example by getting the struct_time object for the current local time and the number of seconds from the epoch expressed as a float number (time.time()).

Let's now see an example using datetime objects, which bring together dates and times.

```python
>>> now = datetime.now()
>>> utcnow = datetime.utcnow()
>>> now
datetime.datetime(2021, 3, 28, 15, 25, 16, 258274)
>>> utcnow
datetime.datetime(2021, 3, 28, 14, 25, 22, 918195)
>>> now.date()
datetime.date(2021, 3, 28)
>>> now.day, now.month, now.year
(28, 3, 2021)
>>> now.date() == date.today()
True
>>> now.time()
datetime.time(15, 25, 16, 258274)
>>> now.hour, now.minute, now.second, now.microsecond
(15, 25, 16, 258274)
>>> now.ctime()
'Sun Mar 28 15:25:16 2021'
>>> now.isoformat()
'2021-03-28T15:25:16.258274'
>>> now.timetuple()
time.struct_time(
    tm_year=2021, tm_mon=3, tm_mday=28,
    tm_hour=15, tm_min=25, tm_sec=16,
    tm_wday=6, tm_yday=87, tm_isdst=-1
)
>>> now.tzinfo
>>> utcnow.tzinfo
>>> now.weekday()
6
```

The preceding example is rather self-explanatory. We start by setting up two instances that represent the current time. One is related to UTC (utcnow), and the other one is a local representation (now). It just so happens that we ran this code on the first day after daylight saving time was introduced in the UK in 2021, so now represents the current time in BST. BST is one hour ahead of UTC when daylight saving time is in effect, as can be seen from the code.

You can get date, time, and specific attributes from a datetime object in a similar way as to what we have already seen. It is also worth noting how both now and utcnow present the value None for the tzinfo attribute. This happens because those objects are naive.

Date and time objects may be categorized as aware if they include time zone information, or naïve if they don't.

Let's now see how a duration is represented in this context:

```python
>>> f_bday = datetime(
    1975, 12, 29, 12, 50, tzinfo=ZoneInfo('Europe/Rome')
    )
>>> h_bday = datetime(
    1981, 10, 7, 15, 30, 50, tzinfo=timezone(timedelta(hours=2))
    )
>>> diff = h_bday - f_bday
>>> type(diff)
<class 'datetime.timedelta'>
>>> diff.days
2109
>>> diff.total_seconds()
182223650.0
>>> today + timedelta(days=49)
datetime.date(2021, 5, 16)
>>> now + timedelta(weeks=7)
datetime.datetime(2021, 5, 16, 15, 25, 16, 258274)
```

Two objects have been created that represent Fabrizio and Heinrich's birthdays. This time, in order to show you the alternative, we have created aware objects. There are several ways to include time zone information when creating a datetime object, and in this example, we are showing you two of them. One uses the brand- new ZoneInfo object from the zoneinfo module, introduced in Python 3.9. The second one uses a simple timedelta, an object that represents a duration.

We then create the diff object, which is assigned as the subtraction of them. The result of that operation is an instance of timedelta. You can see how we can interrogate the diff object to tell us how many days Fabrizio and Heinrich's birthdays are apart, and even the number of seconds that represent that whole duration. Notice that we need to use total_seconds, which expresses the whole duration in seconds. The seconds attribute represents the number of seconds assigned to that duration. So, a timedelta(days=1) will have seconds equal to 0, and total_seconds equal to 86,400 (which is the number of seconds in a day).

Combining a datetime with a duration adds or subtracts that duration from the original date and time information. In the last few lines of the example, we can see how adding a duration to a date object produces a date as a result, whereas adding it to a datetime produces a datetime, as it is fair to expect.

One of the more difficult undertakings to carry out using dates and times is parsing. Let's see a short example:

```python
>>> datetime.fromisoformat('1977-11-24T19:30:13+01:00')
datetime.datetime(
    1977, 11, 24, 19, 30, 13,
    tzinfo=datetime.timezone(datetime.timedelta(seconds=3600))
)
>>> datetime.fromtimestamp(time.time())
datetime.datetime(2021, 3, 28, 15, 42, 2, 142696)
```

We can easily create datetime objects from ISO-formatted strings, as well as from timestamps. However, in general, parsing a date from unknown formats can prove to be a difficult task.

### Third-party libraries

To finish off this subsection, we would like to mention a few third-party libraries that you will very likely come across the moment you will have to deal with dates and times in your code:

- dateutil: Powerful extensions to datetime (https://dateutil.readthedocs.io/en/stable/)
- Arrow: Better dates and times for Python (https://arrow.readthedocs.io/en/latest/)
- pytz: World time zone definitions for Python (https://pythonhosted.org/pytz/)

These three are some of the most common, and they are worth investigating.

Let's take a look at one final example, this time using the Arrow third-party library:
 
```python
>>> import arrow
>>> arrow.utcnow()
<Arrow [2021-03-28T14:43:20.017213+00:00]>
>>> arrow.now()
<Arrow [2021-03-28T15:43:39.370099+01:00]>

>>> local = arrow.now('Europe/Rome')
>>> local
<Arrow [2021-03-28T16:59:14.093960+02:00]>
>>> local.to('utc')
<Arrow [2021-03-28T14:59:14.093960+00:00]>
>>> local.to('Europe/Moscow')
<Arrow [2021-03-28T17:59:14.093960+03:00]>
>>> local.to('Asia/Tokyo')
<Arrow [2021-03-28T23:59:14.093960+09:00]>
>>> local.datetime
datetime.datetime(
    2021, 3, 28, 16, 59, 14, 93960,
    tzinfo=tzfile('/usr/share/zoneinfo/Europe/Rome')
)
>>> local.isoformat()
'2021-03-28T16:59:14.093960+02:00'
```

Arrow provides a wrapper around the data structures of the standard library, plus a whole set of methods and helpers that simplify the task of dealing with dates and times. You can see from this example how easy it is to get the local date and time in the Italian time zone (Europe/Rome), as well as to convert it to UTC, or to the Russian or Japanese time zones. The last two instructions show how you can get the underlying datetime object from an Arrow one, and the very useful ISO-formatted representation of a date and time.

---
### The collections module

When Python general-purpose built-in containers (tuple, list, set, and dict) aren't enough, we can find specialized container data types in the collections module. They are described in Table 2.1.

| Data type    | Description                                                                |
| ---          | ---                                                                        |
| namedtuple() | Factory function for creating tuple subclasses with named fields           |
| deque        | List-like container with fast appends and pops on either end               |
| ChainMap     | Dictionary-like class for creating a single view of multiple mappings      |
| Counter      | Dictionary subclass for counting hashable objects                          |
| OrderedDict  | Dictionary subclass with methods that allow for re-ordering entries        |
| defaultdict  | Dictionary subclass that calls a factory function to supply missing values |
| UserDict     | Wrapper around dictionary objects for easier dictionary subclassing        |
| UserList     | Wrapper around list objects for easier list subclassing                    |
| UserString   | Wrapper around string objects for easier string subclassing                |

Table 2.1: Collections module data types

There isn't enough space here to cover them all, but you can find plenty of examples in the official documentation; here, we will just give a small example to show you namedtuple, defaultdict, and ChainMap.

#### namedtuple

A namedtuple is a tuple-like object that has fields accessible by attribute lookup, as well as being indexable and iterable (it's actually a subclass of tuple). This is sort of a compromise between a fully-fledged object and a tuple, and it can be useful in those cases where you don't need the full power of a custom object, but only want your code to be more readable by avoiding weird indexing. Another use case is when there is a chance that items in the tuple need to change their position after refactoring, forcing the coder to also refactor all the logic involved, which can be very tricky.

For example, say we are handling data about the left and right eyes of a patient. We save one value for the left eye (position 0) and one for the right eye (position 1) in a regular tuple. Here's how that may look:

```python
>>> vision = (9.5, 8.8)
>>> vision
(9.5, 8.8)
>>> vision[0]  # left eye (implicit positional reference)
9.5
>>> vision[1]  # right eye (implicit positional reference)
8.8
```

Now let's pretend we handle vision objects all of the time, and, at some point, the designer decides to enhance them by adding information for the combined vision, so that a vision object stores data in this format (left eye, combined, right eye). Do you see the trouble we're in now? We may have a lot of code that depends on vision[0] being the left eye information (which it still is) and vision[1] being the right eye information (which is no longer the case). We have to refactor our code wherever we handle these objects, changing vision[1] to vision[2], and that can be painful. We could have probably approached this a bit better from the beginning, by using a namedtuple. Let us show you what we mean:

```python
>>> from collections import namedtuple
>>> Vision = namedtuple('Vision', ['left', 'right'])
>>> vision = Vision(9.5, 8.8)
>>> vision[0]
9.5
>>> vision.left  # same as vision[0], but explicit
9.5
>>> vision.right  # same as vision[1], but explicit
8.8
```

If, within our code, we refer to the left and right eyes using vision.left and vision. right, all we need to do to fix the new design issue is change our factory and the way we create instances—the rest of the code won't need to change:

```python
>>> Vision = namedtuple('Vision', ['left', 'combined', 'right'])
>>> vision = Vision(9.5, 9.2, 8.8)
>>> vision.left  # still correct
9.5
>>> vision.right  # still correct (though now is vision[2])
8.8
>>> vision.combined  # the new vision[1]
9.2
```

You can see how convenient it is to refer to those values by name rather than by position. After all, as a wise man once wrote, Explicit is better than implicit (Can you recall where? Think Zen if you can't...). This example may be a little extreme; of course, it's not likely that our code designer will go for a change like this, but you'd be amazed to see how frequently issues similar to this one occur in a professional environment, and how painful it is to refactor in such cases.

#### defaultdict

The defaultdict data type is one of our favorites. It allows you to avoid checking whether a key is in a dictionary by simply inserting it for you on your first access attempt, with a default value whose type you pass on creation. In some cases, this tool can be very handy and shorten your code a little. Let's see a quick example. Say we are updating the value of age, by adding one year. If age is not there, we assume it was 0 and we update it to 1:

```python
>>> d = {}
>>> d['age'] = d.get('age', 0) + 1  # age not there, we get 0 + 1
>>> d
{'age': 1}
>>> d = {'age': 39}
>>> d['age'] = d.get('age', 0) + 1  # age is there, we get 40
>>> d
{'age': 40}
```

Now let's see how it would work with a defaultdict data type. The second line is actually the short version of an if clause that runs to a length of four lines, and that we would have to write if dictionaries didn't have the get() method (we'll see all about if clauses in Chapter 3, Conditionals and Iteration):

```python
>>> from collections import defaultdict
>>> dd = defaultdict(int)  # int is the default type (0 the value)
>>> dd['age'] += 1  # short for dd['age'] = dd['age'] + 1
>>> dd
defaultdict(<class 'int'>, {'age': 1})  # 1, as expected
```

Notice how we just need to instruct the defaultdict factory that we want an int number to be used if the key is missing (we'll get 0, which is the default for the int type). Also notice that even though in this example there is no gain on the number of lines, there is definitely a gain in readability, which is very important. You can also use a different technique to instantiate a defaultdict data type, which involves creating a factory object. To dig deeper, please refer to the official documentation.

#### ChainMap

ChainMap is an extremely useful data type which was introduced in Python 3.3. It behaves like a normal dictionary but, according to the Python documentation, is provided for quickly linking a number of mappings so they can be treated as a single unit. This is usually much faster than creating one dictionary and running multiple update calls on it. ChainMap can be used to simulate nested scopes and is useful in templating. The underlying mappings are stored in a list. That list is public and can be accessed or updated using the maps attribute. Lookups search the underlying mappings successively until a key is found. By contrast, writes, updates, and deletions only operate on the first mapping.

A very common use case is providing defaults, so let's see an example:

```python
>>> from collections import ChainMap
>>> default_connection = {'host': 'localhost', 'port': 4567}
>>> connection = {'port': 5678}
>>> conn = ChainMap(connection, default_connection) # map creation
>>> conn['port']  # port is found in the first dictionary
5678
>>> conn['host']  # host is fetched from the second dictionary
'localhost'
>>> conn.maps  # we can see the mapping objects
[{'port': 5678}, {'host': 'localhost', 'port': 4567}]
>>> conn['host'] = 'packtpub.com'  # let's add host
>>> conn.maps
[{'port': 5678, 'host': 'packtpub.com'},
 {'host': 'localhost', 'port': 4567}]
>>> del conn['port']  # let's remove the port information
>>> conn.maps
[{'host': 'packtpub.com'}, {'host': 'localhost', 'port': 4567}]
>>> conn['port']  # now port is fetched from the second dictionary
4567
>>> dict(conn)  # easy to merge and convert to regular dictionary
{'host': 'packtpub.com', 'port': 4567}
```

Isn't it just lovely that Python makes your life so easy? You work on a ChainMap object, configure the first mapping as you want, and when you need a complete dictionary with all the defaults as well as the customized items, you can just feed the ChainMap object to a dict constructor. If you have ever coded in other languages, such as Java or C++, you probably will be able to appreciate how precious this is, and how well Python simplifies some tasks.

### Enums

Technically not a built-in data type, as you have to import them from the enum module, but definitely worth mentioning, are enumerations. They were introduced in Python 3.4, and though it is not that common to see them in professional code, we thought it would be a good idea to give you an example anyway for the sake of completeness.

The official definition of an enumeration is that it is a set of symbolic names (members) bound to unique, constant values. Within an enumeration, the members can be compared by identity, and the enumeration itself can be iterated over.

Say you need to represent traffic lights; in your code, you might resort to the following:

```python
>>> GREEN = 1
>>> YELLOW = 2
>>> RED = 4
>>> TRAFFIC_LIGHTS = (GREEN, YELLOW, RED)
>>> # or with a dict
>>> traffic_lights = {'GREEN': 1, 'YELLOW': 2, 'RED': 4}
```

There's nothing special about this code. It's something, in fact, that is very common to find. But, consider doing this instead:

```python
>>> from enum import Enum
>>> class TrafficLight(Enum):
...     GREEN = 1
...     YELLOW = 2
...     RED = 4
...
>>> TrafficLight.GREEN
<TrafficLight.GREEN: 1>
>>> TrafficLight.GREEN.name
'GREEN'
>>> TrafficLight.GREEN.value
1
>>> TrafficLight(1)
<TrafficLight.GREEN: 1>
>>> TrafficLight(4)
<TrafficLight.RED: 4>
```

Ignoring for a moment the (relative) complexity of a class definition, you can appreciate how this approach may be advantageous. The data structure is much cleaner, and the API it provides is much more powerful. We encourage you to check out the official documentation to explore all the great features you can find in the enum module. We think it's worth exploring, at least once. 

## Final considerations

That's it. Now you have seen a very good proportion of the data structures that you will use in Python. We encourage you to take a look at the Python documentation and experiment further with each and every data type we've seen in this chapter. It's worth it—believe us. Everything you'll write will be about handling data, so make sure your knowledge about it is rock solid.

Before we leap into Chapter 3, Conditionals and Iteration, we'd like to share some final considerations about different aspects that, to our minds, are important and not to be neglected.

### Small value caching

While discussing objects at the beginning of this chapter, we saw that when we assigned a name to an object, Python creates the object, sets its value, and then points the name to it. We can assign different names to the same value, and we expect different objects to be created, like this:

```python
>>> a = 1000000
>>> b = 1000000
>>> id(a) == id(b)
False
```

In the preceding example, a and b are assigned to two int objects, which have the same value, but they are not the same object—as you can see, their id is not the same. So, let's do it again:

```python
>>> a = 5
>>> b = 5
>>> id(a) == id(b)
True
```

Uh-oh! Is Python broken? Why are the two objects the same now? We didn't do a = b = 5; we set them up separately. 

Well, the answer is performance. Python caches short strings and small numbers to avoid having many copies of them clogging up the system memory. In the case of strings, caching or, more appropriately, interning them, also provides a significant performance improvement for comparison operations. Everything is handled properly under the hood, so you don't need to worry, but make sure that you remember this behavior should your code ever need to fiddle with IDs.

---
## How to choose data structures

As we've seen, Python provides you with several built-in data types and, sometimes, if you're not that experienced, choosing the one that serves you best can be tricky, especially when it comes to collections. For example, say you have many dictionaries to store, each of which represents a customer. Within each customer dictionary, there's an 'id': 'code' unique identification code. In what kind of collection would you place them? Well, unless we know more about these customers, it's very hard to answer. What kind of access will we need? What sort of operations will we have to perform on each of them, and how many times? Will the collection change over time? Will we need to modify the customer dictionaries in any way? What is going to be the most frequent operation we will have to perform on the collection?

If you can answer the preceding questions, then you will know what to choose. If the collection never shrinks or grows (in other words, it won't need to add/delete any customer object after creation) or shuffles, then tuples are a possible choice. Otherwise, lists are a good candidate. Every customer dictionary has a unique identifier though, so even a dictionary could work. Let us draft these options for you:

```python
# example customer objects
customer1 = {'id': 'abc123', 'full_name': 'Master Yoda'}
customer2 = {'id': 'def456', 'full_name': 'Obi-Wan Kenobi'}
customer3 = {'id': 'ghi789', 'full_name': 'Anakin Skywalker'}
# collect them in a tuple
customers = (customer1, customer2, customer3)
# or collect them in a list
customers = [customer1, customer2, customer3]
# or maybe within a dictionary, they have a unique id after all
customers = {
    'abc123': customer1,
    'def456': customer2,
    'ghi789': customer3,
}
```

Some customers we have there, right? We probably wouldn't go with the tuple option, unless we wanted to highlight that the collection is not going to change. We would say that, usually, a list is better, as it allows for more flexibility.

Another factor to keep in mind is that tuples and lists are ordered collections. If you use a dictionary (prior to Python 3.6) or a set, you would lose the ordering, so you need to know if ordering is important in your application.

What about performance? For example, in a list, operations such as insertion and membership testing can take O(n) time, while they are O(1) for a dictionary. It's not always possible to use dictionaries though, if we don't have the guarantee that we can uniquely identify each item of the collection by means of one of its properties, and that the property in question is hashable (so it can be a key in dict).

If you're wondering what O(n) and O(1) mean, please search "big O notation". In this context, let's just say that if performing an operation Op on a data structure takes O(f(n)), it would mean that Op takes at most a time t ≤ c * f(n) to complete, where c is some positive constant, n is the size of the input, and f is some function. So, think of O(...) as an upper bound for the running time of an operation (it can also be used to size other measurable quantities, of course).

Another way of understanding whether you have chosen the right data structure is by looking at the code you have to write in order to manipulate it. If everything comes easily and flows naturally, then you probably have chosen correctly, but if you find yourself thinking your code is getting unnecessarily complicated, then you probably should try to decide whether you need to reconsider your choices. It's quite hard to give advice without a practical case though, so when you choose a data structure for your data, try to keep ease of use and performance in mind, and give precedence to what matters most in the context you are in.

---
## About indexing and slicing

At the beginning of this chapter, we saw slicing applied to strings. Slicing, in general, applies to a sequence: tuples, lists, strings, and so on. With lists, slicing can also be used for assignment. We have almost never seen this used in professional code, but still, you know you can. Could you slice dictionaries or sets? We hear you scream, Of course not! Excellent — we see that we're on the same page here, so let's talk about indexing.

There is one characteristic regarding Python indexing that we haven't mentioned before. We'll show you by way of an example. How do you address the last element of a collection? Let's see:

```python
>>> a = list(range(10))  # `a` has 10 elements. Last one is 9.
>>> a
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
>>> len(a)  # its length is 10 elements
10
>>> a[len(a) - 1]  # position of last one is len(a) - 1
9
>>> a[-1]  # but we don't need len(a)! Python rocks!
9
>>> a[-2]  # equivalent to len(a) - 2
8
>>> a[-3]  # equivalent to len(a) - 3
7
```

If list a has 10 elements, then due to the 0-index positioning system of Python, the first one is at position 0 and the last one is at position 9. In the preceding example, the elements are conveniently placed in a position equal to their value: 0 is at position 0, 1 at position 1, and so on.

So, in order to fetch the last element, we need to know the length of the whole list (or tuple, or string, and so on) and then subtract 1. Hence: len(a) - 1. This is so common an operation that Python provides you with a way to retrieve elements using negative indexing. This proves very useful when performing data manipulation. Figure 2.2 displays a neat diagram about how indexing works on the string "HelloThere" (which is Obi-Wan Kenobi sarcastically greeting General Grievous in Star Wars: Episode III – Revenge of the Sith):

IMAGE

Figure 2.2: Python indexing

Trying to address indexes greater than 9 or smaller than -10 will raise an IndexError, as expected.

---
## About names

You may have noticed that, in order to keep the examples as short as possible, we have named many objects using simple letters, like a, b, c, d, and so on. This is perfectly fine when debugging on the console or showing that a + b == 7, but it's bad practice when it comes to professional coding (or any type of coding, for that matter). We hope you will indulge us where we have done it; the reason is to present the code in a more compact way.

In a real environment though, when you choose names for your data, you should choose them carefully—they should reflect what the data is about. So, if you have a collection of Customer objects, customers is a perfectly good name for it. Would customers_list, customers_tuple, or customers_collection work as well? Think about it for a second. Is it good to tie the name of the collection to the datatype? We don't think so, at least in most cases. So, if you have an excellent reason to do so, go ahead; otherwise, don't. The reasoning behind this is that once customers_tuple starts being used in different places of your code, and you realize you actually want to use a list instead of a tuple, you're up for some fun refactoring (also known as wasted time). Names for data should be nouns, and names for functions should be verbs. Names should be as expressive as possible. Python is actually a very good example when it comes to names. Most of the time you can just guess what a function is called if you know what it does. Crazy, huh? Chapter 2 from the book Clean Code by Robert C. Martin is entirely dedicated to names. It's an amazing book that helped us improve our coding style in many different ways; it is a must-read if you want to take your coding to the next level.

---
## Summary

In this chapter, we've explored the built-in data types of Python. We've seen how many there are and how much can be achieved just by using them in different combinations.

We've seen number types, sequences, sets, mappings, dates, times, and collections (and a special guest appearance by enumerations). We have also seen that everything is an object, learned the difference between mutable and immutable, and we've also learned about slicing and indexing.

We've presented the cases with simple examples, but there's much more that you can learn about this subject, so stick your nose into the official documentation and go exploring!

Most of all, we encourage you to try out all of the exercises by yourself—get your fingers using that code, build some muscle memory, and experiment, experiment, experiment. Learn what happens when you divide by zero, when you combine different number types into a single expression, and when you massage strings. Play with all data types. Exercise them, break them, discover all their methods, enjoy them, and learn them very, very well. If your foundation is not rock solid, how good can your code be? Data is the foundation for everything; data shapes what dances around it.

The more you progress with the book, the more likely it is that you will find some discrepancies or maybe a small typo here and there in our code (or yours). You will get an error message, something will break. That's wonderful! When you code, things break and you have to debug them, all the time, so consider errors as useful exercises to learn something new about the language you're using, and not as failures or problems. Errors will keep coming up, that's for sure, so you may as well start making your peace with them now.

The next chapter is about conditionals and iteration. We'll see how to actually put collections to use, and make decisions based on the data that we're presented with. We'll start to go a little faster now that your knowledge is building up, so make sure you're comfortable with the contents of this chapter before you move to the next one. Once more, have fun, explore, and break things—it's a very good way to learn.
