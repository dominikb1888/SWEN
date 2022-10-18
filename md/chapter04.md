# Functions, the Building Blocks of Code

    "To create architecture is to put in order. Put what in order? Functions and objects."

     – Le Corbusier

In the previous chapters, we have seen that everything is an object in Python, and functions are no exception. But what exactly is a function? A function is a sequence of instructions that perform a task, bundled together as a unit. This unit can then be imported and used wherever it is needed. There are many advantages to using functions in your code, as we'll see shortly.

In this chapter, we are going to cover the following:

• Functions—what they are and why we should use them
• Scopes and name resolution
• Function signatures—input parameters and return values
• Recursive and anonymous functions
• Importing objects for code reuse

We believe the saying a picture is worth a thousand words is particularly true when explaining functions to someone who is new to this concept, so please take a look at the following diagram in Figure 4.1.

Figure 4.1: An example function

As you can see, a function is a block of instructions, packaged as a whole, like a box. Functions can accept input parameters and produce output values. Both of these are optional, as we'll see in the examples in this chapter.

A function in Python is defined by using the def keyword, after which the name of the function follows, terminated by a pair of parentheses (which may or may not contain input parameters); a colon (:) then signals the end of the function definition line. Immediately afterward, indented by four spaces, we find the body of the function, which is the set of instructions that the function will execute when called.

Note that the indentation by four spaces is not mandatory, but it is the number of spaces suggested by PEP 8, and, in practice, it is the most widely used spacing measure.

A function may or may not return an output. If a function wants to return an output, it does so by using the return keyword, followed by the desired output. The eagle-eyed may have noticed the little * after Optional in the output section of the preceding diagram. This is because a function always returns something in Python, even if 
you don't explicitly use the return clause. If the function has no return statement in its body, or no value is given to the return statement itself, the function returns None. The reasons behind this design choice are outside the scope of an introductory chapter, so all you need to know is that this behavior will make your life easier. As always, thank you, Python.

## Why use functions?

Functions are among the most important concepts and constructs of any language, so let us give you a few reasons why we need them:

• They reduce code duplication in a program. By having a specific task be taken care of by a nice block of packaged code that we can import and call whenever we want, we don't need to duplicate its implementation.
• They help in splitting a complex task or procedure into smaller blocks, each of which becomes a function.
• They hide the implementation details from their users.
• They improve traceability.
• They improve readability.

Let us now look at a few examples to get a better understanding of each point.

### Reducing code duplication
Imagine that you are writing a piece of scientific software, and you need to calculate prime numbers up to a certain limit—as we did in the previous chapter. You have a nice algorithm to calculate them, so you copy and paste it to wherever you need. One day, though, your friend, B. Riemann, gives you a better algorithm to calculate primes, which will save you a lot of time. At this point, you need to go over your whole code base and replace the old code with the new one.

This is actually a bad way to go about it. It's error-prone, you never know what lines you are chopping out or leaving in by mistake or when you might be cutting and pasting code into other code, and you may also risk missing one of the places where prime calculation is done, leaving your software in an inconsistent state where the same action is performed in different places in different ways. What if, instead of replacing code with a better version of it, you need to fix a bug and you miss one of the places? That would be even worse. What if the names of the variables in the old algorithm are different than those used in the new one? That will also complicate things.

So, what should you do? Simple! You write the function get_prime_numbers(upto) and use it anywhere you need a list of primes. When B. Riemann gives you the new code, all you have to do is replace the body of that function with the new implementation, and you're done! The rest of the software will automatically adapt, since it's just calling the function.

Your code will be shorter, it will not suffer from inconsistencies between old and new ways of performing a task, nor will undetected bugs be left behind due to copy-and-paste failures or oversights. Use functions, and you'll only gain from it.

### Splitting a complex task

Functions are also very useful for splitting long or complex tasks into smaller ones. The end result is that the code benefits from it in several ways—for example, through its readability, testability, and reusability. To give you a simple example, imagine that you are preparing a report. Your code needs to fetch data from a data source, parse it, filter it, and polish it, and then a whole series of algorithms needs to be run against it, in order to produce the results that will feed the Report class. It's not uncommon to read procedures like this that are just one big do_report(data_source) function. There are tens or hundreds of lines of code that end with return report.

These situations are slightly more common in scientific code, which tends to be brilliant from an algorithmic point of view but sometimes lacks the touch of experienced programmers when it comes to the style in which they are written. Now, picture a few hundred lines of code. It's very hard to follow through, to find the places where things are changing context (such as finishing one task and starting the next one). Do you have the picture in your mind? Good. Don't do it! Instead, look at this code:

```python
# data.science.example.py
def do_report(data_source):
    # fetch and prepare data
    data = fetch_data(data_source)
    parsed_data = parse_data(data)
    filtered_data = filter_data(parsed_data)
    polished_data = polish_data(filtered_data)

    # run algorithms on data
    final_data = analyse(polished_data)

    # create and return report
    report = Report(final_data)
    return report

```

The previous example is fictitious, of course, but can you see how easy it would be to go through the code? If the end result looks wrong, it would be very easy to debug each of the single data outputs in the do_report function. Moreover, it's even easier to exclude part of the process temporarily from the whole procedure (you just need to comment out the parts that you need to suspend). Code like this is easier to 
deal with.

### Hiding implementation details

Let's stay with the preceding example to talk about this point as well. We can see that, by going through the code of the do_report() function, we can get a pretty good understanding without reading one single line of implementation. This is because functions hide the implementation details. 

This feature means that, if we don't need to delve into the details, we are not forced to, in the way that we would be if do_report was just one big, fat function. In order to understand what was going on, we would have to read every single line of code. With functions, we don't need to. This reduces the time we spend reading the code and since, in a professional environment, reading code takes much more time than actually writing it, it's very important to reduce it to a minimum.

### Improving readability

Programmers sometimes don't see the point in writing a function with a body of one or two lines of code, so let's look at an example that shows you why you should do it.

Imagine that you need to multiply two matrices, like in the example below:

```python

(1 23 4)∙(5 12 1)=(9
3
23 7) 

```
Would you prefer to have to read this code:
```python

# matrix.multiplication.nofunc.py
a = [[1, 2], [3, 4]]
b = [[5, 1], [2, 1]]

c = [[sum(i * j for i, j in zip(r, c)) for c in zip(*b)]
     for r in a]

```

Or would you prefer this:
```python

# matrix.multiplication.func.py
# this function could also be defined in another module
def matrix_mul(a, b):
    return [[sum(i * j for i, j in zip(r, c)) for c in zip(*b)]
            for r in a]

a = [[1, 2], [3, 4]]
b = [[5, 1], [2, 1]]
c = matrix_mul(a, b)

```

It's much easier to understand that c is the result of the multiplication of a and b in the second example, and it's much easier to read through the code. If we don't need to modify that multiplication logic, we don't even need to go into the implementation details. Therefore, readability is improved here, while, in the first snippet, we would have to spend time trying to understand what that complicated list comprehension is doing.

