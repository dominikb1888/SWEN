# Comprehensions and Generators

    "It's not the daily increase but daily decrease. Hack away at the unessential."

    – Bruce Lee

We love this quote from Bruce Lee. He was such a wise man! The second part in particular, "hack away at the unessential," is to us what makes a computer program elegant. After all, if there is a better way of doing things so that we don't waste time or memory, why wouldn't we?

Sometimes, there are valid reasons for not pushing our code up to the maximum limit: for example, sometimes, in order to achieve a negligible improvement, we have to sacrifice readability or maintainability. Does it make any sense to have a web page served in 1 second with unreadable, complicated code, when we can serve it in 1.05 seconds with readable, clean code? No, it makes no sense.

On the other hand, sometimes it's perfectly reasonable to try to shave off a millisecond from a function, especially when the function is meant to be called thousands of times. Every millisecond you save there means seconds saved over thousands of calls, and this could be meaningful for your application.

In light of these considerations, the focus of this chapter will not be to give you the tools to push your code to the absolute limits of performance and optimization no matter what, but rather to enable you to write efficient, elegant code that reads well, runs fast, and doesn't waste resources in an obvious way.

In this chapter, we are going to cover the following:


---

## Agenda

- Review on Functions (from Session 4)
- The map(), zip(), and filter() functions
- Comprehensions
- Generators


We will perform several measurements and comparisons and cautiously draw some conclusions. Please do keep in mind that on a different machine with a different setup or operating system, results may vary. Take a look at this code:

```python
#squares.py
def square1(n):
    return n ** 2  # squaring through the power operator

def square2(n):
    return n * n  # squaring through multiplication

```

Both functions return the square of n, but which is faster? From a simple benchmark that we ran on them, it looks like the second is slightly faster. If you think about it, it makes sense: calculating the power of a number involves multiplication and therefore, whatever algorithm you may use to perform the power operation, it's not likely to beat a simple multiplication such as the one in square2.

Do we care about this result? In most cases, no. If you're coding an e-commerce website, chances are you won't ever even need to raise a number to the second power, and if you do, it's likely to be a sporadic operation. You don't need to concern yourself with saving a fraction of a microsecond on a function you call a few times.

So, when does optimization become important? One very common case is when you have to deal with huge collections of data. If you're applying the same function on a million customer objects, then you want your function to be tuned up to its best. Gaining one-tenth of a second on a function called one million times saves you 100,000 seconds, which is about 27.7 hours—that makes a big difference! So, let's focus on collections, and let's see which tools Python gives you to handle them with efficiency and grace.

Many of the concepts we will see in this chapter are based on those of the iterator and iterable. Simply put, this is the ability of an object to return its next element when asked, and to raise a StopIteration exception when exhausted. We'll see how to code a custom iterator and iterable objects in Chapter 6, OOP, Decorators, and Iterators.

Some of the objects we're going to explore in this chapter are iterators, which save memory by only operating on a single element of a collection at a time rather than creating a modified copy. As a result, some extra work is needed if we just want to show the result of the operation. We will often resort to wrapping the iterator in a list() constructor. This is because passing an iterator to list(...) exhausts it and puts all the generated items in a newly created list, which we can easily print to show you its content. Let's see an example of using the technique on a range object:

```python

#list.iterable.py
>>> range(7)
range(0, 7)
>>> list(range(7))  # put all elements in a list to view them
[0, 1, 2, 3, 4, 5, 6]

```

We've highlighted the result of typing range(7) into a Python console. Notice that it doesn't show the contents of the range, because range never actually loads the entire sequence of numbers into memory. The second highlighted line shows how wrapping the range in a list() allows us to see the numbers it generated.


## The map, zip, and filter functions

We'll start by reviewing map(), filter(), and zip(), which are the main built-in functions you can employ when handling collections, and then we'll learn how to achieve the same results using two very important constructs: comprehensions and generators.


### map

According to the official Python documentation:

```python
map(function, iterable, …)
```

Return an iterator that applies function to every item of iterable, yielding the results. If additional iterable arguments are passed, function must take that many arguments and is applied to the items from all iterables in parallel. With multiple iterables, the iterator stops when the shortest iterable is exhausted.

We will explain the concept of yielding later on in the chapter. For now, let's translate this into code—we'll use a lambda function that takes a variable number of positional arguments, and just returns them as a tuple:

```python
#map.example.py
>>> map(lambda *a: a, range(3))  # 1 iterable
<map object at 0x10acf8f98>  # Not useful! Let's use list
>>> list(map(lambda *a: a, range(3)))  # 1 iterable
[(0,), (1,), (2,)]
>>> list(map(lambda *a: a, range(3), 'abc'))  # 2 iterables
[(0, 'a'), (1, 'b'), (2, 'c')]
>>> list(map(lambda *a: a, range(3), 'abc', range(4, 7)))  # 3
[(0, 'a', 4), (1, 'b', 5), (2, 'c', 6)]
>>> # map stops at the shortest iterator
>>> list(map(lambda *a: a, (), 'abc'))  # empty tuple is shortest
[]
>>> list(map(lambda *a: a, (1, 2), 'abc'))  # (1, 2) shortest
[(1, 'a'), (2, 'b')]
>>> list(map(lambda *a: a, (1, 2, 3, 4), 'abc'))  # 'abc' shortest
[(1, 'a'), (2, 'b'), (3, 'c')]

```

In the preceding code, you can see why we have to wrap calls in list(...). Without it, we get the string representation of a map object, which is not really useful in this context, is it?

You can also notice how the elements of each iterable are applied to the function; at first, the first element of each iterable, then the second one of each iterable, and so on. Notice also that map() stops when the shortest of the iterables we called it with is exhausted. This is actually a very nice behavior; it doesn't force us to level off all the iterables to a common length, nor does it break if they aren't all the same length.

map() is very useful when you have to apply the same function to one or more collections of objects. As a more interesting example, let's see the decorate-sort-undecorate idiom (also known as Schwartzian transform). It's a technique that was extremely popular in older Python versions, when sorting did not support the use of key functions. Nowadays, it is not needed as often, but it's a cool trick that still comes in handy once in a while.

Let's see a variation of it in the next example: we want to sort in descending order by the sum of credits accumulated by students, so as to have the best student at position 0. We write a function to produce a decorated object, we sort, and then we undecorate. Each student has credits in three (possibly different) subjects. In this context, to decorate an object means to transform it, either adding extra data to it, or putting it into another object, in a way that allows us to be able to sort the original objects the way we want. This technique has nothing to do with Python decorators, which we will explore later on in the book.

After sorting, we revert the decorated objects to get the original ones from them. This is referred to as undecorating.

```python

#decorate.sort.undecorate.py
students = [
    dict(id=0, credits=dict(math=9, physics=6, history=7)),
    dict(id=1, credits=dict(math=6, physics=7, latin=10)),
    dict(id=2, credits=dict(history=8, physics=9, chemistry=10)),
    dict(id=3, credits=dict(math=5, physics=5, geography=7)),
]

def decorate(student):
    # create a 2-tuple (sum of credits, student) from student dict
    return (sum(student['credits'].values()), student)

def undecorate(decorated_student):
    # discard sum of credits, return original student dict
    return decorated_student[1]

students = sorted(map(decorate, students), reverse=True)
students = list(map(undecorate, students))

# Let's start by understanding what each student object is. In fact, let's print the first one:

{'credits': {'history': 7, 'math': 9, 'physics': 6}, 'id': 0}

```

