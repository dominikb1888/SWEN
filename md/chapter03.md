# Conditionals and Iteration

    "Would you tell me, please, which way I ought to go from here?"
    "That depends a good deal on where you want to get to."
    —Lewis Carroll, from Alice's Adventures in Wonderland

In the previous chapter, we looked at Python's built-in data types. Now that you are familiar with data in its many forms and shapes, it's time to start looking at how a program can use it.

According to Wikipedia:

    In computer science, control flow (or flow of control) is the order in which individual statements, instructions or function calls of an imperative program are executed or evaluated.

In order to control the flow of a program, we have two main weapons: conditional programming (also known as branching) and looping. We can use them in many different combinations and variations, but in this chapter, instead of going through all the possible forms of those two constructs in a documentation fashion, we'd rather give you the basics, and then write a couple of small scripts with you. In the first one, we will see how to create a rudimentary prime number generator, while in the second, we'll see how to apply discounts to customers based on coupons. This way, you should get a better feeling for how conditional programming and looping can be used.

In this chapter, we are going to cover the following:

---
## Agenda

-  Recap: Dict and Choosing Data Types
-  Conditional programming
-  Looping in Python
-  Assignment expressions
-  A quick peek at the itertools module


---

## TOC

- [Conditionals and Iteration](#conditionals-and-iteration)
  - [Agenda](#agenda)
  - [TOC](#toc)
  - [Conditional Programming](#conditional-programming)
    - [A specialized else: elif](#a-specialized-else-elif)
    - [The ternary operator](#the-ternary-operator)
  - [Looping](#looping)
    - [The for loop](#the-for-loop)
      - [Iterating over a range](#iterating-over-a-range)
      - [Iterating over a sequence](#iterating-over-a-sequence)
    - [Iterators and iterables](#iterators-and-iterables)
      - [Iterating over multiple sequences](#iterating-over-multiple-sequences)
    - [The while loop](#the-while-loop)
      - [The break and continue statements](#the-break-and-continue-statements)
    - [A special else clause](#a-special-else-clause)
  - [Assignment expressions](#assignment-expressions)
    - [Statements and expressions](#statements-and-expressions)
    - [Using the walrus operator](#using-the-walrus-operator)
    - [A word of warning](#a-word-of-warning)
    - [Putting all this together](#putting-all-this-together)
  - [A prime generator](#a-prime-generator)
  - [Applying discounts](#applying-discounts)
  - [A quick peek at the itertools module](#a-quick-peek-at-the-itertools-module)
    - [Infinite iterators](#infinite-iterators)
  - [Combinatoric generators](#combinatoric-generators)
  - [Summary](#summary)
  - [Further Reading](#further-reading)

---
## Conditional Programming

Conditional programming, or branching, is something you do every day, every moment. It's about evaluating conditions: if the light is green, then I can cross; if it's raining, then I'm taking the umbrella; and if I'm late for work, then I'll call my manager. The main tool is the if statement, which comes in different forms and colors, but its basic function is to evaluate an expression and, based on the result, choose which part of the code to execute. As usual, let's look at an example:

```python
# conditional.1.py
late = True 
if late: 
    print('I need to call my manager!') 
```

This is possibly the simplest example: when fed to the if statement, late acts as a conditional expression, which is evaluated in a Boolean context (exactly like if we were calling bool(late)). If the result of the evaluation is True, then we enter the body of the code immediately after the if statement. Notice that the print instruction is indented, which means that it belongs to a scope defined by the if clause. Execution of this code yields:

```bash
$ python conditional.1.py
I need to call my manager!
```

Since late is True, the print() statement was executed. Let's expand on this example:

```python
# conditional.2.py
late = False 
if late: 
    print('I need to call my manager!')  #1 
else: 
    print('no need to call my manager...')  #2 
``````

This time we set late = False, so when we execute the code, the result is different:

```python
$ python conditional.2.py
no need to call my manager...
```

Depending on the result of evaluating the late expression, we can either enter block #1 or block #2, but not both. Block #1 is executed when late evaluates to True, while block #2 is executed when late evaluates to False. Try assigning False/ True values to the late name, and see how the output for this code changes accordingly.

The preceding example also introduces the else clause, which becomes very handy when we want to provide an alternative set of instructions to be executed when an expression evaluates to False within an if clause. The else clause is optional, as is evident by comparing the preceding two examples.

### A specialized else: elif

Sometimes all you need is to do something if a condition is met (a simple if clause). At other times, you need to provide an alternative, in case the condition is False (if/ else clause). But there are situations where you may have more than two paths to choose from; since calling the manager (or not calling them) is a type of binary example (either you call or you don't), let's change the type of example and keep expanding. This time, we decide on tax percentages. If your income is less than $10,000, you don't need to pay any taxes. If it is between $10,000 and $30,000, you have to pay 20% in taxes. If it is between $30,000 and $100,000, you pay 35% in taxes, and if you're fortunate enough to earn over $100,000, you must pay 45% in taxes. Let's put this all down into beautiful Python code:


```python
# taxes.py
income = 15000 
if income < 10000: 
    tax_coefficient = 0.0  #1 
elif income < 30000: 
    tax_coefficient = 0.2  #2 
elif income < 100000: 
    tax_coefficient = 0.35  #3 
else: 
    tax_coefficient = 0.45  #4 
 
print(f'You will pay: ${income * tax_coefficient} in taxes') 
```

Executing the preceding code yields:

```python
$ python taxes.py
You will pay: $3000.0 in taxes
```

Let's go through the example one line at a time. We start by setting up the income value. In the example, your income is $15,000. We enter the if clause. Notice that this time we also introduced the elif clause, which is a contraction of else-if, and it's different from a bare else clause in that it also has its own condition. So, the if expression of income < 10000 evaluates to False, therefore block #1 is not executed.

The control passes to the next condition evaluator: elif income < 30000. This one evaluates to True, therefore block #2 is executed, and because of this, Python then resumes execution after the whole if/elif/elif/else clause (which we can just call the if clause from now on). There is only one instruction after the if clause, the print() call, which tells us that you will pay $3000.0 in taxes this year (15,000 * 20%). Notice that the order is mandatory: if comes first, then (optionally) as many elif 
clauses as you may need, and then (optionally) a single else clause.

Interesting, right? No matter how many lines of code you may have within each block, when one of the conditions evaluates to True, the associated block is executed, and then execution resumes after the whole clause. If none of the conditions evaluates to True (for example, income = 200000), then the body of the else clause would be executed (block #4). This example expands our understanding of the behavior of the else clause. Its block of code is executed when none of the preceding if/elif/.../elif expressions has evaluated to True.

Try to modify the value of income until you can comfortably execute all blocks at will (one per execution, of course). And then try the boundaries. This is crucial, as whenever you have conditions expressed as equalities or inequalities (==, !=, <, >, <=, >=), those numbers represent boundaries. It is essential to test boundaries thoroughly. Should we allow you to drive at 18 or 17? Are we checking your age with age < 18, or age <= 18? You can't imagine how many times we've had to fix subtle bugs that stemmed from using the wrong operator, so go ahead and experiment with the preceding code. Change some < to <= and set income to be one of the boundary values (10,000, 30,000, 100,000) as well as any value in between. See how the result changes, and get a good understanding of it before proceeding.

Let's now see another example that shows us how to nest if clauses. Say your program encounters an error. If the alert system is the console, we print the error. If the alert system is an email, we send it according to the severity of the error. If the alert system is anything other than console or email, we don't know what to do, therefore we do nothing. Let's put this into code:

```python
# errorsalert.py
alert_system = 'console'  # other value can be 'email' 
error_severity = 'critical'  # other values: 'medium' or 'low' 
error_message = 'OMG! Something terrible happened!' 
 
if alert_system == 'console': 
    print(error_message)  #1 
elif alert_system == 'email': 
    if error_severity == 'critical': 
        send_email('admin@example.com', error_message)  #2 
    elif error_severity == 'medium': 
        send_email('support.1@example.com', error_message)  #3 
    else: 
        send_email('support.2@example.com', error_message)  #4 
```

The preceding example is quite interesting because of how silly it is. It shows us two nested if clauses (outer and inner). It also shows us that the outer if clause doesn't have any else, while the inner one does. Notice how indentation is what allows us to nest one clause within another.

If alert_system == 'console', body #1 is executed, and nothing else happens. On the other hand, if alert_system == 'email', then we enter into another if clause, which we call the inner clause. In the inner if clause, according to error_severity, we send an email either to an admin, first-level support, or second-level support (blocks #2, #3, and #4). The send_email() function is not defined in this example, therefore trying to run it would give you an error. In the source code of this book, we included a trick to redirect that call to a regular print() function, just so you can experiment on the console without actually sending an email. Try changing the values and see how it all works.


### The ternary operator

One last thing we would like to show you, before moving on, is the ternary operator or, in layman's terms, the short version of an if/else clause. When the value of a name is to be assigned according to some condition, it is sometimes easier and more readable to use the ternary operator instead of a proper if clause. For example, instead of:

````python``````
# ternary.py
order_total = 247  # GBP 
 
# classic if/else form 
if order_total > 100: 
    discount = 25  # GBP 
else: 
    discount = 0  # GBP 
print(order_total, discount) 

We could write:

# ternary.py
# ternary operator 
discount = 25 if order_total > 100 else 0 
print(order_total, discount) 
```

For simple cases like this, we find it very convenient to be able to express that logic in one line instead of four. Remember, as a coder, you spend much more time reading code than writing it, so Python's conciseness is invaluable.

In some languages (like C or JavaScript) the ternary operator is 
even more concise. For example, the above could be written as:

`discount = order_total > 100 ? 25 : 0;`

Although Python's version is slightly more verbose, we think it 
more than makes up for that by being much easier to read and 
understand.

Are you clear on how the ternary operator works? Basically, name = something if condition else something-else. So name is assigned something if condition evaluates to True, and something-else if condition evaluates to False.

Now that you know everything about controlling the path of the code, let's move on to the next subject: looping.


## Looping

If you have any experience with looping in other programming languages, you will find Python's way of looping a bit different. First of all, what is looping? Looping means being able to repeat the execution of a code block more than once, according to the loop parameters given. There are different looping constructs, which serve different purposes, and Python has distilled all of them down to just two, which you can use to achieve everything you need. These are the for and while statements.

While it's definitely possible to do everything you need using either of them, they do serve different purposes, and therefore they're usually used in different contexts. We'll explore this difference thoroughly in this chapter.


### The for loop

The for loop is used when looping over a sequence, such as a list, tuple, or collection of objects. Let's start with a simple example and expand on the concept to see what the Python syntax allows us to do:

```python
# simple.for.py
for number in [0, 1, 2, 3, 4]: 
    print(number) 
```

This simple snippet of code, when executed, prints all numbers from 0 to 4. The for loop is fed the list [0, 1, 2, 3, 4] and, at each iteration number, is given a value from the sequence (which is iterated sequentially in the order given), then the body of the loop is executed (the print() line). The number value changes at every iteration, according to which value is coming next from the sequence. When the sequence is exhausted, the for loop terminates, and the execution of the code resumes normally with the code after the loop.


#### Iterating over a range

Sometimes we need to iterate over a range of numbers, and it would be quite unpleasant to have to do so by hardcoding the list somewhere. In such cases, the range() function comes to the rescue. Let's see the equivalent of the previous snippet of code:

```python
# simple.for.py
for number in range(5): 
    print(number) 
```

The range() function is used extensively in Python programs when it comes to creating sequences; you can call it by passing one value, which acts as stop (counting from 0), or you can pass two values (start and stop), or even three (start, stop, and step). Check out the following example:

```python
>>> list(range(10))  # one value: from 0 to value (excluded)
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
>>> list(range(3, 8))  # two values: from start to stop (excluded)
[3, 4, 5, 6, 7]
>>> list(range(-10, 10, 4))  # three values: step is added
[-10, -6, -2, 2, 6]
```

For the moment, ignore that we need to wrap range(...) within a list. The range() object is a little bit special, but in this case, we are only interested in understanding what values it will return to us. You can see that the behavior is the same as with slicing (which we described in the previous chapter): start is included, stop is excluded, and optionally you can add a step parameter, which by default is 1.

Try modifying the parameters of the range() call in our simple.for.py code and see what it prints. Get comfortable with it.


#### Iterating over a sequence

We now have all the tools to iterate over a sequence, so let's build on that example:

```python
# simple.for.2.py
surnames = ['Rivest', 'Shamir', 'Adleman'] 
for position in range(len(surnames)): 
    print(position, surnames[position]) 
```

The preceding code adds a little bit of complexity to the game. Execution will show this result:

```bash
$ python simple.for.2.py
0 Rivest
1 Shamir
2 Adleman
```

Let's use the inside-out technique to break it down. We start from the innermost part of what we're trying to understand, and we expand outward. So, len(surnames) is the length of the surnames list: 3. Therefore, range(len(surnames)) is actually transformed into range(3). This gives us the range [0, 3), which is basically the sequence (0, 1, 2). This means that the for loop will run three iterations. In the first one, position will take value 0, while in the second one, it will take value 1, and value 2 in the third and final iteration. What is (0, 1, 2), if not the possible indexing positions for the surnames list? At position 0, we find 'Rivest'; at position 1, 'Shamir'; and at position 2, 'Adleman'. If you are curious about what these three men created together, change print(position, surnames[position]) to print(surnames, position][0], end=''), add a final print() outside of the loop, and run the code again.

Now, this style of looping is actually much closer to languages such as Java or C. In Python, it's quite rare to see code like this. You can just iterate over any sequence or collection, so there is no need to get the list of positions and retrieve elements out of a sequence at each iteration. Let's change the example into a more Pythonic form:

```python
# simple.for.3.py
surnames = ['Rivest', 'Shamir', 'Adleman'] 
for surname in surnames: 
    print(surname) 
```

Now that's something! It's practically English. The for loop can iterate over the surnames list, and it gives back each element in order at each iteration. Running this code will print the three surnames, one at a time, which is much easier to read—right?

What if you wanted to print the position as well, though? Or what if you actually needed it? Should you go back to the range(len(...)) form? No. You can use the enumerate() built-in function, like this:

```python
# simple.for.4.py
surnames = ['Rivest', 'Shamir', 'Adleman'] 
for position, surname in enumerate(surnames): 
    print(position, surname) 
```

This code is very interesting as well. Notice that enumerate() gives back a two-tuple (position, surname) at each iteration, but still, it's much more readable (and more efficient) than the range(len(...)) example. You can call enumerate() with a start parameter, such as enumerate(iterable, start), and it will start from start, rather than 0. Just another little thing that shows you how much thought has been given to designing Python so that it makes your life easier.

You can use a for loop to iterate over lists, tuples, and in general anything that Python calls iterable. This is a very important concept, so let's talk about it a bit more.


### Iterators and iterables

According to the Python documentation (https://docs.python.org/3/glossary.html), an iterable is:

    An object capable of returning its members one at a time. Examples of iterables include all sequence types (such as list, str, and tuple) and some non-sequence types like dict, file objects, and objects of any classes you define with an __iter__() method or with a __getitem__() method that implements Sequence semantics.

Iterables can be used in a for loop and in many other places where a sequence is needed (zip(), map(), ...). When an iterable object is passed as an argument to the built-in function iter(), it returns an iterator for the object. This iterator is good for one pass over the set of values. When using iterables, it is usually not necessary to call iter() or deal with iterator objects yourself. The for statement does that automatically for you, creating a temporary unnamed variable to hold the iterator for the duration of the loop.

Simply put, what happens when you write for k in sequence: ... body ... is that the for loop asks sequence for the next element, it gets something back, it calls that something k, and then executes its body. Then, once again, the for loop asks sequence for the next element, it calls it k again, and executes the body again, and so on and so forth, until the sequence is exhausted. Empty sequences will result in zero executions of the body.

Some data structures, when iterated over, produce their elements in order, such as lists, tuples, dictionaries, and strings, while others, such as sets, do not. Python gives us the ability to iterate over iterables, using a type of object called an iterator. According to the official documentation, an iterator is:

An object representing a stream of data. Repeated calls to the iterator's __next__() method (or passing it to the built-in function next()) return successive items in the stream. When no more data are available a StopIteration exception is raised instead. At this point, the iterator object is exhausted and any further calls to its __next__() method just raise StopIteration again. Iterators are required to have an __iter__() method that returns the iterator object itself so every iterator is also iterable and may be used in most places where other iterables are accepted. One notable exception is code which attempts multiple iteration passes. A container object (such as a list) produces a fresh new iterator each time you pass it to the iter() function or use it in a for loop. Attempting this with an iterator will just return the same exhausted iterator object used in the previous iteration pass, making it appear like an empty container.

Don't worry if you do not fully understand all the preceding legalese, as you will in due course. We have put it here to serve as a handy reference for the future.

In practice, the whole iterable/iterator mechanism is somewhat hidden behind the code. Unless you need to code your own iterable or iterator for some reason, you won't have to worry about this too much. But it's very important to understand how Python handles this key aspect of control flow, because it will shape the way in which you write code.

#### Iterating over multiple sequences

Let's see another example of how to iterate over two sequences of the same length, in order to work on their respective elements in pairs. Say we have a list of people and a list of numbers representing the age of the people in the first list. We want to print the pair person/age on one line for each of them. Let's start with an example, which we will refine gradually:

```python
# multiple.sequences.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
for position in range(len(people)):
    person = people[position]
    age = ages[position]
    print(person, age)
```

By now, this code should be pretty straightforward for you to understand. We need to iterate over the list of positions (0, 1, 2, 3) because we want to retrieve elements from two different lists. Executing it, we get the following:
```bash

$ python multiple.sequences.py
Nick 23
Rick 24
Roger 23
Syd 21

```

The code works, but it's not very Pythonic. It's rather cumbersome to have to get the length of people, construct a range, and then iterate over that. For some data structures it may also be expensive to retrieve items by their position. Wouldn't it be nice if we could use the same approach as for iterating over a single sequence? Let's try to improve it by using enumerate():

```python

# multiple.sequences.enumerate.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
for position, person in enumerate(people):
    age = ages[position]
    print(person, age)

```

That's better, but still not perfect. And it's still a bit ugly. We're iterating properly on people, but we're still fetching age using positional indexing, which we want to lose as well. 

Well, no worries, Python gives you the zip() function, remember? Let's use it:

```python

# multiple.sequences.zip.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
for person, age in zip(people, ages):
    print(person, age)

```

Ah! So much better! Once again, compare the preceding code with the first example and admire Python's elegance. The reason we wanted to show this example is twofold. On the one hand, we wanted to give you an idea of how much shorter code in Python can be compared to other languages where the syntax doesn't allow you to iterate over sequences or collections as easily. And on the other hand, and much more importantly, notice that when the for loop asks zip(sequenceA, sequenceB) for the next element, it gets back a tuple, not just a single object. It gets back a tuple with as many elements as the number of sequences we feed to the zip() function. Let's expand a little on the previous example in two ways, using explicit and implicit assignment:

```python
# multiple.sequences.explicit.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
instruments = ['Drums', 'Keyboards', 'Bass', 'Guitar']
for person, age, instrument in zip(people, ages, instruments):
    print(person, age, instrument)

```

In the preceding code we added the instruments list. Now that we feed three sequences to the zip() function, the for loop gets back a three-tuple at each iteration. Notice that the position of the elements in the tuple respects the position of the sequences in the zip() call. Executing the code will yield the following result:

```bash
$ python multiple.sequences.explicit.py
Nick 23 Drums
Rick 24 Keyboards
Roger 23 Bass
Syd 21 Guitar

```

Sometimes, for reasons that may not be clear in a simple example such as the preceding one, you may want to explode the tuple within the body of the for loop. If that is your desire, it's perfectly possible to do so:

```python
# multiple.sequences.implicit.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
instruments = ['Drums', 'Keyboards', 'Bass', 'Guitar']
for data in zip(people, ages, instruments):
    person, age, instrument = data
    print(person, age, instrument)

```

It's basically doing what the for loop does automatically for you, but in some cases you may want to do it yourself. Here, the three-tuple data that comes from zip(...) is exploded within the body of the for loop into three variables: person, age, and instrument.

### The while loop

In the preceding pages, we saw the for loop in action. It's incredibly useful when you need to loop over a sequence or a collection. The key point to keep in mind, when you need to decide which looping construct to use, is that the for loop is best suited in cases where you need to iterate over the elements of some container or other iterable object.

There are other cases though, when you just need to loop until some condition is satisfied, or even loop indefinitely until the application is stopped, such as cases where we don't really have something to iterate on, and therefore the for loop would be a poor choice. But fear not, for these cases, Python provides us with the while loop.

The while loop is similar to the for loop in that they both loop, and at each iteration they execute a body of instructions. The difference is that the while loop doesn't loop over a sequence (it can, but you have to write the logic manually, which would make little sense as you would just use a for loop); rather, it loops as long as a certain condition is satisfied. When the condition is no longer satisfied, the loop ends.

As usual, let's see an example that will clarify everything for us. We want to print the binary representation of a positive number. In order to do so, we can use a simple algorithm that divides by 2 until we reach 0 and collects the remainders. When we reverse the list of remainders we collected, we get the binary representation of the number we started with:

6 / 2 = 3 (remainder: 0) 
3 / 2 = 1 (remainder: 1) 
1 / 2 = 0 (remainder: 1) 
List of remainders: 0, 1, 1. 

Reversed is 1, 1, 0, which is also the binary representation of 6: 110

Let's write some code to calculate the binary representation for the number 39, 1001112:

```python
# binary.py
n = 39
remainders = []
while n > 0:
    remainder = n % 2  # remainder of division by 2
    remainders.append(remainder)  # we keep track of remainders
    n //= 2  # we divide n by 2

remainders.reverse()
print(remainders)

```

In the preceding code, we highlighted n > 0, which is the condition to keep looping. Notice how the code matches the algorithm we described: as long as n is greater than 0, we divide by 2 and add the remainder to a list. At the end (when n has reached 0) we reverse the list of remainders to get the binary representation of the original value of n.

We can make the code a little shorter (and more Pythonic), by using the divmod() function, which is called with a number and a divisor, and returns a tuple with the result of the integer division and its remainder. For example, divmod(13, 5) would return (2, 3), and indeed 5 * 2 + 3 = 13:

```python
# binary.2.py
n = 39
remainders = []
while n > 0:
    n, remainder = divmod(n, 2)
    remainders.append(remainder)

remainders.reverse()
print(remainders)
```

In the preceding code, we have reassigned n to the result of the division by 2, along with the remainder, in one single line. Notice that the condition in a while loop is a condition to continue looping. If it evaluates to True, then the body is executed and then another evaluation follows, and so on, until the condition evaluates to False. When that happens, the loop is exited immediately without executing its body.

If the condition never evaluates to False, the loop becomes a so-called infinite loop. Infinite loops are used, for example, when polling from network devices: you ask the socket whether there is any data, you do something with it if there is any, then you sleep for a small amount of time, and then you ask the socket again, over and over again, without ever stopping.

Having the ability to loop over a condition, or to loop indefinitely, is the reason why the for loop alone is not enough, and therefore Python provides the while loop.

By the way, if you need the binary representation of a number, 
check out the bin() function.

Just for fun, let's adapt one of the examples (multiple.sequences.py) using the while logic:

```python
# multiple.sequences.while.py
people = ['Nick', 'Rick', 'Roger', 'Syd']
ages = [23, 24, 23, 21]
position = 0
while position < len(people):
    person = people[position]
    age = ages[position]
    print(person, age)
    position += 1

```

In the preceding code, we have highlighted the initialization, condition, and update of the position variable, which makes it possible to simulate the equivalent for loop code by handling the iteration variable manually. Everything that can be done with a for loop can also be done with a while loop, even though you can see there is a bit of boilerplate you have to go through in order to achieve the same result. The opposite is also true, but unless you have a reason to do so, you ought to use the 
right tool for the job, and 99.9% of the time you'll be fine.

So, to recap, use a for loop when you need to iterate over an iterable, and a while loop when you need to loop according to a condition being satisfied or not. If you keep in mind the difference between the two purposes, you will never choose the wrong looping construct.

Let us now see how to alter the normal flow of a loop.


#### The break and continue statements

According to the task at hand, sometimes you will need to alter the regular flow of a loop. You can either skip a single iteration (as many times as you want), or you can break out of the loop entirely. A common use case for skipping iterations is, for example, when you are iterating over a list of items and you need to work on each of them only if some condition is verified. On the other hand, if you're iterating over a collection of items, and you have found one of them that satisfies some need you have, you may decide not to continue the loop entirely and therefore break out of it. There are countless possible scenarios, so it's better to take a look at a couple of examples.

Let's say you want to apply a 20% discount to all products in a basket list for those that have an expiration date of today. The way you achieve this is to use the continue statement, which tells the looping construct (for or while) to stop execution of the body immediately and go to the next iteration, if any. This example will take us a little deeper down the rabbit hole, so be ready to jump:

```python
# discount.py
from datetime import date, timedelta

today = date.today()
tomorrow = today + timedelta(days=1)  # today + 1 day is tomorrow
products = [
    {'sku': '1', 'expiration_date': today, 'price': 100.0},
    {'sku': '2', 'expiration_date': tomorrow, 'price': 50},
    {'sku': '3', 'expiration_date': today, 'price': 20},
]

for product in products:
    if product['expiration_date'] != today:
        continue
    product['price'] *= 0.8  # equivalent to applying 20% discount
    print(
        'Price for sku', product['sku'],
        'is now', product['price'])

```

We start by importing the date and timedelta objects, then we set up our products. Those with sku as 1 and 3 have an expiration date of today, which means we want to apply a 20% discount on them. We loop over each product and inspect the expiration date. If it is not (inequality operator, !=) today, we don't want to execute the rest of the body suite, so we continue.

Notice that it is not important where in the body suite you place the continue statement (you can even use it more than once). When you reach it, execution stops and goes back to the next iteration. If we run the discount.py module, this is the output:

```bash
$ python discount.py
Price for sku 1 is now 80.0
Price for sku 3 is now 16.0

```

This shows you that the last two lines of the body haven't been executed for sku number 2.

Let's now see an example of breaking out of a loop. Say we want to tell whether at least one of the elements in a list evaluates to True when fed to the bool() function. Given that we need to know whether there is at least one, when we find it, we don't need to keep scanning the list any further. In Python code, this translates to using the break statement. Let's write this down into code:

```python
# any.py
items = [0, None, 0.0, True, 0, 7]  # True and 7 evaluate to True

found = False  # this is called "flag"
for item in items:
    print('scanning item', item)
    if item:
        found = True  # we update the flag
        break

if found:  # we inspect the flag
    print('At least one item evaluates to True')
else:
    print('All items evaluate to False')
```

The preceding code makes use of a very common programming pattern; you set up a flag variable before starting the inspection of the items. If you find an element that matches your criteria (in this example, that evaluates to True), you update the flag and stop iterating. After iteration, you inspect the flag and take action accordingly. Execution yields:

```bash
$ python any.py
scanning item 0
scanning item None
scanning item 0.0
scanning item True
At least one item evaluates to True

```

See how execution stopped after True was found? The break statement acts exactly like the continue one, in that it stops executing the body of the loop immediately, but it also prevents any further iterations from running, effectively breaking out of the loop. The continue and break statements can be used together with no limitation in their numbers, both in the for and while looping constructs.

There is no need to write code to detect whether there is at least one element in a sequence that evaluates to True. Just check out the built-in any() function.

#### A special else clause

One of the features we've seen only in the Python language is the ability to have else clauses after while and for loops. It's very rarely used, but it's definitely useful to have. In short, you can have an else suite after a for or while loop. If the loop ends normally, because of exhaustion of the iterator (for loop) or because the condition is finally not met (while loop), then the else suite (if present) is executed. If execution is interrupted by a break statement, the else clause is not executed. 

Let's take an example of a for loop that iterates over a group of items, looking for one that would match some condition. If we don't find at least one that satisfies the condition, we want to raise an exception. This means that we want to arrest the regular execution of the program and signal that there was an error, or exception, that we cannot deal with. Exceptions will be the subject of Chapter 7, Exceptions and Context Managers, so don't worry if you don't fully understand them for now. Just bear in mind that they will alter the regular flow of the code.

Let us now show you two examples that do exactly the same thing, but one of them is using the special for...else syntax. Say that we want to find, among a collection of people, one that could drive a car:

```python
# for.no.else.py
class DriverException(Exception):
    pass

people = [('James', 17), ('Kirk', 9), ('Lars', 13), ('Robert', 8)]
driver = None
for person, age in people:
    if age >= 18:
        driver = (person, age)
        break

if driver is None:
    raise DriverException('Driver not found.')

```

Notice the flag pattern again. We set the driver to be None, then if we find one, we update the driver flag, and then, at the end of the loop, we inspect it to see whether one was found. We kind of have the feeling that those kids would drive a very metallic-a car, but anyway, notice that if a driver is not found, DriverException is raised, signaling to the program that execution cannot continue (we're lacking the driver).

The same functionality can be rewritten a bit more elegantly using the following code:

```python
# for.else.py
class DriverException(Exception):
    pass

people = [('James', 17), ('Kirk', 9), ('Lars', 13), ('Robert', 8)]
for person, age in people:
    if age >= 18:
        driver = (person, age)
        break
else:
    raise DriverException('Driver not found.')

```

Notice that we are not forced to use the flag pattern any more. The exception is raised as part of the for loop logic, which makes good sense, because the for loop is checking on some condition. All we need is to set up a driver object in case we find one, because the rest of the code is going to use that information somewhere. Notice the code is shorter and more elegant, because the logic is now correctly grouped together where it belongs.

In his Transforming Code into Beautiful, Idiomatic Python video, Raymond Hettinger suggests a much better name for the else statement associated with a for loop: nobreak. If you struggle with remembering how the else works for a for loop, simply remembering this fact should help you.

---
## Assignment expressions

Before we look at some more complicated examples, we would like to briefly introduce you to a relatively new feature that was added to the language in Python 3.8, via PEP 572 (https://www.python.org/dev/peps/pep-0572). Assignment expressions allow us to bind a value to a name in places where normal assignment statements are not allowed. Instead of the normal assignment operator =, assignment expressions use := (known as the walrus operator because it resembles the eyes and tusks of a walrus).


### Statements and expressions

To understand the difference between normal assignments and assignment expressions, we need to understand the difference between statements and expressions. According to the Python documentation (https://docs.python.org/3/glossary.html), a statement is:

    …part of a suite (a "block" of code). A statement is either an expression or one of several constructs with a keyword, such as if, while or for.

An expression, on the other hand, is:

    A piece of syntax which can be evaluated to some value. In other words, an expression is an accumulation of expression elements like literals, names, attribute access, operators or function calls which all return a value.

The key distinguishing feature of an expression is that it has a return value. Notice that an expression can be a statement, but not all statements are expressions. In particular, assignments like name = "heinrich" are not expressions, and so do not have a return value. This means that you cannot use an assignment statement in the conditional expression of a while loop or if statement (or any other place where a value is required).

Have you ever wondered why the Python console doesn't print a value when you assign a value to a name? For example:
```python

>>> name = "heinrich"
>>>

```

Well, now you know! It's because what you've entered is a 
statement, which doesn't have a return value to print.

### Using the walrus operator

Without assignment expressions, you would have to use two separate statements if you want to bind a value to a name and use that value in an expression. For example, it is quite common to see code similar to:

```python
# walrus.if.py
remainder = value % modulus
if remainder:
    print(f"Not divisible! The remainder is {remainder}.")

```

With assignment expressions, we could rewrite this as: 

```python
# walrus.if.py
if remainder := value % modulus:
    print(f"Not divisible! The remainder is {remainder}.")

```

Assignment expressions allow us to write fewer lines of code. Used with care, they can also lead to cleaner, more understandable code. Let's look at a slightly bigger example to see how an assignment expression can really simplify a while loop.

In interactive scripts, we often need to ask the user to choose between a number of options. For example, suppose we are writing an interactive script that allows customers at an ice cream shop to choose what flavor they want. To avoid confusion when preparing orders, we want to ensure that the user chooses one of the available flavors. Without assignment expressions, we might write something like this:

```python
# menu.no.walrus.py
flavors = ["pistachio", "malaga", "vanilla", "chocolate", "strawberry"]
prompt = "Choose your flavor: "
print(flavors)
while True:
    choice = input(prompt)
    if choice in flavors:
        break
    print(f"Sorry, '{choice}' is not a valid option.")
print(f"You chose '{choice}'.")

```

Take a moment to read this code carefully. Notice the condition on that loop: while True means "loop forever." That's not what we really want, is it? We want to stop looping when the user inputs a valid flavor (choice in flavors). To achieve that, we've used an if statement and a break inside the loop. The logic to control the loop is not immediately obvious. In spite of that, this is actually quite a common pattern when the value needed to control the loop can only be obtained inside the loop. The input() function is very useful in interactive scripts. It allows you to prompt the user for input, and returns it as a string. You can read more about it in the official Python documentation.

How can we improve on this? Let us try to use an assignment expression:

```python

# menu.walrus.py
flavors = ["pistachio", "malaga", "vanilla", "chocolate", "strawberry"]
prompt = "Choose your flavor: "
print(flavors)
while (choice := input(prompt)) not in flavors:
    print(f"Sorry, '{choice}' is not a valid option.")
print(f"You chose '{choice}'.")

```

Now the loop's conditional expression says exactly what we want. That is much easier to understand. The code is also three lines shorter. 

Did you notice the parentheses around the assignment expression? We need them because the := operator has lower precedence than the not in operator. Try removing the parentheses and see what happens.

We have seen examples of using assignment expressions in the conditional expressions of if and while statements. Besides these use cases, assignment expressions are also very useful in lambda expressions (which you will meet in Chapter 4, Functions, the Building Blocks of Code) as well as comprehensions and generators (which you will learn about in Chapter 5, Comprehensions and Generators).

### A word of warning

The introduction of the walrus operator in Python was somewhat controversial. Some people feared that it would make it too easy to write ugly, non-Pythonic code. We think that these fears are not entirely justified. As you saw above, the walrus operator can improve code and make it easier to read. Like any powerful feature, it can however be abused to write obfuscated code. We would advise you to use it sparingly. Always think carefully about how it impacts the readability of your code.


### Putting all this together

Now that you have seen all there is to see about conditionals and loops, it's time to spice things up a little, and look at those two examples we anticipated at the beginning of this chapter. We'll mix and match here, so you can see how you can use all these concepts together. Let's start by writing some code to generate a list of prime numbers up to some limit. Please bear in mind that we are going to write a very inefficient and rudimentary algorithm to detect primes. The important thing is to concentrate on those bits in the code that belong to this chapter's subject.

## A prime generator

According to Wikipedia:

    A prime number (or a prime) is a natural number greater than 1 that is not a product of two smaller natural numbers. A natural number greater than 1 that is not prime is called a composite number.

Based on this definition, if we consider the first 10 natural numbers, we can see that 2, 3, 5, and 7 are primes, while 1, 4, 6, 8, 9, and 10 are not. In order to have a computer tell you whether a number, N, is prime, you can divide that number by the natural numbers in the range [2, N). If any of those divisions yields zero as a remainder, then the number is not a prime. We will write two versions of this, the second of which will exploit the for...else syntax:

```python

# primes.py
primes = []  # this will contain the primes at the end
upto = 100  # the limit, inclusive
for n in range(2, upto + 1):
    is_prime = True  # flag, new at each iteration of outer for
    for divisor in range(2, n):
        if n % divisor == 0:
            is_prime = False
            break
    if is_prime:  # check on flag
        primes.append(n)
print(primes)

```

There are a lot of things to notice in the preceding code. First of all, we set up an empty primes list, which will contain the primes at the end. The limit is 100, and you can see that it is inclusive in the way we call range() in the outer loop. If we wrote range(2, upto) that would be [2, upto). Therefore range(2, upto + 1) gives us [2, upto + 1) = [2, upto]. So, there are two for loops. In the outer one, we loop over the candidate primes—that is, all natural numbers from 2 to upto. Inside each iteration of this outer loop, we set up a flag (which is set to True at each iteration), and then start dividing the current value of n by all numbers from 2 to n - 1. If we find a proper divisor for n, it means n is composite, and therefore we set the flag to False and break the loop. Notice that when we break the inner loop, the outer one keeps on going as normal. The reason why we break after having found a proper divisor for n is that we don't need any further information to be able to tell that n is not a prime.

When we check on the is_prime flag, if it is still True, it means we couldn't find any number in [2, n) that is a proper divisor for n, therefore n is a prime. We append n to the primes list, and hop! Another iteration proceeds, until n equals 100.

Running this code yields:

```bash
$ python primes.py
[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
67, 71, 73, 79, 83, 89, 97]
```
Before proceeding, we will pose the following: of all the iterations of the outer loop, one of them is different from all the others. Can you tell which one this is—and why? Think about it for a second, go back to the code, try to figure it out for yourself, and then keep reading on.

Did you figure it out? If not, don't feel bad; it's perfectly normal. We asked you to do it as a small exercise because this is what coders do all the time. The skill to understand what the code does by simply looking at it is something you build over time. It's very important, so try to exercise it whenever you can. We'll tell you the answer now: the iteration that behaves differently from all others is the first one. The reason is that in the first iteration, n is 2. Therefore the innermost for loop won't even run, because it's a for loop that iterates over range(2, 2), and what is that if not [2, 2)? Try it out for yourself, write a simple for loop with that iterable, put a print in the body suite, and see whether anything happens.

Now, from an algorithmic point of view, this code is inefficient; let's at least make it a bit easier on the eyes:

```python
# primes.else.py
primes = []
upto = 100
for n in range(2, upto + 1):
    for divisor in range(2, n):
        if n % divisor == 0:
            break
    else:
        primes.append(n)
print(primes)
```

Much better, right? The is_prime flag is gone, and we append n to the primes list when we know the inner for loop hasn't encountered any break statements. See how the code looks cleaner and reads better?

## Applying discounts

In this example, we want to show you a technique that we are very fond of. In many programming languages, besides the if/elif/else constructs, in whatever form or syntax they may come, you can find another statement, usually called switch/ case, that is not in Python. It is the equivalent of a cascade of if/elif/.../elif/ else clauses, with a syntax similar to this (warning, we are using JavaScript code here):

````javascript
/* switch.js */
switch (day_number) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
        day = "Weekday";
        break;
    case 6:
        day = "Saturday";
        break;
    case 0:
        day = "Sunday";
        break;
    default:
        day = "";
        alert(day_number + ' is not a valid day number.')
}

```

In the preceding code, we switch on a variable called day_number. This means we get its value and then decide what case it fits in (if any). From 1 to 5 there is a cascade, which means no matter the number, [1, 5] all go down to the bit of logic that sets day as "Weekday". Then we have single cases for 0 and 6, and a default case to prevent errors, which alerts the system that day_number is not a valid day number—that is, not in [0, 6]. Python is perfectly capable of realizing such logic using if/elif/else statements:

In the preceding code, we reproduce the same logic of the JavaScript snippet in Python using if/elif/else statements. We raised the ValueError exception just as an example at the end, if day_number is not in [0, 6]. This is one possible way of translating the switch/case logic, but there is also another one, sometimes called dispatching, which we will show you in the last version of the next example.

By the way, did you notice the first line of the previous snippet? Have you noticed that Python can make double (actually, even multiple) comparisons? It's just wonderful!

Let's start the new example by simply writing some code that assigns a discount to customers based on their coupon value. We'll keep the logic down to a minimum here—remember that all we really care about is understanding conditionals and loops:

```python
# coupons.py
customers = [
    dict(id=1, total=200, coupon_code='F20'),  # F20: fixed, £20
    dict(id=2, total=150, coupon_code='P30'),  # P30: percent, 30%
    dict(id=3, total=100, coupon_code='P50'),  # P50: percent, 50%
    dict(id=4, total=110, coupon_code='F15'),  # F15: fixed, £15
]

for customer in customers:
    code = customer['coupon_code']
    if code == 'F20':
        customer['discount'] = 20.0
    elif code == 'F15':
        customer['discount'] = 15.0
    elif code == 'P30':
        customer['discount'] = customer['total'] * 0.3
    elif code == 'P50':
        customer['discount'] = customer['total'] * 0.5
    else:
        customer['discount'] = 0.0

for customer in customers:
    print(customer['id'], customer['total'], customer['discount'])

```

We start by setting up some customers. They have an order total, a coupon code, and an ID. We made up four different types of coupons: two are fixed and two are percentage-based. You can see that in the if/elif/else cascade we apply the discount accordingly, and we set it as a 'discount' key in the customer dictionary.

At the end, we just print out part of the data to see whether our code is working properly:
```bash

$ python coupons.py
1 200 20.0
2 150 45.0
3 100 50.0
4 110 15.0

```

This code is simple to understand, but all those conditional clauses are cluttering the logic. It's not easy to see what's going on at a first glance, which we don't like. In cases like this, you can exploit a dictionary to your advantage, like this:

```python
# coupons.dict.py
customers = [
    dict(id=1, total=200, coupon_code='F20'),  # F20: fixed, £20
    dict(id=2, total=150, coupon_code='P30'),  # P30: percent, 30%
    dict(id=3, total=100, coupon_code='P50'),  # P50: percent, 50%
    dict(id=4, total=110, coupon_code='F15'),  # F15: fixed, £15
]
discounts = {
    'F20': (0.0, 20.0),  # each value is (percent, fixed)
    'P30': (0.3, 0.0),
    'P50': (0.5, 0.0),
    'F15': (0.0, 15.0),
}
for customer in customers:
    code = customer['coupon_code']
    percent, fixed = discounts.get(code, (0.0, 0.0))
    customer['discount'] = percent * customer['total'] + fixed

for customer in customers:
    print(customer['id'], customer['total'], customer['discount'])

```

Running the preceding code yields exactly the same result we had from the snippet before it. We spared two lines, but more importantly, we gained a lot in readability, as the body of the for loop is now just three lines long, and very easy to understand. The concept here is to use a dictionary as a dispatcher. In other words, we try to fetch something from the dictionary based on a code (our coupon_code), and by using dict.get(key, default), we make sure we also cater for when the code is not in the dictionary and we need a default value.

Notice that we had to apply some very simple linear algebra in order to calculate the discount properly. Each discount has a percentage and fixed part in the dictionary, represented by a two-tuple. By applying percent * total + fixed, we get the correct discount. When percent is 0, the formula just gives the fixed amount, and it gives percent * total when fixed is 0. 

This technique is important, because it is also used in other contexts with functions where it becomes much more powerful than what we've seen in the preceding example. Another advantage of using it is that you can code it in such a way that the keys and values of the discounts dictionary are fetched dynamically (for example, from a database). This will allow the code to adapt to whatever discounts and conditions you have, without having to modify anything.

If you are still unclear as to how this works, we suggest you take your time and experiment with it. Change values and add print() statements to see what's going on while the program is running.

## A quick peek at the itertools module

A chapter about iterables, iterators, conditional logic, and looping wouldn't be complete without a few words about the itertools module. If you are into iterating, this is a kind of heaven.

According to the Python official documentation (https://docs.python.org/3/ library/itertools.html), the itertools module:

    …implements a number of iterator building blocks inspired by constructs from APL, Haskell, and SML. Each has been recast in a form suitable for Python.

The module standardizes a core set of fast, memory efficient tools that are useful by themselves or in combination. Together, they form an "iterator algebra" making it possible to construct specialized tools succinctly and efficiently in pure Python.

By no means do we have the room here to show you all the goodies you can find in this module, so we encourage you to go check it out for yourself. We can promise that you will enjoy it, though. In a nutshell, it provides you with three broad categories of iterators. We shall give you a very small example of one iterator taken from each one of them, just to make your mouth water a little.

### Infinite iterators

Infinite iterators allow you to work with a for loop in a different fashion, such as if it were a while loop:
```python

# infinite.py
from itertools import count

for n in count(5, 3):
    if n > 20:
        break
    print(n, end=', ') # instead of newline, comma and space

```

Running the code outputs:
```bash

$ python infinite.py
5, 8, 11, 14, 17, 20,

```

The count factory class makes an iterator that simply goes on and on counting. It starts from 5 and keeps adding 3 to it. We need to break it manually if we don't want to get stuck in an infinite loop.

Iterators terminating on the shortest input sequence This category is very interesting. It allows you to create an iterator based on multiple iterators, combining their values according to some logic. The key point here is that among those iterators, if any of them are shorter than the rest, the resulting iterator won't break, but will simply stop as soon as the shortest iterator is exhausted. This is very theoretical, we know, so let us give you an example using compress(). This iterator gives you back the data according to a corresponding item in a selector being True or False; compress('ABC', (1, 0, 1)) would give back 'A' and 'C', because they correspond to 1. Let's see a simple example:

```python
# compress.py
from itertools import compress
data = range(10)
even_selector = [1, 0] * 10
odd_selector = [0, 1] * 10

even_numbers = list(compress(data, even_selector))
odd_numbers = list(compress(data, odd_selector))

print(odd_selector)
print(list(data))
print(even_numbers)
print(odd_numbers)

```

Notice that odd_selector and even_selector are 20 elements in length, while data is only 10. compress() will stop as soon as data has yielded its last element. Running this code produces the following:

```bash
$ python compress.py
[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
[0, 2, 4, 6, 8]
[1, 3, 5, 7, 9]

```

It's a very fast and convenient way of selecting elements out of an iterable. The code is very simple, but notice that instead of using a for loop to iterate over each value that is given back by the compress() calls, we used list(), which does the same, but instead of executing a body of instructions, it puts all the values into a list and returns it.


## Combinatoric generators

Last but not least is combinatoric generators. These are really fun, if you are into this kind of thing. Let's look at a simple example on permutations. According to Wolfram MathWorld:

A permutation, also called an "arrangement number" or "order," is a rearrangement of the elements of an ordered list S into a one-to-one correspondence with S itself.

For example, there are six permutations of ABC: ABC, ACB, BAC, BCA, CAB, and CBA. If a set has N elements, then the number of permutations of them is N! (N factorial). For the ABC string, the permutations are 3! = 3 * 2 * 1 = 6. Let's see this in Python:

```python
# permutations.py
from itertools import permutations 
print(list(permutations('ABC'))) 
```

This very short snippet of code produces the following result:

```bash
$ python permutations.py
[('A', 'B', 'C'), ('A', 'C', 'B'), ('B', 'A', 'C'), ('B', 'C', 'A'),
('C', 'A', 'B'), ('C', 'B', 'A')]
```

Be very careful when you play with permutations. Their number grows at a rate that is proportional to the factorial of the number of the elements you're permuting, and that number can get really big, really fast.


## Summary

In this chapter, we've taken another step toward expanding our Python vocabulary. We've seen how to drive the execution of code by evaluating conditions, along with how to loop and iterate over sequences and collections of objects. This gives us the power to control what happens when our code is run, which means we are getting an idea of how to shape it so that it does what we want, having it react to data that changes dynamically.

We've also seen how to combine everything together in a couple of simple examples, and in the end, we took a brief look at the itertools module, which is full of interesting iterators that can enrich our abilities with Python to a greater degree.

Now it's time to switch gears, take another step forward, and talk about functions. The next chapter is all about them, and they are extremely important. Make sure you're comfortable with what has been covered up to now. We want to provide you with interesting examples, so we'll have to go a little faster. Ready? Turn the page.


## Further Reading

- https://guicommits.com/python-yield-examples/