Don't worry if you don't understand list comprehensions, as we will study them in Chapter 5, Comprehensions and Generators.

### Improving traceability

Imagine that we have coded for an e-commerce website. We have displayed the product prices all over the pages. Imagine that the prices in the database are stored with no VAT (sales tax), but we want to display them on the website with VAT at 20%. Here are a few ways of calculating the VAT-inclusive price from the VAT-exclusive price:

```python
# vat.py
price = 100  # GBP, no VAT
final_price1 = price * 1.2
final_price2 = price + price / 5.0
final_price3 = price * (100 + 20) / 100.0
final_price4 = price + price * 0.2
```

These four different ways of calculating a VAT-inclusive price are all perfectly acceptable; we have encountered all of them in the professional code that we have worked on over the years. Now, imagine that we have started selling products in different countries, and some of them have different VAT rates, so we need to refactor the code (throughout the website) in order to make that VAT calculation dynamic.

How do we trace all the places in which we are performing a VAT calculation? Coding today is a collaborative task and we cannot be sure that the VAT has been calculated using only one of those forms. It's going to be difficult. So, let's write a function that takes the input values vat and price (VAT-exclusive) and returns a VAT-inclusive price:
```python

# vat.function.py
def calculate_price_with_vat(price, vat):
    return price * (100 + vat) / 100

```

Now we can import that function and use it in any place in the website where we need to calculate a VAT-inclusive price, and when we need to trace those calls we can search for calculate_price_with_vat.

Note that, in the preceding example, price is assumed to be VAT-exclusive, and vat is a percentage value (for example, 19, 20, or 23).

### Scopes and name resolution

Do you remember when we talked about scopes and namespaces in Chapter 1, A Gentle Introduction to Python? We're going to expand on that concept now. Finally, we can talk in terms of functions, and this will make everything easier to understand. Let's start with a very simple example:

```python
# scoping.level.1.py
def my_function():
    test = 1 # this is defined in the local scope of the function
    print('my_function:', test)

test = 0  # this is defined in the global scope
my_function()
print('global:', test)

```

We have defined the test name in two different places in the previous example—it is actually in two different scopes. One is the global scope (test = 0), and the other is the local scope of the my_function() function (test = 1). If we execute the code, we will see this:

```bash
$ python scoping.level.1.py
my_function: 1
global: 0

```

It's clear that test = 1 shadows the test = 0 assignment in my_function(). In the global context, test is still 0, as you can see from the output of the program, but we define the test name again in the function body, and we set it to point to an integer of value 1. Both of the two test names therefore exist: one in the global scope, pointing to an int object with a value of 0, the other in the my_function() scope, pointing to an int object with a value of 1. Let's comment out the line with test = 1. Python searches for the test name in the next enclosing namespace (recall the LEGB rule: local, enclosing, global, built-in, described in Chapter 1, A Gentle Introduction to Python) and, in this case, we will see the value 0 printed twice. Try it in your code.

Now, let's raise the stakes here and level up:
```python

# scoping.level.2.py
def outer():
    test = 1  # outer scope

    def inner():
        test = 2  # inner scope
        print('inner:', test)

    inner()
    print('outer:', test)

test = 0  # global scope
outer()
print('global:', test)

```

In the preceding code, we have two levels of shadowing. One level is in the function outer, and the other one is in the function inner(). It is far from rocket science, but it can be tricky. If we run the code, we get:

```bash
$ python scoping.level.2.py
inner: 2
outer: 1
global: 0

```

Try commenting out the test = 1 line. Can you figure out what the result will be? Well, when reaching the print('outer:', test) line, Python will have to look for test in the next enclosing scope; therefore it will find and print 0, instead of 1. Make sure you comment out test = 2 as well, to see whether you understand what happens and whether the LEGB rule is clear, before proceeding. Another thing to note is that Python gives us the ability to define a function in another function. The inner() function's name is defined within the namespace of the outer() function, exactly as would happen with any other name.

### The global and nonlocal statements

In the preceding example, we can alter what happens to the shadowing of the test name by using one of these two special statements: global and nonlocal. As you can see from the previous example, when we define test = 2 in the inner() function, we overwrite test neither in the outer() function nor in the global scope. 

We can get read access to those names if we use them in a nested scope that doesn't define them, but we cannot modify them because when we write an assignment instruction, we're actually defining a new name in the current scope.

How do we change this behavior? Well, we can use the nonlocal statement. 
According to the official documentation:

    "The nonlocal statement causes the listed identifiers to refer to previously bound variables in the nearest enclosing scope excluding globals."

Let's introduce it in the inner() function and see what happens:
```python

# scoping.level.2.nonlocal.py
def outer():
    test = 1  # outer scope

    def inner():
        nonlocal test
        test = 2  # nearest enclosing scope (which is 'outer')
        print('inner:', test)

    inner()
    print('outer:', test)

test = 0  # global scope
outer()
print('global:', test)

```

Notice how in the body of the inner() function, we have declared the test name to be nonlocal. Running this code produces the following result:

```bash
$ python scoping.level.2.nonlocal.py
inner: 2
outer: 2
global: 0

```

Wow, look at that result! It means that, by declaring test to be nonlocal in the inner() function, we actually get to bind the test name to the one declared in the outer function. If we removed the nonlocal test line from the inner() function and tried the same trick in the outer() function, we would get a SyntaxError, because the nonlocal statement works on enclosing scopes, excluding the global one.

Is there a way to get to that test = 0 in the global namespace then? Of course—we just need to use the global statement:

```python
# scoping.level.2.global.py
def outer():
    test = 1  # outer scope

    def inner():
        global test
        test = 2  # global scope
        print('inner:', test)

    inner()
    print('outer:', test)

test = 0  # global scope
outer()
print('global:', test)

```

Note that we have now declared the test name to be global, which will basically bind it to the one we defined in the global namespace (test = 0). Run the code and you should get the following:
```bash
$ python scoping.level.2.global.py
inner: 2
outer: 1
global: 2

```

This shows that the name affected by the test = 2 assignment is now the global one. This trick would also work in the outer function because, in this case, we're referring to the global scope. Try it for yourself and see what changes, and get comfortable with scopes and name resolution—it's very important. Also, can you tell what would happen if you defined inner() outside outer() in the preceding examples?

## Input parameters

At the beginning of this chapter, we saw that a function can take input parameters. Before we delve into all the possible types of parameters, let's make sure you have a clear understanding of what passing an argument to a function means. There are three key points to keep in mind:

•  Argument-passing is nothing more than assigning an object to a local 
variable name
•  Assigning an object to an argument name inside a function doesn't affect the caller
•  Changing a mutable object argument in a function affects the caller