You can see that it's a dictionary with two keys: id and credits. The value of credits is also a dictionary in which there are three subject/grade key/value pairs. As you may recall from our visit to the data structures world, calling dict. values() returns an iterable object, with only the dictionary's values. Therefore, sum(student['credits'].values()) for the first student is equivalent to sum((9, 6, 7)). 

Let's print the result of calling decorate with the first student:

```bash

>>> decorate(students[0])
(22, {'credits': {'history': 7, 'math': 9, 'physics': 6}, 'id': 0})

```

If we decorate all the students like this, we can sort them on their total number of credits by just sorting the list of tuples. To apply the decoration to each item in students, we call map(decorate, students). We sort the result, and then we undecorate in a similar fashion. If you have gone through the previous chapters correctly, understanding this code shouldn't be too hard. 

Printing students after running the whole code yields:

```python

$ python decorate.sort.undecorate.py
[{'credits': {'chemistry': 10, 'history': 8, 'physics': 9}, 'id': 2},
 {'credits': {'latin': 10, 'math': 6, 'physics': 7}, 'id': 1},
 {'credits': {'history': 7, 'math': 9, 'physics': 6}, 'id': 0},
 {'credits': {'geography': 7, 'math': 5, 'physics': 5}, 'id': 3}]

```

You can see, by the order of the student objects, that they have indeed been sorted by 
the sum of their credits.

For more on the decorate-sort-undecorate idiom, there's a very nice introduction in the Sorting HOW TO section of the official Python documentation: https://docs.python.org/3.9/howto/sorting.html#the-old-way-using-decorate-sort-undecorate

One thing to notice about the sorting part is what happens when two or more students share the same total sum. The sorting algorithm would then proceed to sort the tuples by comparing the student objects with each other. This doesn't make any sense and, in more complex cases, could lead to unpredictable results, or even errors. If you want to be sure to avoid this issue, one simple solution is to create a three-tuple instead of a two-tuple, having the sum of credits in the first position, the position of the student object in the students list in second place, and the student object itself in third place. This way, if the sum of credits is the same, the tuples will be sorted against the position, which will always be different, and therefore enough to resolve the sorting between any pair of tuples.

### zip

We've already covered zip() in the previous chapters, so let's just define it properly, after which we want to show you how you could combine it with map(). 

According to the Python documentation:

```python

zip(*iterables)

```

Returns an iterator of tuples, where the i-th tuple contains the i-th element from each of the argument sequences or iterables. The iterator stops when the shortest input iterable is exhausted. With a single iterable argument, it returns an iterator of 1-tuples. With no arguments, it returns an empty iterator.

Let's see an example:

```python

#zip.grades.py
>>> grades = [18, 23, 30, 27]
>>> avgs = [22, 21, 29, 24]
>>> list(zip(avgs, grades))
[(22, 18), (21, 23), (29, 30), (24, 27)]
>>> list(map(lambda *a: a, avgs, grades))  # equivalent to zip
[(22, 18), (21, 23), (29, 30), (24, 27)]

```

Here, we're zipping together the average and the grade for the last exam for each student. Notice how easy it is to reproduce zip() using map() (the last two instructions of the example). Here as well, in order to visualize the results, we have to use list().

A simple example of the combined use of map() and zip() could be a way of calculating the element-wise maximum among sequences; that is, the maximum of the first element of each sequence, then the maximum of the second one, and so on:
```python

#maxims.py
>>> a = [5, 9, 2, 4, 7]
>>> b = [3, 7, 1, 9, 2]
>>> c = [6, 8, 0, 5, 3]
>>> maxs = map(lambda n: max(*n), zip(a, b, c))
>>> list(maxs)
[6, 9, 2, 9, 7]

```

Notice how easy it is to calculate the maximum values of three sequences. zip() is not strictly needed of course—we could just use map(). Sometimes it's hard, when showing a simple example, to grasp why using a technique might be good or bad. We forget that we aren't always in control of the source code; we might have to use a third-party library that we can't change the way we want. Having different ways to work with data is therefore really helpful.


### filter

According to the Python documentation:

```python

filter(function, iterable)

```

Construct an iterator from those elements of iterable for which function returns True. iterable may be either a sequence, a container which supports iteration, or an iterator. If function is None, the identity function is assumed, that is, all elements of iterable that are false are removed.

Let's see a very quick example:
```python

#filter.py
>>> test = [2, 5, 8, 0, 0, 1, 0]
>>> list(filter(None, test))
[2, 5, 8, 1]
>>> list(filter(lambda x: x, test))  # equivalent to previous one
[2, 5, 8, 1]
>>> list(filter(lambda x: x > 4, test))  # keep only items > 4
[5, 8]

```

Notice how the second call to filter() is equivalent to the first one. If we pass a function that takes one argument and returns the argument itself, only those arguments that are True will make the function return True. Therefore, this behavior is exactly the same as passing None. It's often a very good exercise to mimic some of the built-in Python behaviors. When you succeed, you can say you fully understand how Python behaves in a specific situation.

Armed with map(), zip(), and filter() (and several other functions from the Python standard library) we can manipulate sequences very effectively. But these functions are not the only way to do it. Let's look at one of the nicest features of Python: comprehensions.

## Comprehensions