Before we explore the topic of arguments any further, please allow us to clarify the terminology a little. According to the official Python documentation:

    "Parameters are defined by the names that appear in a function definition, whereas arguments are the values actually passed to a function when calling it. Parameters define what types of arguments a function can accept."

We will try to be precise when referring to parameters and arguments, but it is worth noting that they are sometimes used synonymously as well. Let's now look at some examples.

### Argument-passing

Take a look at the following code. We declare a name, x, in the global scope, then we declare a function, func(y), and finally we call it, passing x:
```python

# key.points.argument.passing.py
x = 3
def func(y):
    print(y)

func(x)  # prints: 3

```

When func() is called with x, within its local scope, a name, y, is created, and it's pointed to the same object that x is pointing to. This is better clarified in Figure 4.2 (don't worry about the fact that this example was run with Python 3.6—this is a feature that hasn't changed).

The right-hand side of Figure 4.2 depicts the state of the program when execution has reached the end, after func() has returned (None). Take a look at the Frames column, and note that we have two names, x and func, in the global namespace (Global frame), pointing respectively to an int (with a value of 3) and to a function object. Right beneath it, in the rectangle titled func, we can see the function's local namespace, in which only one name has been defined: y. Because we have called func() with x (line 5 on the left side of the figure), y is pointing to the same object that x is. This is what happens under the hood when an argument is passed to a function. If we had used the name x instead of y in the function definition, things would have been exactly the same (but perhaps a bit confusing at first)—there would be a local x in the function, and a global x outside, as we saw in the Scopes and name resolution section previously in this chapter.

So, in a nutshell, what really happens is that the function creates, in its local scope, the names defined as parameters and, when we call it, we basically tell Python which objects those names must be pointed toward.

### Assignment to parameter names 

Assignment to parameter names doesn't affect the caller. This is something that can be tricky to understand at first, so let's look at an example:
```python

# key.points.assignment.py
x = 3
def func(x):
    x = 7  # defining a local x, not changing the global one

func(x)
print(x)  # prints: 3

```

In the preceding code, when we call the function with func(x), the instruction x = 7 is executed within the local scope of the func() function; the name, x, is pointed to an integer with a value of 7, leaving the global x unaltered.

### Changing a mutable object

Changing a mutable object affects the caller. This is the final point, and it's very important because Python apparently behaves differently with mutable objects (just apparently, though). Let's look at an example:
```python

# key.points.mutable.py
x = [1, 2, 3]
def func(x):
    x[1] = 42  # this affects the 'x' argument!

func(x)
print(x)  # prints: [1, 42, 3]

```

Wow, we actually changed the original object! If you think about it, there is nothing weird in this behavior. The name x in the function is set to point to the caller object by the function call; within the body of the function, we are not changing x, in that we're not changing its reference, or, in other words, we are not changing which object x is pointing to. We are merely accessing the element at position 1 in that object, and changing its value. Remember point 2 the Input parameters section: Assigning an object to an parameter name within a function doesn't affect the caller. If that is clear to you, the following code should not be surprising:
```python

# key.points.mutable.assignment.py
x = [1, 2, 3]
def func(x):
    x[1] = 42  # this changes the original 'x' argument!
    x = 'something else'  # this points x to a new string object

func(x)
print(x)  # still prints: [1, 42, 3]

```

Take a look at the two lines we have highlighted. At first, like before, we just access the caller object again, at position 1, and change its value to number 42. Then, we reassign x to point to the 'something else' string. This leaves the caller unaltered and, in fact, the output is the same as that of the previous snippet.

Take your time to play around with this concept, and experiment with prints and calls to the id function until everything is clear in your mind. This is one of the key aspects of Python and it must be very clear, otherwise you risk introducing subtle bugs into your code. Once again, the Python Tutor website (http://www.pythontutor.com/) will help you a lot by giving you a visual representation of these concepts.

Now that we have a good understanding of input parameters and how they behave, let's look at the different ways of passing arguments to functions.

## Passing arguments

There are four different ways of passing arguments to a function:

- Positional arguments
- Keyword arguments
- Dictionary unpacking
- Iterable unpacking

Let's take a look at them one by one.

### Positional arguments

When we call a function, each positional argument is assigned to the parameter in the corresponding position in the function definition:
```python

# arguments.positional.py
def func(a, b, c):
    print(a, b, c)

func(1, 2, 3)  # prints: 1 2 3

```

This is the most common way of passing arguments to functions (and in some programming languages this is also the only way of passing arguments).


### Keyword arguments

Keyword arguments in a function call are assigned to parameters using the 
name=value syntax:

```python

# arguments.keyword.py
def func(a, b, c):
    print(a, b, c)

func(a=1, c=2, b=3)  # prints: 1 3 2

```

When we use keyword arguments, the order of the arguments does not need to match the order of the parameters in the function definition. This can make our code easier to read and debug. We don't need to remember (or look up) the order of parameters in a function definition. We can look at a function call and immediately know which argument corresponds to which parameter.

You can also use both positional and keyword arguments at the same time:
```python

# arguments.positional.keyword.py
def func(a, b, c):
    print(a, b, c)

func(42, b=1, c=2)

```

Keep in mind, however, that positional arguments always have to be listed before any keyword arguments. For example, if you try something like this:
```bash

# arguments.positional.keyword.py
func(b=1, c=2, 42)  # positional argument after keyword arguments
```

You will get an error:
```bash
$ python arguments.positional.keyword.py
  File "arguments.positional.keyword.py", line 7
    func(b=1, c=2, 42)  # positional argument after keyword arguments
                    ^
    SyntaxError: positional argument follows keyword argument
```


### Iterable unpacking

Iterable unpacking uses the syntax `*iterable_name` to pass the elements of an iterable as positional arguments to a function:


```python

# arguments.unpack.iterable.py
def func(a, b, c):
    print(a, b, c)

values = (1, 3, -7)
func(*values)  # equivalent to: func(1, 3, -7)

```

This is a very useful feature, particularly when we need to programmatically generate arguments for a function.


### Dictionary unpacking

Dictionary unpacking is to keyword arguments what iterable unpacking is to positional arguments. We use the syntax **dictionary_name to pass keyword arguments, constructed from the keys and values of a dictionary, to a function:

```python

# arguments.unpack.dict.py
def func(a, b, c):
    print(a, b, c)

values = {'b': 1, 'c': 2, 'a': 42}
func(**values)  # equivalent to func(b=1, c=2, a=42)

```

## Combining argument types

We've already seen that positional and keyword arguments can be used together, as long as they are passed in the proper order. As it turns out, we can also combine unpacking (of both kinds) with normal positional and keyword arguments. We can even unpack multiple iterables and multiple dictionaries!

Arguments must be passed in the following order:

•  First, positional arguments: both ordinary (name) and iterable unpacking (*name)
•  Next come keyword arguments (name=value), which can be mixed with iterable unpacking (*name)
•  Finally, there is dictionary unpacking (**name), which can be mixed with keyword arguments (name=value)

This will be much easier to understand with an example:
```python

# arguments.combined.py
def func(a, b, c, d, e, f):
    print(a, b, c, d, e, f)

func(1, *(2, 3), f=6, *(4, 5))
func(*(1, 2), e=5, *(3, 4), f=6)
func(1, **{'b': 2, 'c': 3}, d=4, **{'e': 5, 'f': 6})
func(c=3, *(1, 2), **{'d': 4}, e=5, **{'f': 6})

```

All the calls to func() above are equivalent. Play around with this example until you are sure you understand it. Pay close attention to the errors you get when you get the order wrong.

The ability to unpack multiple iterables and dictionaries was introduced to Python by PEP 448. This PEP also introduced the ability to use unpacking in contexts other than just function calls. You can read all about it at https://www.python.org/dev/peps/pep-0448/.

When combining positional and keyword arguments, it is important to remember that each parameter can only appear once in the argument list:
```python

# arguments.multiple.value.py
def func(a, b, c):
    print(a, b, c)

func(2, 3, a=1)

```

Here, we are passing two values for parameter a: the positional argument 2 and the keyword argument a=1. This is illegal, so we get an error when we try to run it:
```python

$ python arguments.multiple.value.py
Traceback (most recent call last):
  File "arguments.multiple.value.py", line 5, in <module>
    func(2, 3, a=1)
TypeError: func() got multiple values for argument 'a'

```

## Defining parameters

Function parameters can be classified into five groups.

• Positional or keyword parameters: allow both positional and keyword arguments
• Variable positional parameters: collect an arbitrary number of positional arguments in a tuple
• Variable keyword parameters: collect an arbitrary number of keyword arguments in a dictionary
• Positional-only parameters: can only be passed as positional arguments
• Keyword-only parameters: can only be passed as keyword arguments

All the parameters in the examples we have seen so far in this chapter are normal positional or keyword parameters. We've seen how they can be passed as both positional and keyword arguments. There's not much more to say about them, so let's look at the other categories. Before we do though, let's briefly look at optional parameters.

### Optional parameters

Apart from the categories we've looked at here, parameters can also be classified as either required or optional. Optional parameters have a default value specified in the function definition. The syntax is name=value:
```python

# parameters.default.py
def func(a, b=4, c=88):
    print(a, b, c)

func(1)              # prints: 1 4 88
func(b=5, a=7, c=9)  # prints: 7 5 9
func(42, c=9)        # prints: 42 4 9
func(42, 43, 44)     # prints: 42, 43, 44

```

Here, a is required, while b has the default value 4 and c has the default value 88. It's important to note that, with the exception of keyword-only parameters, required parameters must always be to the left of all optional parameters in the function definition. Try removing the default value from c in the above example and see for yourself what happens.

### Variable positional parameters

Sometimes you may prefer not to specify the exact number of positional parameters to a function; Python provides you with the ability to do this by using variable positional parameters. Let's look at a very common use case, the minimum() function. This is a function that calculates the minimum of its input values:
```python

# parameters.variable.positional.py
def minimum(*n):
    # print(type(n))  # n is a tuple
    if n:  # explained after the code
        mn = n[0]
        for value in n[1:]:
            if value < mn:
                mn = value
        print(mn)

minimum(1, 3, -7, 9)  # n = (1, 3, -7, 9) - prints: -7
minimum()             # n = () - prints: nothing

```

As you can see, when we define a parameter with an asterisk(*) prepended to its name, we are telling Python that this parameter will collect a variable number of positional arguments when the function is called. Within the function, n is a tuple. Uncomment print(type(n)) to see for yourself, and play around with it for a bit.

Note that a function can have at most one variable positional parameter—it wouldn't make sense to have more. Python would have no way of deciding how to divide up the arguments between them. You are also unable to specify a default value for a variable positional parameter. The default value is always an empty tuple.

Have you noticed how we checked whether n wasn't empty with a simple if n:? This is because collection objects evaluate to True when non-empty, and otherwise False, in Python. This is the case for tuples, sets, lists, dictionaries, and so on.

One other thing to note is that we may want to throw an error when we call the function with no parameters, instead of silently doing nothing. In this context, we're not concerned about making this function robust, but rather understanding variable positional parameters.

Did you notice that the syntax for defining variable positional parameters looks very much like the syntax for iterable unpacking? This is no coincidence. After all, the two features mirror each other. They are also frequently used together, since variable positional parameters save you from worrying whether the length of the iterable you're unpacking matches the number of parameters in the function definition.

### Variable keyword parameters

Variable keyword parameters are very similar to variable positional parameters. The only difference is the syntax (** instead of *) and the fact that they are collected in a dictionary:

```python
# parameters.variable.keyword.py
def func(**kwargs):
    print(kwargs)

func(a=1, b=42)  # prints {'a': 1, 'b': 42}
func()  # prints {}
func(a=1, b=46, c=99)  # prints {'a': 1, 'b': 46, 'c': 99}

```

You can see that adding ** in front of the parameter name in the function definition tells Python to use that name to collect a variable number of keyword parameters. As in the case of variable positional parameters, each function can have at most one variable keyword parameter—and you cannot specify a default value.

Just like variable positional parameters resemble iterable unpacking, variable keyword parameters resemble dictionary unpacking. Dictionary unpacking is also often used to pass arguments to functions with variable keyword parameters.

The reason why being able to pass a variable number of keyword arguments is so important may not be evident at the moment, so how about a more realistic example? Let's define a function that connects to a database: we want to connect to a default database by simply calling this function with no parameters. We also want to connect to any other database by passing to the function the appropriate parameters. Before you read on, try to spend a couple of minutes figuring out a solution by yourself:

```python
# parameters.variable.db.py
def connect(**options):
    conn_params = {
        'host': options.get('host', '127.0.0.1'),
        'port': options.get('port', 5432),
        'user': options.get('user', ''),
        'pwd': options.get('pwd', ''),
    }
    print(conn_params)
    # we then connect to the db (commented out)
    # db.connect(**conn_params)

connect()
connect(host='127.0.0.42', port=5433)
connect(port=5431, user='fab', pwd='gandalf')

```

Note that, in the function, we can prepare a dictionary of connection parameters (conn_params) using default values as fallbacks, allowing them to be overwritten if they are provided in the function call. There are better ways to do this with fewer lines of code, but we're not concerned with that right now. Running the preceding code yields the following result:
```python

$ python parameters.variable.db.py
{'host': '127.0.0.1', 'port': 5432, 'user': '', 'pwd': ''}
{'host': '127.0.0.42', 'port': 5433, 'user': '', 'pwd': ''}
{'host': '127.0.0.1', 'port': 5431, 'user': 'fab', 'pwd': 'gandalf'}

```

Note the correspondence between the function calls and the output, and how default values are overridden according to what was passed to the function.

### Positional-only parameters

Starting from Python 3.8, PEP 570 (https://www.python.org/dev/peps/pep-0570/) introduced positional-only parameters. There is a new function parameter syntax, /, indicating that a set of the function parameters must be specified positionally and cannot be passed as keyword arguments. Let's see a simple example:

```python
# parameters.positional.only.py
def func(a, b, /, c):
    print(a, b, c)

func(1, 2, 3)  # prints: 1 2 3
func(1, 2, c=3)  # prints 1 2 3

```

In the preceding example, we define a function func(), which specifies three parameters: a, b, and c. The / in the function signature indicates that a and b must be passed positionally, that is, not by keyword. The last two lines in the example show that we can call the function passing all three arguments positionally, or we can pass c by keyword. Both cases work fine, as c is defined after the / in the function signature. If we try to call the function by passing a or b by keyword, like so:
```python

func(1, b=2, c=3)

This produces the following traceback:

Traceback (most recent call last):
  File "arguments.positional.only.py", line 7, in <module>
    func(1, b=2, c=3)
TypeError: func() got some positional-only arguments
passed as keyword arguments: 'b'

```

The preceding example show us that Python is now complaining about how we 
called func(). We have passed b by keyword, but we are not allowed to do that.

Positional-only parameters can also be optional:
```python

# parameters.positional.only.optional.py
def func(a, b=2, /):
    print(a, b)
func(4, 5)  # prints 4 5
func(3)  # prints 3 2

```

Let's see what this feature brings to the language with a few examples borrowed from the official documentation. One advantage is the ability to fully emulate behaviors of existing C-coded functions:

```python

def divmod(a, b, /):
    "Emulate the built in divmod() function"
    return (a // b, a % b)

```

Another important use case is to preclude keyword arguments when the parameter name is not helpful:

```python
len(obj='hello')
```

In the preceding example, the obj keyword argument impairs readability. Moreover, if we wish to refactor the internals of the len function, and rename obj to the_object (or any other name), the change is guaranteed not to break any client code, because there won't be any call to the len() function involving the now stale obj parameter name.

Finally, using positional-only parameters implies that whatever is on the left of / remains available for use in variable keyword arguments, as shown by the following example:
```python

def func_name(name, /, **kwargs):
    print(name)
    print(kwargs)

func_name('Positional-only name', name='Name in **kwargs')

# Prints:
# Positional-only name
# {'name': 'Name in **kwargs'}

```

The ability to retain parameter names in function signatures for use in **kwargs can lead to simpler and cleaner code.

Let us now explore the mirror version of positional-only: keyword-only parameters.

### Keyword-only parameters

Python 3 introduced keyword-only parameters. We are going to study them only briefly, as their use cases are not that frequent. There are two ways of specifying them, either after the variable positional parameters, or after a bare *. Let's see an example of both:

```python
# parameters.keyword.only.py
def kwo(*a, c):
    print(a, c)

kwo(1, 2, 3, c=7)  # prints: (1, 2, 3) 7
kwo(c=4)           # prints: () 4
# kwo(1, 2)  # breaks, invalid syntax, with the following error
# TypeError: kwo() missing 1 required keyword-only argument: 'c'

def kwo2(a, b=42, *, c):
    print(a, b, c)

kwo2(3, b=7, c=99)  # prints: 3 7 99
kwo2(3, c=13)       # prints: 3 42 13
# kwo2(3, 23)  # breaks, invalid syntax, with the following error
# TypeError: kwo2() missing 1 required keyword-only argument: 'c'
```

As anticipated, the function, kwo(), takes a variable number of positional parameters (a) and a keyword-only one, c. The results of the calls are straightforward and you can uncomment the third call to see what error Python returns.

The same applies to the function kwo2(), which differs from kwo in that it takes a positional argument, a, a keyword argument, b, and then a keyword-only one, c. You can uncomment the third call to see the error that is produced.

Now that you know how to specify different types of input parameters, let's see how you can combine them in function definitions.

### Combining input parameters

You can combine different parameter types in the same function (in fact it is often very useful to do so). As in the case of combining different types of arguments in the same function call, there are some restrictions on ordering:

•  Positional-only parameters come first, followed by a /.
•  Normal parameters go after any positional-only parameters.
•  Variable positional parameters go after normal parameters.
•  Keyword-only parameters go after variable positional parameters.
•  Variable keyword parameters always go last.
•  For positional-only and normal parameters, any required parameters have to be defined before any optional parameters. This means that if you have an optional positional-only parameter, all your normal parameters must be optional too. This rule doesn't affect keyword-only parameters.

These rules can be a bit tricky to understand without an example, so let's look at a couple:
```python

# parameters.all.py
def func(a, b, c=7, *args, **kwargs):
    print('a, b, c:', a, b, c)
    print('args:', args)
    print('kwargs:', kwargs)

func(1, 2, 3, 5, 7, 9, A='a', B='b')

```

Note the order of the parameters in the function definition. The execution of this yields the following:
```python

$ python parameters.all.py
a, b, c: 1 2 3
args: (5, 7, 9)
kwargs: {'A': 'a', 'B': 'b'}

```

Let's now look at an example with keyword-only parameters:
```python

# parameters.all.pkwonly.py
def allparams(a, /, b,  c=42, *args, d=256, e, **kwargs):
    print('a, b, c:', a, b, c)
    print('d, e:', d, e)
    print('args:', args)
    print('kwargs:', kwargs)

allparams(1, 2, 3, 4, 5, 6, e=7, f=9, g=10)

```

Note that we have both positional-only and keyword-only parameters in the function declaration: a is positional-only, while d and e are keyword-only. They come after the *args variable positional argument, and it would be the same if they came right after a single * (in which case there wouldn't be any variable positional parameter). The 
execution of this yields the following:
```python

$ python parameters.all.pkwonly.py
a, b, c: 1 2 3
d, e: 256 7
args: (4, 5, 6)
kwargs: {'f': 9, 'g': 10}

```

One other thing to note is the names we gave to the variable positional and keyword parameters. You're free to choose differently, but be aware that args and kwargs are the conventional names given to these parameters, at least generically.

### More signature examples

To briefly recap on function signatures that use the positional- and keyword-only specifiers, here are some further examples. Omitting the variable positional and keyword parameters, for brevity, we are left with the following syntax:
```python

def func_name(positional_only_parameters, /,
    positional_or_keyword_parameters, *,
    keyword_only_parameters):

```

First, we have positional-only, then positional or keyword parameters, and finally keyword-only ones.

Some other valid signatures are presented below:
```python

def func_name(p1, p2, /, p_or_kw, *, kw):
def func_name(p1, p2=None, /, p_or_kw=None, *, kw):
def func_name(p1, p2=None, /, *, kw):
def func_name(p1, p2=None, /):
def func_name(p1, p2, /, p_or_kw):
def func_name(p1, p2, /):

```

All of the above are valid signatures, while the following would be invalid:
```python

def func_name(p1, p2=None, /, p_or_kw, *, kw):
def func_name(p1=None, p2, /, p_or_kw=None, *, kw):
def func_name(p1=None, p2, /):

```

You can read about the grammar specifications in the official documentation:

https://docs.python.org/3/reference/compound_stmts.html#function-definitions

A useful exercise for you at this point would be to implement any of the above example signatures, printing out the values of those parameters, like we have done in previous exercises, and play around passing arguments in different ways.

### Avoid the trap! Mutable defaults

One thing to be aware of, in Python, is that default values are created at definition time; therefore, subsequent calls to the same function will possibly behave differently according to the mutability of their default values. Let's look at an example:
```python

# parameters.defaults.mutable.py
def func(a=[], b={}):
    print(a)
    print(b)
    print('#' * 12)
    a.append(len(a))  # this will affect a's default value
    b[len(a)] = len(a)  # and this will affect b's one

func()
func()
func()

```

Both parameters have mutable default values. This means that, if you affect those objects, any modification will stick around in subsequent function calls. See if you can understand the output of those calls:

```bash

$ python parameters.defaults.mutable.py
[]
{}
############
[0]
{1: 1}
############
[0, 1]
{1: 1, 2: 2}
############

```

It's interesting, isn't it? While this behavior may seem very weird at first, it actually makes sense, and it's very handy—when using memoization techniques, for example. Even more interesting is what happens when, between the calls, we introduce one that doesn't use defaults, such as this:
```python

# parameters.defaults.mutable.intermediate.call.py 
func()
func(a=[1, 2, 3], b={'B': 1})
func()

```

When we run this code, this is the output:

```python

$ python parameters.defaults.mutable.intermediate.call.py
[]
{}
############
[1, 2, 3]
{'B': 1}
############
[0]
{1: 1}
############

```

This output shows us that the defaults are retained even if we call the function with other values. One question that comes to mind is, how do I get a fresh empty value every time? Well, the convention is the following:
```python

# parameters.defaults.mutable.no.trap.py
def func(a=None):
    if a is None:
        a = []
    # do whatever you want with 'a' ...

```

Note that, by using the preceding technique, if a isn't passed when calling the function, we always get a brand new, empty list.

After a thorough exposition of input parameters, it's now time to look at the other side of the coin, output parameters.

## Return values

The return values of functions are one of those things where Python is ahead of the competition. In most other languages, functions are usually allowed to return only one object but, in Python, you can return a tuple—which implies that you can return whatever you want. This feature allows a programmer to write software that would be much harder to write in other languages, or certainly more tedious. We've already said that to return something from a function we need to use the return statement, followed by what we want to return. There can be as many return statements as needed in the body of a function.

On the other hand, if within the body of a function we don't return anything, or we invoke a bare return statement, the function will return None. This behavior is harmless when it's not needed, but allows for interesting patterns, and confirms Python as a very consistent language.

We say it's harmless because you are never forced to collect the result of a function call. We'll show you what we mean with an example:

```python

# return.none.py
def func():
    pass

func()  # the return of this call won't be collected. It's lost.
a = func()  # the return of this one instead is collected into 'a'
print(a)  # prints: None

```

Note that the whole body of the function is composed only of the pass statement. As the official documentation tells us, pass is a null operation, as, when it is executed, nothing happens. It is useful as a placeholder when a statement is required syntactically but no code needs to be executed. In other languages, we would probably just indicate that with a pair of curly brackets ({}), which define an empty scope; but in Python, a scope is defined by indenting code, therefore a statement such as pass is necessary.

Notice also that the first call to func() returns a value (None) that we don't collect. As we mentioned before, collecting the return value of a function call is not mandatory.

That's all well, but not very interesting, so how about we write an interesting function? Remember that, in Chapter 1, A Gentle Introduction to Python, we talked about the factorial function. Let's write our own implementation here (for simplicity, we will assume the function is always called correctly with appropriate values, so we won't need to sanity-check the input argument):

```python

# return.single.value.py
def factorial(n):
    if n in (0, 1):
        return 1
    result = n
    for k in range(2, n):
        result *= k
    return result

f5 = factorial(5)  # f5 = 120

```

Note that we have two points of return. If n is either 0 or 1, we return 1. Otherwise, we perform the required calculation and return result. 

In Python it's common to use the in operator to do a membership check, as we did in the preceding example, instead of the more verbose:

```python

if n == 0 or n == 1:

    ...

```

Let's now try to write this function a little bit more succinctly:

```python

# return.single.value.2.py
from functools import reduce
from operator import mul

def factorial(n):
    return reduce(mul, range(1, n + 1), 1)

f5 = factorial(5)  # f5 = 120

```

This simple example shows how Python is both elegant and concise. This implementation is readable even if we have never seen reduce() or mul(). If you can't read or understand it, set aside a few minutes and do some research in the Python documentation until its behavior is clear to you. Being able to look up functions in the documentation and understand code written by someone else is a task that every developer needs to be able to perform, so take this as a challenge.

To this end, make sure you look up the help() function, which proves quite helpful when exploring with the console.

### Returning multiple values

As we mentioned before, unlike in most other languages, in Python it's very easy to return multiple objects from a function. This feature opens up a whole world of possibilities and allows you to code in a style that is hard to reproduce with other languages. Our thinking is limited by the tools we use; therefore, when Python gives you more freedom than other languages, it is boosting your ability to be creative. 

To return multiple values is very easy: you just use tuples (either explicitly or implicitly). Let's look at a simple example that mimics the divmod() built-in function:

```python

# return.multiple.py
def moddiv(a, b):
    return a // b, a % b

print(moddiv(20, 7))  # prints (2, 6)

```

We could have wrapped the part that is in bold in the preceding code in brackets, making it an explicit tuple, but there's no need for that. The preceding function returns both the result and the remainder of the division, at the same time.

In the source code for this example, we have left a simple example of a test function to make sure the code is doing the correct calculation.

## A few useful tips

When writing functions, it's very useful to follow guidelines so that you write them well. We'll quickly point some of them out.

### Functions should do one thing

Functions that do one thing are easy to describe in one short sentence; functions that do multiple things can be split into smaller functions that do one thing. These smaller functions are usually easier to read and understand.

### Functions should be small

The smaller they are, the easier it is to test and write them so that they do one thing.

### The fewer input parameters, the better

Functions that take a lot of parameters quickly become hard to manage (among other issues).

### Functions should be consistent in their return values

Returning False and returning None are not the same thing, even if, within a Boolean context, they both evaluate to False. False means that we have information (False), while None means that there is no information. Try writing functions that return in a consistent way, no matter what happens in their logic.

### Functions shouldn't have side effects

In other words, functions should not affect the values you call them with. This is probably the hardest statement to understand at this point, so we'll give you an example using lists. In the following code, note how numbers is not sorted by the sorted() function, which actually returns a sorted copy of numbers. Conversely, the list.sort() method is acting on the numbers object itself, and that is fine because it is a method (a function that belongs to an object and therefore has the right to modify it):
```python

>>> numbers = [4, 1, 7, 5]
>>> sorted(numbers)  # won't sort the original 'numbers' list
[1, 4, 5, 7]
>>> numbers  # let's verify
[4, 1, 7, 5]  # good, untouched
>>> numbers.sort()  # this will act on the list
>>> numbers
[1, 4, 5, 7]

```

Follow these guidelines and you will write better functions, which will serve you well. Chapter 3 of Clean Code, by Robert C. Martin, is dedicated to functions, and it's one of the best sets of guidelines we have ever read on the subject.

## Recursive functions

When a function calls itself to produce a result, it is said to be recursive. Sometimes recursive functions are very useful, in that they make it easier to write code—some algorithms are very easy to write using the recursive paradigm, while others are not. There is no recursive function that cannot be rewritten in an iterative fashion, so it's usually up to the programmer to choose the best approach for the case at hand.

The body of a recursive function usually has two sections: one where the return value depends on a subsequent call to itself, and one where it doesn't (called the base case). As an example, we can consider the (hopefully now familiar) factorial function, N!. The base case is when N is either 0 or 1—the function returns 1 with no need for further calculation. On the other hand, in the general case, N! returns the product:

    1 * 2 * ... * (N-1) * N 

If you think about it, N! can be rewritten like this: N! = (N-1)! * N. As a practical example, consider this:

    5! = 1 * 2 * 3 * 4 * 5 = (1 * 2 * 3 * 4) * 5 = 4! * 5

Let's write this down in code:
```python

# recursive.factorial.py
def factorial(n):
    if n in (0, 1):  # base case
        return 1
    return factorial(n - 1) * n  # recursive case

```

Recursive functions are often used when writing algorithms, and they can be really fun to write. As an exercise, try to solve a couple of simple problems using both a recursive and an iterative approach. Good candidates for practice might be calculating Fibonacci numbers, or the length of a string—things like that.

When writing recursive functions, always consider how many nested calls you make, since there is a limit. For further information on this, check out sys.getrecursionlimit() and sys.setrecursionlimit().

## Anonymous functions

One last type of function that we want to talk about are anonymous functions. These functions, which are called lambdas in Python, are usually used when a fully fledged function with its own name would be overkill, and all we want is a quick, simple one-liner that does the job. Imagine that we wanted a list of all the numbers up to a certain value of N that are also multiples of five. We could use the filter() function for this, which will require a function and an iterable as input. The return value is a filter object that, when you iterate over it, yields the elements from the input iterable for which the function returns True. Without using an anonymous function, we might do something like this:
```python

# filter.regular.py
def is_multiple_of_five(n):
    return not n % 5

def get_multiples_of_five(n):
    return list(filter(is_multiple_of_five, range(n)))

```

Note how we use is_multiple_of_five() to filter the first n natural numbers. This seems a bit excessive—the task is simple and we don't need to keep the is_multiple_ of_five() function around for anything else. Let's rewrite it using a lambda function:
```python

# filter.lambda.py
def get_multiples_of_five(n):
    return list(filter(lambda k: not k % 5, range(n)))

```

The logic is exactly the same, but the filtering function is now a lambda. Defining a lambda is very easy and follows this form: func_name = lambda [parameter_list]: expression. A function object is returned, which is equivalent to this: def func_name([parameter_list]): return expression.

Note that optional parameters are indicated following the common syntax of wrapping them in square brackets.

Let's look at another couple of examples of equivalent functions, defined in both forms:
```python

# lambda.explained.py
# example 1: adder
def adder(a, b):
    return a + b

# is equivalent to:
adder_lambda = lambda a, b: a + b

# example 2: to uppercase
def to_upper(s):
    return s.upper()

# is equivalent to:
to_upper_lambda = lambda s: s.upper()

```

The preceding examples are very simple. The first one adds two numbers, and the second one produces the uppercase version of a string. Note that we assigned what is returned by the lambda expressions to a name (adder_lambda, to_upper_lambda), but there is no need for that when you use lambdas in the way we did in the filter() example.

## Function attributes

Every function is a fully fledged object and, as such, it has many attributes. Some of them are special and can be used in an introspective way to inspect the function object at runtime. The following script is an example that shows a part of them and how to display their value for an example function:
```python

# func.attributes.py
def multiplication(a, b=1):
    """Return a multiplied by b. """
    return a * b

if __name__ == "__main__":
    special_attributes = [
        "__doc__", "__name__", "__qualname__", "__module__",
        "__defaults__", "__code__", "__globals__", "__dict__",
        "__closure__", "__annotations__", "__kwdefaults__",
    ]
    for attribute in special_attributes:
        print(attribute, '->', getattr(multiplication, attribute))

```

We used the built-in getattr() function to get the value of those attributes. getattr(obj, attribute) is equivalent to obj.attribute and comes in handy when we need to dynamically get an attribute at runtime, taking the name of the attribute from a variable (as in this example). Running this script yields:

```python
$ python func.attributes.py
__doc__ -> Return a multiplied by b. 
__name__ -> multiplication
__qualname__ -> multiplication
__module__ -> __main__
__defaults__ -> (1,)
__code__ -> <code object multiplication at 0x10fb599d0,
             file "func.attributes.py", line 2>
__globals__ -> {... omitted ...}
__dict__ -> {}
__closure__ -> None
__annotations__ -> {}
__kwdefaults__ -> None

```

We have omitted the value of the __globals__ attribute, as it was too big. An explanation of the meaning of this attribute can be found in the Callable types section of the Python Data Model documentation page:

https://docs.python.org/3/reference/datamodel.html#the-standard-type-hierarchy

Should you want to see all the attributes of an object, just call dir(object_name) and you will be given a list of all of its attributes.

## Built-in functions

Python comes with a lot of built-in functions. They are available anywhere, and you can get a list of them by inspecting the builtins module with dir(__builtins__), or by going to the official Python documentation. Unfortunately, we don't have the room to go through all of them here. We've already seen some of them, such as any, bin, bool, divmod, filter, float, getattr, id, int, len, list, min, print, set, tuple, type, and zip, but there are many more, which you should read about at least once. Get familiar with them, experiment, write a small piece of code for each of them, and make sure you have them at your fingertips so that you can use them when needed.

You can find a list of built-in functions in the official documentation, here: https://docs.python.org/3/library/functions.html.

## Documenting your code

We are big fans of code that doesn't need documentation. When we program correctly, choose the right names, and take care of the details, the code should come out as self-explanatory, with documentation being almost unnecessary. Sometimes a comment is very useful though, and so is some documentation. You can find the guidelines for documenting Python in PEP 257 -- Docstring conventions: https://www.python.org/dev/peps/pep-0257/, but we'll show you the basics here. Python is documented with strings, which are aptly called docstrings. Any object can be documented, and we can use either one-line or multi-line docstrings. One-liners are very simple. They should not provide another signature for the function, but instead state its purpose:

```python
# docstrings.py
def square(n):
    """Return the square of a number n. """
    return n ** 2

def get_username(userid):
    """Return the username of a user given their id. """
    return db.get(user_id=userid).username


```

Using triple double-quoted strings allows you to expand easily later on. Use sentences that end in a period, and don't leave blank lines before or after.

Multiline comments are structured in a similar way. There should be a one-liner that briefly gives you the gist of what the object is about, and then a more verbose description. As an example, we have documented a fictitious connect() function, using the Sphinx notation, in the following example:
```python

def connect(host, port, user, password):
    """Connect to a database.

    Connect to a PostgreSQL database directly, using the given
    parameters.

    :param host: The host IP.
    :param port: The desired port.
    :param user: The connection username.
    :param password: The connection password.
    :return: The connection object.
    """
    # body of the function here...
    return connection

```

Sphinx is one of the most widely used tools for creating Python documentation—in fact, the official Python documentation was written with it. It's definitely worth spending some time checking it out.

The help() built-in function, which is intended for interactive use, creates a documentation page for an object using its docstring.

## Importing objects

Now that we know a lot about functions, let's look at how to use them. The whole point of writing functions is to be able to reuse them later, and in Python, this translates to importing them into the namespace where you need them. There are many different ways to import objects into a namespace, but the most common ones are import module_name and from module_name import function_name. Of course, these are quite simplistic examples, but bear with us for the time being.

The import module_name form finds the module_name module and defines a name for it in the local namespace, where the import statement is executed. The from module_name import identifier form is a little bit more complicated than that but basically does the same thing. It finds module_name and searches for an attribute (or a submodule) and stores a reference to identifier in the local namespace. Both forms have the option to change the name of the imported object using the as clause:

```python
from mymodule import myfunc as better_named_func
```

Just to give you a flavor of what importing looks like, here's an example from a test module of one of Fabrizio's projects (notice that the blank lines between blocks of imports follow the guidelines from PEP 8 at https://www.python.org/dev/peps/pep-0008/#imports: standard library, third party, and local code):
```python

# imports.py
from datetime import datetime, timezone  # two imports on the same line
from unittest.mock import patch  # single import

import pytest  # third party library

from core.models import (  # multiline import
    Exam,
    Exercise,
    Solution,
)
```

When we have a structure of files starting in the root of our project, we can use the dot notation to get to the object we want to import into our current namespace, be it a package, a module, a class, a function, or anything else. 

The from module import syntax also allows a catch-all clause, from module import *, which is sometimes used to get all the names from a module into the current namespace at once; this is frowned upon for several reasons though, relating to performance and the risk of silently shadowing other names. You can read all that there is to know about imports in the official Python documentation but, before we leave the subject, let us give you a better example.

Imagine that we have defined a couple of functions, square(n) and cube(n), in a module, funcdef.py, which is in the lib folder. We want to use them in a couple of modules that are at the same level as the lib folder, called func_import.py and func_ from.py. Showing the tree structure of that project produces something like this:
```bash

├── func_from.py
├── func_import.py
├── lib
│   ├── __init__.py
│   └── funcdef.py

```

Before we show you the code of each module, please remember that in order to tell Python that it is actually a package, we need to put an __init__.py module in it.

There are two things to note about the __init__.py file. First of all, it is a fully fledged Python module so you can put code into it as you would with any other module. Second, as of Python 3.3, its presence is no longer required to make a folder be interpreted as a Python package.

The code is as follows:

```python
# lib/funcdef.py
def square(n):
    return n ** 2

def cube(n):
    return n ** 3

# func_import.py
import lib.funcdef
print(lib.funcdef.square(10))
print(lib.funcdef.cube(10))

# func_from.py
from lib.funcdef import square, cube
print(square(10))
print(cube(10))

```

Both these files, when executed, print 100 and 1000. You can see how differently we then access the square and cube functions, according to how and what we imported in the current scope.

## Relative imports

The type of import we've seen so far is called an absolute import; that is, it defines the whole path of either the module that we want to import or from which we want to import an object. There is another way of importing objects into Python, which is called a relative import. Relative imports are done by adding as many leading dots in front of the module as the number of folders we need to backtrack, in order to find what we're searching for. Simply put, it is something such as this:

    from .mymodule import myfunc

Relative imports are quite useful when restructuring projects. Not having the full path in the imports allows the developer to move things around without having to rename too many of those paths.

For a complete explanation of relative imports, refer to PEP 328: https://www.python.org/dev/peps/pep-0328/

In later chapters, we will create projects using different libraries and use several different types of imports, including relative ones, so make sure you take a bit of time to read up about them in the official Python documentation.

## One final example

Before we finish off this chapter, let's go through one last example. We could write a function to generate a list of prime numbers up to a limit; we've already seen the code for this in Chapter 3, so let's make it a function and, to keep it interesting, let's optimize it a bit. It turns out that we don't need to divide by all numbers from 2 to N-1 to decide whether a number, N, is prime. We can stop at √N (the square root of N). Moreover, we don't need to test the division for all numbers from 2 to √N, as we can just use the primes in that range. We leave it up to you to figure out why this works, if you're interested in the beauty of mathematics. 

Let's see how the code changes:
```python

# primes.py
from math import sqrt, ceil

def get_primes(n):
    """Calculate a list of primes up to n (included). """
    primelist = []
    for candidate in range(2, n + 1):
        is_prime = True
        root = ceil(sqrt(candidate))  # division limit
        for prime in primelist:  # we try only the primes
            if prime > root:  # no need to check any further
                break
            if candidate % prime == 0:
                is_prime = False
                break
        if is_prime:
            primelist.append(candidate)
    return primelist

```

The code is the same as that in the previous chapter. We have changed the division algorithm so that we only test divisibility using the previously calculated primes, and we stopped once the testing divisor was greater than the root of the candidate. We used the primelist result list to get the primes for the division and calculated the root value using a fancy formula, the integer value of the ceiling of the root of the candidate. While a simple int(k ** 0.5) + 1 would have also served our purpose, the formula we chose is cleaner and requires a couple of imports, which is what we wanted to show. Check out the functions in the math module—they are very interesting!

## Summary

In this chapter, we explored the world of functions. They are very important and, from now on, we'll use them in virtually everything we do. We talked about the main reasons for using them, the most important of which are code reuse and implementation hiding.

We saw that a function object is like a box that takes optional inputs and may produce outputs. We can feed input arguments to a function in many different ways, using positional and keyword arguments, and using variable syntax for both types.

You should now know how to write a function, document it, import it into your code, and call it.

In the next chapter we will be picking up the pace a little a bit, so we suggest you take any opportunity you get to consolidate and enrich the knowledge you've gathered so far by putting your nose into the Python official documentation.