A comprehension is a concise notation for performing some operation on each element of a collection of objects, and/or selecting a subset of elements that satisfy some condition. They are borrowed from the functional programming language Haskell (https://www.haskell.org/) and, together with iterators and generators, contribute to giving Python a functional flavor.

Python offers different types of comprehensions: list, dictionary, and set. We'll concentrate mainly on list comprehensions; once you understand those, the other types will be quite easy to grasp.

Let's start with a very simple example. We want to calculate a list with the squares of the first 10 natural numbers. How would you do it? There are a couple of equivalent ways:

```python

#squares.map.py
# If you code like this you are not a Python dev! ;)
>>> squares = []
>>> for n in range(10):
...     squares.append(n ** 2)
...
>>> squares
[0, 1, 4, 9, 16, 25, 36, 49, 64, 81]

# This is better, one line, nice and readable
>>> squares = map(lambda n: n**2, range(10))
>>> list(squares)
[0, 1, 4, 9, 16, 25, 36, 49, 64, 81]

```

The preceding example should be nothing new. Now, let's see how to achieve the same result using a list comprehension:

```python

#squares.comprehension.py
>>> [n ** 2 for n in range(10)]
[0, 1, 4, 9, 16, 25, 36, 49, 64, 81]

```

As simple as that. Isn't it elegant? Basically, we have placed a for loop within square brackets. Let's now filter out the odd squares. We'll show you how to do it with map() and filter() first, before then using a list comprehension again:

```python

#even.squares.py
# using map and filter
sq1 = list(
    map(lambda n: n ** 2, filter(lambda n: not n % 2, range(10)))
)
# equivalent, but using list comprehensions
sq2 = [n ** 2 for n in range(10) if not n % 2]

print(sq1, sq1 == sq2)  # prints: [0, 4, 16, 36, 64] True

```

We think that the difference in readability is now evident. The list comprehension reads much better. It's almost English: give us all squares (n ** 2) for n between 0 and 9 if n is even.

According to the Python documentation:

    A list comprehension consists of brackets containing an expression followed by a for clause, then zero or more for or if clauses. The result will be a new list resulting from evaluating the expression in the context of the for and if clauses which follow it.


### Nested comprehensions

Let's see an example of nested loops. It's very common when dealing with algorithms to have to iterate on a sequence using two placeholders. The first one runs through the whole sequence, left to right. The second one does, too, but it starts from the first one, instead of 0. The concept is that of testing all pairs without duplication. Let's see the classical for loop equivalent:

```python

#neoterm_default_ pairs.for.loop.py
items = 'ABCD'
pairs = []

for a in range(len(items)):
    for b in range(a, len(items)):
        pairs.append((items[a], items[b]))

```

If you print pairs at the end, you get:

```bash

$ python pairs.for.loop.py
[('A', 'A'), ('A', 'B'), ('A', 'C'), ('A', 'D'), ('B', 'B'), ('B', 
'C'), ('B', 'D'), ('C', 'C'), ('C', 'D'), ('D', 'D')]

```

All the tuples with the same letter are those where b is at the same position as a. Now, let's see how we can translate this to a list comprehension:

```python

#pairs.list.comprehension.py
items = 'ABCD'
pairs = [(items[a], items[b])
    for a in range(len(items)) for b in range(a, len(items))]

```

This version is just two lines long and achieves the same result. Notice that in this particular case, because the for loop over b has a dependency on a, it must follow the for loop over a in the comprehension. If you swap them around, you'll get a name error.

Another way of achieving the same result is to use the combinations_with_replacement() function from the itertools module (which we briefly introduced in Chapter 3, Conditionals and Iteration). You can read more about it in the official Python documentation.


### Filtering a comprehension

We can also apply filtering to a comprehension. Let's first do it with filter(), and find all Pythagorean triples whose short sides are numbers smaller than 10. We obviously don't want to test a combination twice, and therefore we'll use a trick similar to the one we saw in the previous example:

```python

#pythagorean.triple.py
from math import sqrt
#this will generate all possible pairs
mx = 10
triples = [(a, b, sqrt(a**2 + b**2))
    for a in range(1, mx) for b in range(a, mx)]
#this will filter out all non-Pythagorean triples
triples = list(
    filter(lambda triple: triple[2].is_integer(), triples))

print(triples)  # prints: [(3, 4, 5.0), (6, 8, 10.0)]

```

A Pythagorean triple is a triple (a, b, c) of integer numbers satisfying the equation a2 + b2 = c2.

In the preceding code, we generated a list of three-tuples, triples. Each tuple contains two integer numbers (the legs), and the hypotenuse of the Pythagorean triangle whose legs are the first two numbers in the tuple. For example, when a is 3 and b is 4, the tuple will be (3, 4, 5.0), and when a is 5 and b is 7, the tuple will be (5, 7, 8.602325267042627).

After generating all the triples, we need to filter out all those where the hypotenuse is not an integer number. In order to do this, we filter based on float_number. is_integer() being True. This means that, of the two example tuples we just showed you, the one with hypotenuse 5.0 will be retained, while the one with the 8.602325267042627 hypotenuse will be discarded. 

This is good, but we don't like the fact that the triple has two integer numbers and a float—they are all supposed to be integers. Let's use map() to fix this:

```python

#pythagorean.triple.int.py
from math import sqrt
mx = 10
# We generate a set of triplets
triples = [(a, b, sqrt(a**2 + b**2)) for a in range(1, mx) for b in range(a, mx)]
# Results in the tuples are mixed between Ints and floats, but not all result are float. Let's filter the in values
triples = filter(lambda triple: triple[2].is_integer(), triples)
# this will make the third number in the tuples integer
triples = list( map(lambda triple: triple[:2] + (int(triple[2]), ), triples))

print(triples)  # prints: [(3, 4, 5), (6, 8, 10)]

```

Notice the step we added. We take each element in triples and we slice it, taking only the first two elements in it. Then, we concatenate the slice with a one-tuple, in which we put the integer version of that float number that we didn't like. Seems like a lot of work, right? Indeed it is. Let's see how to do all this with a list comprehension:

```python

#pythagorean.triple.comprehension.py
from math import sqrt
# this step is the same as before
mx = 10
triples = [(a, b, sqrt(a**2 + b**2))
    for a in range(1, mx) for b in range(a, mx)]
# here we combine filter and map in one CLEAN list comprehension
triples = [(a, b, int(c)) for a, b, c in triples if c.is_integer()]
print(triples)  # prints: [(3, 4, 5), (6, 8, 10)]

```

That's much better! It's clean, readable, and shorter. It's not quite as elegant as it could have been, though. We're still wasting memory by constructing a list with a lot of triples that we end up discarding. We can fix that by combining the two comprehensions into one:

```python
#pythagorean.triple.walrus.py
from math import sqrt
# this step is the same as before
mx = 10
# We can combine generating and filtering in one comprehension
triples = [(a, b, int(c)) for a in range(1, mx) for b in range(a, mx) if (c := sqrt(a**2 + b**2)).is_integer()]
print(triples)  # prints: [(3, 4, 5), (6, 8, 10)]

```

Now that really is elegant. By generating the triples and filtering them in the same list comprehension, we avoid keeping any triple that doesn't pass the test in memory. Notice that we used an assignment expression to avoid needing to compute the value of sqrt(a**2 + b**2) twice.

We're going quite fast here, as anticipated in the Summary of Chapter 4, Functions, the Building Blocks of Code. Are you playing with this code? If not, we suggest you do. It's very important that you play around, break things, change things, and see what happens. Make sure you have a clear understanding of what is going on.

### Dictionary comprehensions

Dictionary comprehensions work exactly like list comprehensions, but to construct dictionaries. There is only a slight difference in the syntax. The following example will suffice to explain everything you need to know:

```python

#dictionary.comprehensions.py
from string import ascii_lowercase
lettermap = {c: k for k, c in enumerate(ascii_lowercase, 1)}

```

If you print lettermap, you will see the following (we omitted the intermediate results, but you get the gist):

```python

$ python dictionary.comprehensions.py
{'a': 1,
 'b': 2,
 ...
 'y': 25,
 'z': 26}

```

In the preceding code, we are enumerating the sequence of all lowercase ASCII letters (using the enumerate function). We then construct a dictionary with the resulting letter/number pairs as keys and values. Notice how the syntax is similar to the familiar dictionary syntax. 

There is also another way to do the same thing:

```python
lettermap = dict((c, k) for k, c in enumerate(ascii_lowercase, 1))
```

In this case, we are feeding a generator expression (we'll talk more about these later in this chapter) to the dict constructor.

Dictionaries do not allow duplicate keys, as shown in the following example:

```python

#dictionary.comprehensions.duplicates.py
word = 'Hello'
swaps = {c: c.swapcase() for c in word}
print(swaps)  # prints: {'H': 'h', 'e': 'E', 'l': 'L', 'o': 'O'}

```

We create a dictionary with the letters of the string 'Hello' as keys and the same letters, but with the case swapped, as values. Notice that there is only one 'l': 'L' pair. The constructor doesn't complain; it simply reassigns duplicates to the last value. Let's make this clearer with another example that assigns to each key its position in the string:

```python

#dictionary.comprehensions.positions.py
word = 'Hello'
positions = {c: k for k, c in enumerate(word)}
print(positions)  # prints: {'H': 0, 'e': 1, 'l': 3, 'o': 4}

```

Notice the value associated with the letter 'l': 3. The 'l': 2 pair isn't there; it has been overridden by 'l': 3.


### Set comprehensions

Set comprehensions are very similar to list and dictionary ones. Let's see one quick example:

```python

#set.comprehensions.py
word = 'Hello'
letters1 = {c for c in word}
letters2 = set(c for c in word)
print(letters1)  # prints: {'H', 'o', 'e', 'l'}
print(letters1 == letters2)  # prints: True

```

Notice how for set comprehensions, as for dictionaries, duplication is not allowed, and therefore the resulting set has only four letters. Also notice that the expressions assigned to letters1 and letters2 produce equivalent sets.

The syntax used to create letters1 is very similar to that of a dictionary comprehension. You can spot the difference only by the fact that dictionaries require keys and values, separated by colons, while sets don't. For letters2,  we fed a generator expression to the set() constructor.


## Generators

Generators are very powerful tools. They are based on the concept of iteration, as we said before, and they allow for coding patterns that combine elegance with efficiency.

Generators are of two types:

- Generator functions: These are very similar to regular functions, but instead of returning results through return statements, they use yield, which allows them to suspend and resume their state between each call.

- Generator expressions: These are very similar to the list comprehensions we've seen in this chapter, but instead of returning a list, they return an object that produces results one by one.

### Generator functions

Generator functions behave like regular functions in all respects, except for one difference: instead of collecting results and returning them at once, they are automatically turned into iterators that yield results one at a time when you call next on them. Generator functions are automatically turned into their own iterators by Python.

This is all very theoretical, so let's make it clear why such a mechanism is so powerful, and then let's see an example.

Say we asked you to count out loud from 1 to 1,000,000. You start, and at some point, we ask you to stop. After some time, we ask you to resume. At this point, what is the minimum information you need to be able to resume correctly? Well, you need to remember the last number you called. If we stopped you after 31,415, you will just go on with 31,416, and so on.

The point is that you don't need to remember all the numbers you said before 31,415, nor do you need them to be written down somewhere. Well, you may not know it, but you're behaving like a generator already!

Take a good look at the following code:

```python
# first.n.squares.py
def get_squares(n): # classic function approach
    return [x ** 2 for x in range(n)]
print(get_squares(10))

def get_squares_gen(n):  # generator approach
    for x in range(n):
        yield x ** 2  # we yield, we don't return
print(list(get_squares_gen(10)))
```

The result of the two print statements will be the same: [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]. But there is a huge difference between the two functions. get_squares() is a classic function that collects all the squares of numbers in [0, n) in a list, and returns it. On the other hand, get_squares_gen() is a generator and behaves very differently. Each time the interpreter reaches the yield line, its execution is suspended. The only reason those print statements return the same result is because we fed get_squares_gen() to the list() constructor, which exhausts the generator completely by asking for the next element until a StopIteration is raised. Let's see this in detail:

```python
#first.n.squares.manual.py
def get_squares_gen(n):
    for x in range(n):
        yield x ** 2

squares = get_squares_gen(4)  # this creates a generator object
print(squares)  #<generator object get_squares_gen at 0x10dd...>
print(next(squares))  # prints: 0
print(next(squares))  # prints: 1
print(next(squares))  # prints: 4
print(next(squares))  # prints: 9
#the following raises StopIteration, the generator is exhausted,
#any further call to next will keep raising StopIteration
print(next(squares))
```
Each time we call next on the generator object, we either start it (the first next) or make it resume from the last suspension point (any other next). The first time we call next on it, we get 0, which is the square of 0, then 1, then 4, then 9, and since the for loop stops after that (n is 4), the generator naturally ends. A classic function would at that point just return None, but in order to comply with the iteration protocol, a generator will instead raise a StopIteration exception.

This explains how a for loop works. When you call for k in range(n), what happens under the hood is that the for loop gets an iterator out of range(n) and starts calling next on it, until StopIteration is raised, which tells the for loop that the iteration has reached its end.

Having this behavior built into every iteration aspect of Python makes generators even more powerful because once we've written them, we'll be able to plug them into whatever iteration mechanism we want.

At this point, you're probably asking yourself why you would want to use a generator instead of a regular function. The answer is to save time and (especially) memory. 

We'll talk more about performance later, but for now, let's concentrate on one aspect: sometimes generators allow you to do something that wouldn't be possible with a simple list. For example, say you want to analyze all permutations of a sequence. If the sequence has a length of N, then the number of its permutations is N!. This means that if the sequence is 10 elements long, the number of permutations is 3,628,800. But a sequence of 20 elements would have 2,432,902,008,176,640,000 permutations. They grow factorially.

Now imagine you have a classic function that is attempting to calculate all permutations, put them in a list, and return it to you. With 10 elements, it would require probably a few seconds, but for 20 elements there is simply no way that it could be done (it would take thousands of years and require billions of gigabytes 
of memory).

On the other hand, a generator function will be able to start the computation and give you back the first permutation, then the second, and so on. Of course, you won't have the time to process them all—there are too many—but at least you'll be able to work with some of them. Sometimes the amount of data you have to iterate over is so huge that you cannot keep it all in memory in a list. In this case, generators are invaluable: they make possible that which otherwise wouldn't be.

So, in order to save memory (and time), use generator functions whenever possible.

It's also worth noting that you can use the return statement in a generator function. It will cause a StopIteration exception to be raised, effectively ending the iteration. This is extremely important. If a return statement were actually to make the function return something, it would break the iteration protocol. Python's consistency prevents this, and allows us great ease when coding. 

Let's see a quick example:

```python
#gen.yield.return.py
def geometric_progression(a, q):
    k = 0
    while True:
        result = a * q**k
        if result <= 100000:
```

```python
#gen.yield.return.py
def geometric_progression(a, q):
    k = 0
    while True:
        result = a * q**k
        if result <= 100000:
            yield result
        else:
            return
        k += 1

for n in geometric_progression(2, 5):
    print(n)

            yield result
        else:
            return
        k += 1

for n in geometric_progression(2, 5):
    print(n)

```

The preceding code yields all terms of the geometric progression, a, aq, aq2, aq3, .... When the progression produces a term that is greater than 100,000, the generator stops (with a return statement). Running the code produces the following result:

```bash

$ python gen.yield.return.py
2
10
50
250
1250
6250
31250
```

The next term would have been 156250, which is too big.

### Going beyond next

At the beginning of this chapter, we told you that generator objects are based on the iteration protocol. We'll see in Chapter 6, OOP, Decorators, and Iterators, a complete example of how to write a custom iterator/iterable object. For now, we just want you to understand how next() works.

What happens when you call next(generator) is that you're calling the generator.__ next__() method. Remember, a method is just a function that belongs to an object, and objects in Python can have special methods. __next__() is just one of these and its purpose is to return the next element of the iteration, or to raise StopIteration when the iteration is over and there are no more elements to return.

If you recall, in Python, an object's special methods are also called magic methods, or dunder (from "double underscore") methods.

When we write a generator function, Python automatically transforms it into an object that is very similar to an iterator, and when we call next(generator), that call is transformed in generator.__next__(). Let's revisit the previous example about generating squares:

```python

#first.n.squares.manual.method.py
def get_squares_gen(n):
    for x in range(n):
        yield x ** 2

squares = get_squares_gen(3)

print(squares.__next__())  # prints: 0
print(squares.__next__())  # prints: 1
print(squares.__next__())  # prints: 4
#the following raises StopIteration, the generator is exhausted,
#any further call to next will keep raising StopIteration
print(squares.__next__())

```

The result is exactly the same as the previous example, only this time instead of using the next(squares) proxy call, we're directly calling squares.__next__().

Generator objects also have three other methods that allow us to control their behavior: send(), throw(), and close(). send() allows us to communicate a value back to the generator object, while throw() and close(), respectively, allow us to raise an exception within the generator and close it. Their use is quite advanced and we won't be covering them here in detail, but we want to spend a few words on send(), with a simple example:

```python

#gen.send.preparation.py
def counter(start=0):
    n = start
    while True:
        yield n
        n += 1

c = counter()
print(next(c))  # prints: 0
print(next(c))  # prints: 1
print(next(c))  # prints: 2

```

The preceding iterator creates a generator object that will run forever. You can keep calling it, and it will never stop. Alternatively, you can put it in a for loop, for example, for n in counter(): ..., and it will go on forever as well. But what if you wanted to stop it at some point? One solution is to use a variable to control the while loop, as in something such as this:

```python
#gen.send.preparation.stop.py
stop = False
def counter(start=0):
    n = start
    while not stop:
        yield n
        n += 1
c = counter()
print(next(c))  # prints: 0
print(next(c))  # prints: 1
stop = True
print(next(c))  # raises StopIteration

```

This will do it. We start with stop = False, and until we change it to True, the generator will just keep going, like before. The moment we change stop to True though, the while loop will exit, and the following call to next will raise a StopIteration exception. This trick works, but we don't like it. The function depends on an external variable, and this can lead to issues: what if another function changes that stop? Moreover, the code is scattered. In a nutshell, this isn't good enough.

We can make it better by using generator.send(). When we call generator. send(), the value that we feed to send will be passed into the generator, execution is resumed, and we can fetch it via the yield expression. This is all very complicated when explained with words, so let's see an example:
```python

#gen.send.py
def counter(start=0):
    n = start
    while True:
        result = yield n             # A
        print(type(result), result)  # B
        if result == 'Q':
            break
        n += 1

c = counter()
print(next(c))         # C
print(c.send('Wow!'))  # D
print(next(c))         # E
print(c.send('Q'))     # F

```
Execution of the preceding code produces the following:
```bash
$ python gen.send.py
0
<class 'str'> Wow!
1
<class 'NoneType'> None
2
<class 'str'> Q
Traceback (most recent call last):
  File "gen.send.py", line 15, in <module>

    print(c.send('Q')) # F
StopIteration

```
We think it's worth going through this code line by line, as if we were executing it, to see whether we can understand what's going on.

We start the generator execution with a call to next() (#C). Within the generator, n is set to the same value as start. The while loop is entered, execution stops (#A), and n (0) is yielded back to the caller. 0 is printed on the console.

We then call send() (#D), execution resumes, result is set to 'Wow!'(still #A), and then its type and value are printed on the console (#B). result is not 'Q', so n is incremented by 1 and execution goes back to the while condition, which, being True, evaluates to True (that wasn't hard to guess, right?). Another loop cycle begins, execution stops again (#A), and n (1) is yielded back to the caller. 1 is printed on the console.

At this point, we call next() (#E), execution is resumed again (#A), and because we are not sending anything to the generator explicitly, the yield n expression (#A) returns None (the behavior is exactly the same as when we call a function that does not return anything). result is therefore set to None, and its type and value are again printed on the console (#B). Execution continues, result is not 'Q', so n is incremented by 1, and we start another loop again. Execution stops again (#A) and n (2) is yielded back to the caller. 2 is printed on the console.

And now for the grand finale: we call send again (#F), but this time we pass in 'Q', and so when execution is resumed, result is set to 'Q' (#A). Its type and value are printed on the console (#B), and then finally the if clause evaluates to True and the while loop is stopped by the break statement. The generator naturally terminates, which means a StopIteration exception is raised. You can see the print of its traceback on the last few lines printed on the console.

This is not at all simple to understand at first, so if it's not clear to you, don't be discouraged. You can keep reading on and come back to this example later.

Using send() allows for interesting patterns, and it's worth noting that send() can also be used to start the execution of a generator (provided you call it with None).


### The yield from expression

Another interesting construct is the yield from expression. This expression allows you to yield values from a sub-iterator. Its use allows for quite advanced patterns, so let's see a very quick example of it:

```python
#gen.yield.for.py
def print_squares(start, end):
    for n in range(start, end):
        yield n ** 2

for n in print_squares(2, 5):
    print(n)
```

The previous code prints the numbers 4, 9, and 16 on the console (on separate lines). By now, we expect you to be able to understand it by yourself, but let's quickly recap what happens. The for loop outside the function gets an iterator from print_ squares(2, 5) and calls next() on it until iteration is over. Every time the generator is called, execution is suspended (and later resumed) on yield n ** 2, which returns the square of the current n. Let's see how we can transform this code, benefiting from the yield from expression:

```python
#gen.yield.from.py
def print_squares(start, end):
    yield from (n ** 2 for n in range(start, end))

for n in print_squares(2, 5):
    print(n)
```

This code produces the same result, but as you can see, yield from is actually running a sub-iterator, (n ** 2 ...). The yield from expression returns to the caller each value the sub-iterator is producing. It's shorter and reads better.


### Generator expressions

Let's now talk about the other technique to generate values one at a time. The syntax is exactly the same as list comprehensions, only, instead of wrapping the comprehension with square brackets, you wrap it with round brackets. That is called a generator expression. In general, generator expressions behave like equivalent list comprehensions, but there is one very important thing to remember: generators allow for one iteration only, then they will be exhausted. 

Let's see an example:

```python
#generator.expressions.py
>>> cubes = [k**3 for k in range(10)]  # regular list
>>> cubes
[0, 1, 8, 27, 64, 125, 216, 343, 512, 729]
>>> type(cubes)
<class 'list'>
>>> cubes_gen = (k**3 for k in range(10))  # create as generator
>>> cubes_gen
<generator object <genexpr> at 0x103fb5a98>
>>> type(cubes_gen)
<class 'generator'>
>>> list(cubes_gen)  # this will exhaust the generator
[0, 1, 8, 27, 64, 125, 216, 343, 512, 729]
>>> list(cubes_gen)  # nothing more to give
[]
```
Look at the line in which the generator expression is created and assigned the name cubes_gen; you can see it's a generator object. In order to see its elements, we can use a for loop, a manual set of calls to next, or simply feed it to a list() constructor, which is what we did.

Notice how, once the generator has been exhausted, there is no way to recover the same elements from it again. We need to recreate it if we want to use it from scratch again.

In the next few examples, let's see how to reproduce map() and filter() using generator expressions. First, map():

```python
#gen.map.py
def adder(*n):
    return sum(n)
s1 = sum(map(adder, range(100), range(1, 101)))
s2 = sum(adder(*n) for n in zip(range(100), range(1, 101)))
```

In the previous example, s1 and s2 are exactly the same: they are the sum of adder(0, 1), adder(1, 2), adder(2, 3), and so on, which translates to sum(1, 3, 5, ...). The syntax is different, though we find the generator expression to be much more readable. Now, for filter():

```python
#gen.filter.py
cubes = [x**3 for x in range(10)]

odd_cubes1 = filter(lambda cube: cube % 2, cubes)
odd_cubes2 = (cube for cube in cubes if cube % 2)
```

In this example, odd_cubes1 and odd_cubes2 are the same: they generate a sequence of odd cubes. Yet again, we prefer the generator syntax. This should be evident when things get a little more complicated:

```python
#gen.map.filter.py
N = 20
cubes1 = map(
    lambda n: (n, n**3),
    filter(lambda n: n % 3 == 0 or n % 5 == 0, range(N))
)
cubes2 = (
    (n, n**3) for n in range(N) if n % 3 == 0 or n % 5 == 0)
```

The preceding code creates two generators, cubes1 and cubes2. They are exactly the same and return two-tuples (n, n3) when n is a multiple of 3 or 5. If you print the list (cubes1), you get: [(0, 0), (3, 27), (5, 125), (6, 216), (9, 729), (10, 1000), (12, 1728), (15, 3375), (18, 5832)].

See how much better the generator expression reads? It may be debatable when things are very simple, but as soon as you start nesting functions a bit, as we did in this example, the superiority of the generator syntax is evident. It's shorter, simpler, and more elegant.

Now, let us ask you: what is the difference between the following lines of code?

```python
#sum.example.py
s1 = sum([n**2 for n in range(10**6)])
s2 = sum((n**2 for n in range(10**6)))
s3 = sum(n**2 for n in range(10**6))
```

Strictly speaking, they all produce the same sum. The expressions to get s2 and s3 are exactly the same because the brackets in s2 are redundant. They are both generator expressions inside the sum() function. The expression to get s1 is different though. Inside sum(), we find a list comprehension. This means that in order to calculate s1, the sum() function has to call next() on a list one million times.

Do you see where we're losing time and memory? Before sum() can start calling next() on that list, the list needs to have been created, which is a waste of time and space. It's much better for sum() to call next() on a simple generator expression. There is no need to have all the numbers from range(10**6) stored in a list.

So, watch out for extra parentheses when you write your expressions. Sometimes it's easy to skip over these details that make our code very different. If you don't believe us, check out the following code:

```python
#sum.example.2.py
s = sum([n**2 for n in range(10**9)])  # this is killed
#s = sum(n**2 for n in range(10**9))    # this succeeds
print(s)  # prints: 333333332833333333500000000
```

Try running the example. If we run the first line on an old Linux machine with 6 GB 
RAM, this is what we get:
```bash
$ python sum.example.2.py
Killed 
```

On the other hand, if we comment out the first line, and uncomment the second one, this is the result:

```bash
$ python sum.example.2.py
333333332833333333500000000 
```

Sweet generator expressions. The difference between the two lines is that in the first one, a list with the squares of the first billion numbers must be made before being able to sum them up. That list is huge, and we ran out of memory (at least, Heinrich's machine did; if yours doesn't, try a bigger number), so Python kills the process for us.

But when we remove the square brackets, we no longer have a list. The sum function receives 0, 1, 4, 9, and so on until the last one, and sums them up. No problems.

### Some performance considerations

So, we've seen that we have many different ways of achieving the same result. We can use any combination of map(), zip(), and filter(), or choose to go with a comprehension or a generator. We may even decide to go with for loops; when the logic to apply to each running parameter isn't simple, these may be the best option.

Besides readability concerns, though, let's also talk about performance. When it comes to performance, usually there are two factors that play a major role: space and time. Space means the size of the memory that a data structure is going to take up. The best way to choose is to ask yourself if you really need a list (or tuple), or whether a simple generator function would work instead. 

If the answer is yes to the latter, go with the generator, as it will save a lot of space. The same goes for functions: if you don't actually need them to return a list or tuple, then you can transform them into generator functions as well.

Sometimes, you will have to use lists (or tuples); for example, there are algorithms that scan sequences using multiple pointers, or maybe they run over the sequence more than once. A generator function (or expression) can be iterated over only once and then it's exhausted, so in these situations it wouldn't be the right choice.

Time is a bit more complicated than space because it depends on more variables, and therefore it isn't possible to state that X is faster than Y with absolute certainty for all cases. However, based on tests run on Python today, we can say that on average, map() exhibits performance similar to comprehensions and generator expressions, while for loops are consistently slower.

In order to appreciate the reasoning behind these statements fully, we need to understand how Python works, which is a bit outside the scope of this book as it's quite technical in detail. Let's just say that map() and comprehensions run at C language speed within the interpreter, while a Python for loop is run as Python bytecode within the Python Virtual Machine, which is often much slower.

There are several different implementations of Python. The original one, and still the most common one, is CPython (https:// github.com/python/cpython), which is written in C. C is one of the most powerful and popular programming languages still used today.

How about we do a small exercise and try to find out whether the claims we made are accurate? We will write a small piece of code that collects the results of divmod(a, b) for a certain set of integer pairs, (a, b). We will use the time() function from the time module to calculate the elapsed time of the operations that we will perform:

```python
#performance.py
from time import time
mx = 5000

t = time()  # start time for the for loop
floop = []
for a in range(1, mx):
    for b in range(a, mx):
        floop.append(divmod(a, b))
print('for loop: {:.4f} s'.format(time() - t))  # elapsed time

t = time()  # start time for the list comprehension
compr = [
    divmod(a, b) for a in range(1, mx) for b in range(a, mx)]
print('list comprehension: {:.4f} s'.format(time() - t))

t = time()  # start time for the generator expression
gener = list(
    divmod(a, b) for a in range(1, mx) for b in range(a, mx))
print('generator expression: {:.4f} s'.format(time() - t))
```
As you can see, we're creating three lists: floop, compr, and gener. Running the code 
produces the following:
```bash
$ python performance.py
for loop: 2.3652 s
list comprehension: 1.5173 s
generator expression: 1.5289 s
```

The list comprehension runs in ~64% of the time taken by the for loop. That's impressive. The generator expression came very close to that, with ~65%. The difference in time between the list comprehension and generator expression is hardly significant, and if you re-run the example a few times, you will probably also see the generator expression take less time than the list comprehension.

An interesting result is to notice that, within the body of the for loop, we're appending data to a list. This implies that Python does the work, behind the scenes, of resizing it every now and then, allocating space for items to be appended. We guessed that creating a list of zeros, and simply filling it with the results, might have sped up the for loop, but we were wrong. Check it for yourself; you just need mx * (mx - 1) // 2 elements to be pre-allocated.

The approach we used here for timing execution is rather naïve. In Chapter 11, Debugging and Profiling, we will look at better ways of profiling code and timing execution. 

Let's see a similar example that compares a for loop and a map() call:

```python
#performance.map.py
from time import time
mx = 2 * 10 ** 7

t = time()
absloop = []
for n in range(mx):
    absloop.append(abs(n))
print('for loop: {:.4f} s'.format(time() - t))

t = time()
abslist = [abs(n) for n in range(mx)]
print('list comprehension: {:.4f} s'.format(time() - t))

t = time()
absmap = list(map(abs, range(mx)))
print('map: {:.4f} s'.format(time() - t))
```

This code is conceptually very similar to the previous example. The only thing that has changed is that we're applying the abs() function instead of divmod(), and we have only one loop instead of two nested ones. Execution gives the following result:

```bash
$ python performance.map.py
for loop: 2.3240 s
list comprehension: 1.0891 s
map: 0.5070 s
```

And map wins the race: it took ~47% of the time required by the list comprehension, and ~21% of the time needed by the for loop. Take these results with a pinch of salt, however, as the result might be different according to various factors, such as OS and Python version. But in general, we think it's safe to say that these results are good enough for having an idea when it comes to coding for performance.

Apart from the little case-by-case differences though, it's quite clear that the for loop option is the slowest one, so let's see why we still want to use it.

### Don't overdo comprehensions and generators

We've seen how powerful comprehensions and generator expressions can be. And they are, don't get us wrong, but the feeling that we have when we deal with them is that their complexity grows exponentially. The more you try to do within a single comprehension or a generator expression, the harder it becomes to read, understand, and therefore maintain or change.

If you check the Zen of Python again, there are a few lines that we think are worth keeping in mind when dealing with optimized code:
```bash
>>> import this
...
Explicit is better than implicit.
Simple is better than complex.
...
Readability counts.
...
If the implementation is hard to explain, it's a bad idea.
...
```

Comprehensions and generator expressions are more implicit than explicit, can be quite difficult to read and understand, and can be hard to explain. Sometimes, you have to break them apart using the inside-out technique to understand what's going on.

To give you an example, let's talk a bit more about Pythagorean triples. Just to remind you, a Pythagorean triple is a tuple of positive integers (a, b, c) such that  a2 + b2 = c2. We saw how to calculate them in the Filtering a comprehension section, but we did it in a very inefficient way. We were scanning all pairs of numbers below a certain threshold, calculating the hypotenuse, and filtering out those that were not valid Pythagorean triples.

A better way to get a list of Pythagorean triples is to generate them directly. There are many different formulas you can use to do this; here we will use the Euclidean formula. This formula says that any triple (a, b, c), where a = m2 - n2, b = 2mn and  c = m2 + n2, with m and n positive integers such that m > n, is a Pythagorean triple. For example, when m = 2 and n = 1, we find the smallest triple: (3, 4, 5). There is one catch though: consider the triple (6, 8, 10), which is like (3, 4, 5), only all the numbers are multiplied by 2. This triple is definitely Pythagorean, since 62 + 82 = 102, but we can derive it from (3, 4, 5) simply by multiplying each of its elements by 2. The same goes for (9, 12, 15), (12, 16, 20), and in general for all the triples that we can write as (3k, 4k, 5k), with k being a positive integer greater than 1.

A triple that cannot be obtained by multiplying the elements of another one by some factor, k, is called primitive. Another way of stating this is: if the three elements of a triple are coprime, then the triple is primitive. Two numbers are coprime when they don't share any prime factor among their divisors, that is, when their greatest common divisor (GCD) is 1. For example, 3 and 5 are coprime, while 3 and 6 are not because they are both divisible by 3.

So, the Euclidean formula tells us that if m and n are coprime, and m - n is odd, the triple they generate is primitive. In the following example, we will write a generator expression to calculate all the primitive Pythagorean triples whose hypotenuse, c, is less than or equal to some integer, N. This means we want all triples for which m2 + n2 ≤ N. When n is 1, the formula looks like this: m2 ≤ N - 1, which means we can approximate the calculation with an upper bound of m ≤ N1/2. To recap: m must be greater than n, they must also be coprime, and their difference m - n must be odd. Moreover, to avoid useless calculations, we'll put the upper bound for m at floor(sqrt(N)) + 1.

The floor function for a real number, x, gives the maximum integer, n, such that n < x, for example, floor(3.8) = 3, floor(13.1) = 13. Taking floor(sqrt(N)) + 1 means taking the integer part of the square root of N and adding a minimal margin just to make sure we don't miss any numbers.

Let's put all of this into code, step by step. We start by writing a simple gcd() function that uses Euclid's algorithm:

```python
#functions.py
def gcd(a, b):
    """Calculate the Greatest Common Divisor of (a, b). """
    while b != 0:
        a, b = b, a % b
    return a
```

The explanation of Euclid's algorithm is available on the web, so we won't spend any time talking about it here as we need to focus on the generator expression. The next step is to use the knowledge we gathered before to generate a list of primitive Pythagorean triples:

```python
#pythagorean.triple.generation.py
from functions import gcd
N = 50

triples = sorted(                                    # 1
    ((a, b, c) for a, b, c in (                      # 2
        ((m**2 - n**2), (2 * m * n), (m**2 + n**2))  # 3
        for m in range(1, int(N**.5) + 1)            # 4
        for n in range(1, m)                         # 5
        if (m - n) % 2 and gcd(m, n) == 1            # 6
    ) if c <= N), key=sum                            # 7
)
```

There you go. It's not easy to read, so let's go through it line by line. At #3, we start a generator expression that creates triples. You can see from #4 and #5 that we're looping on m in [1, M], with M being the integer part of sqrt(N), plus 1. On the other hand, n loops within [1, m), to respect the m > n rule. It's worth noting how we calculated sqrt(N), that is, N**.5, which is just another way to do it that we wanted to show you.

At #6, you can see the filtering conditions to make the triples primitive: (m - n) % 2 evaluates to True when (m - n) is odd, and gcd(m, n) == 1 means m and n are coprime. With these in place, we know the triples will be primitive. This takes care of the innermost generator expression. The outermost one starts at #2 and finishes at #7. We take the triples (a, b, c) in (...innermost generator...) such that c <= N.

Finally, at #1, we apply sorting to present the list in order. At #7, after the outermost generator expression is closed, you can see that we specify the sorting key to be the sum a + b + c. This is just our personal preference; there is no mathematical reason behind it.

So, what do you think? Was it straightforward to read? We don't think so. And believe us, this is still a simple example; we have both seen much worse in our careers. This kind of code is difficult to understand, debug, and modify. It should have no place in a professional environment. 

Let's see whether we can rewrite this code into something more readable:

```python
#pythagorean.triple.generation.for.py
from functions import gcd

def gen_triples(N):
    for m in range(1, int(N**.5) + 1):                  # 1
        for n in range(1, m):                           # 2
            if (m - n) % 2 and gcd(m, n) == 1:          # 3
                c = m**2 + n**2                         # 4
                if c <= N:                              # 5
                    a = m**2 - n**2                     # 6
                    b = 2 * m * n                       # 7
                    yield (a, b, c)                     # 8

sorted(gen_triples(50), key=sum)                        # 9
```

This is so much better. Let's go through it, line by line. You'll see how much easier it is to understand.

We start looping at #1 and #2, in exactly the same way we were looping in the previous example. On line #3, we have the filtering for primitive triples. On line #4, we deviate a bit from what we were doing before: we calculate c, and on line #5, we filter on c being less than or equal to N. Only when c satisfies that condition do we calculate a and b, and yield the resulting tuple. We could have calculated the values of a and b earlier, but by delaying until we know all conditions for a valid triple are satisfied, we avoid wasting time and CPU. On the last line, we apply sorting with the same key we were using in the generator expression example.

We hope you agree that this example is easier to understand. If we ever need to modify the code, this will be much easier, and less error-prone to work with than the generator expression.

If you print the results of both examples (they are the same), you will get this:

[(3, 4, 5), (5, 12, 13), (15, 8, 17), (7, 24, 25), (21, 20, 29), (35, 12, 37), (9, 40, 41)]  

The moral of the story is: try to use comprehensions and generator expressions as much as you can, but if the code starts to become complicated to modify or read, you may want to refactor it into something more readable. Your colleagues will thank you.

### Name localization

Now that we are familiar with all types of comprehensions and generator expressions, let's talk about name localization within them. Python 3 localizes loop variables in all four forms of comprehensions: list, dictionary, set, and generator expressions. This behavior is therefore different from that of the for loop. Let's look at some simple examples to show all the cases:

```python
#scopes.py
A = 100
ex1 = [A for A in range(5)]
print(A)  # prints: 100

ex2 = list(A for A in range(5))
print(A)  # prints: 100

ex3 = {A: 2 * A for A in range(5)}

print(A)  # prints: 100

ex4 = {A for A in range(5)}
print(A)  # prints: 100

s = 0
for A in range(5):
    s += A
print(A)  # prints: 4
```

In the preceding code, we declare a global name, A = 100, and then exercise list, dictionary, and set comprehensions and a generator expression. None of them alter the global name, A. Conversely, you can see at the end that the for loop modifies it. The last print statement prints 4.

Let's see what happens if A wasn't there:

```python
#scopes.noglobal.py
ex1 = [A for A in range(5)]
print(A)  # breaks: NameError: name 'A' is not defined
```

The preceding code would work in the same way with any other type of comprehension or a generator expression. After we run the first line, A is not defined in the global namespace. Once again, the for loop behaves differently:

```python
#scopes.for.py
s = 0
for A in range(5):
    s += A
print(A) # prints: 4
print(globals())
```

The preceding code shows that after a for loop, if the loop variable wasn't defined before it, we can find it in the global frame. To make sure of it, let's take a peek at it by calling the globals() built-in function:

```bash
$ python scopes.for.py
4
{'__name__': '__main__', '__doc__': None, ..., 's': 10, 'A': 4}
```

Together, with a lot of other boilerplate stuff that we have omitted, we can  spot 'A': 4.


### Generation behavior in built-ins

Generator-like behavior is quite common among the built-in types and functions. This is a major difference between Python 2 and Python 3. In Python 2, functions such as map(), zip(), and filter() returned lists instead of iterable objects. The idea behind this change is that if you need to make a list of those results, you can always wrap the call in a list() class, and you're done. On the other hand, if you just need to iterate and want to keep the impact on memory as light as possible, you can use those functions safely. Another notable example is the range() function. In Python 2 it returned a list, and there was another function called xrange() that behaved like the range() function now behaves in Python 3.

The idea of functions and methods that return iterable objects is quite widespread. You can find it in the open() function, which is used to operate on file objects (we'll see it in Chapter 8, Files and Data Persistence), but also in enumerate(), in the dictionary keys(), values(), and items() methods, and several other places.

It all makes sense: Python's aim is to try to reduce the memory footprint by avoiding wasting space wherever possible, especially in those functions and methods that are used extensively in most situations. At the beginning of this chapter, we said that it makes more sense to optimize the performance of code that has to deal with a lot of objects, rather than shaving off a few milliseconds from a function that we call twice a day. That is exactly what Python itself is doing here.


## One last example

Before we finish this chapter, we'll show you a simple problem that Fabrizio used to submit to candidates for a Python developer role in a company he used to work for. 

The problem is the following: write a function that returns the terms of the sequence 0 1 1 2 3 5 8 13 21 ..., up to some limit, N. If you haven't recognized it, that is the Fibonacci sequence, which is defined as F(0) = 0, F(1) = 1 and, for any n > 1, F(n) = F(n-1) + F(n-2). This sequence is excellent for testing knowledge about recursion, memoization techniques, and other technical details, but in this case, it was a good opportunity to check whether the candidate knew about generators.

Let's start with a rudimentary version, and then improve on it:

```python
#fibonacci.first.py
def fibonacci(N):
    """Return all fibonacci numbers up to N. """
    result = [0]
    next_n = 1
    while next_n <= N:
        result.append(next_n)
        next_n = sum(result[-2:])
    return result

print(fibonacci(0))   # [0]
print(fibonacci(1))   # [0, 1, 1]
print(fibonacci(50))  # [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

From the top: we set up the result list to a starting value of [0]. Then we start the iteration from the next element (next_n), which is 1. While the next element is not greater than N, we keep appending it to the list and calculating the next value in the sequence. We calculate the next element by taking a slice of the last two elements in the result list and passing it to the sum function. Add some print statements here and there if this is not clear to you, but by now we would expect it not to be an issue.

When the condition of the while loop evaluates to False, we exit the loop and return result. You can see the result of those print statements in the comments next to each of them. At this point, Fabrizio would ask the candidate the following question: What if I just wanted to iterate over those numbers? A good candidate would then change the code to what you'll find here (an excellent candidate would have started with it!):

```python
#fibonacci.second.py
def fibonacci(N):
    """Return all fibonacci numbers up to N. """
    yield 0
    if N == 0:
        return
    a = 0
    b = 1
    while b <= N:
        yield b
        a, b = b, a + b

print(list(fibonacci(0)))   # [0]
print(list(fibonacci(1)))   # [0, 1, 1]
print(list(fibonacci(50)))  # [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

This is actually one of the solutions he was given. We don't know why he kept it, but we're glad he did so we can show it to you. Now, the fibonacci() function is a generator function. First, we yield 0, and then, if N is 0, we return (this will cause a StopIteration exception to be raised). If that's not the case, we start iterating, yielding b at every loop cycle, and then updating a and b. All we need to be able to produce the next element of the sequence is the past two: a and b, respectively.

This code is much better, has a lighter memory footprint, and all we have to do to get a list of Fibonacci numbers is wrap the call with list(), as usual. But what about elegance? We can't leave it like that, can we? Let's try the following:

```python
#fibonacci.elegant.py
def fibonacci(N):
    """Return all fibonacci numbers up to N. """
    a, b = 0, 1
    while a <= N:
        yield a
        a, b = b, a + b
```

Much better. The whole body of the function is four lines, or five if you count the docstring. Notice how, in this case, using tuple assignment (a, b = 0, 1 and a,  b = b, a + b) helps in making the code shorter and more readable.


## Summary

In this chapter, we explored the concepts of iteration and generation a bit more deeply. We looked at the map(), zip(), and filter() functions in detail, and learned how to use them as an alternative to a regular for loop approach.

Then we covered the concept of comprehensions for lists, dictionaries, and sets. We explored their syntax and how to use them as an alternative to both the classic for loop approach and the use of the map(), zip(), and filter() functions.

Finally, we talked about the concept of generation in two forms: generator functions and expressions. We learned how to save time and space by using generation techniques and saw how they can make possible what wouldn't normally be so if we used a conventional approach based on lists.

We talked about performance, and saw that for loops come last in terms of speed, but they provide the best readability and flexibility to change. On the other hand, functions such as map() and filter(), and comprehensions, can be much faster.

The complexity of the code written using these techniques grows exponentially, so in order to favor readability and ease of maintainability, we still need to use the classic for loop approach at times. Another difference is in the name localization, where the for loop behaves differently from all other types of comprehensions.

The next chapter will be all about objects and classes. It is structurally similar to this one, in that we won't explore many different subjects—just a few of them—but we'll try to delve deeper into them.

Make sure you understand the concepts of this chapter before moving on to the next one. We're building a wall brick by brick, and if the foundation is not solid, you won't get very far.
