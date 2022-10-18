# Exceptions and Context Managers

    The best-laid schemes o' mice an' men Gang aft agley

    – Robert Burns

These famous lines by Robert Burns should be etched into the mind of every programmer. Even if our code is correct, errors will happen. If we don't deal with them properly, they can cause our best-laid schemes to go awry.

Unhandled errors can cause software to crash or misbehave. If you are lucky, this just results in an irritated user. If you're unlucky, your business can end up losing money (an e-commerce website that keeps crashing is not likely to be very successful). Therefore, it is important to learn how to detect and handle errors. It is also a good idea to cultivate the habit of always thinking about what errors can occur and how your code should respond when they do.

This chapter is all about errors and dealing with the unexpected. We'll be learning about exceptions, which are Python's way of signaling that an error or other exceptional event has occurred. We'll also talk about context managers, which provide a mechanism to encapsulate and re-use error handling code.

In this chapter, we are going to cover the following:

•  Exceptions
•  Context managers


## Exceptions

Even though we haven't formally introduced them to you, by now we expect you to at least have a vague idea of what an exception is. In the previous chapters, we've seen that when an iterator is exhausted, calling next on it raises a StopIteration exception. We met IndexError when we tried accessing a list at a position that was outside the valid range. We also met AttributeError when we tried accessing an attribute on an object that didn't have it, and KeyError when we did the same with a key and a dictionary.

Now the time has come for us to talk about exceptions.

Sometimes, even though an operation or a piece of code is correct, there are conditions in which something may go wrong. For example, if we're converting user input from string to int, the user could accidentally type a letter in place of a digit, making it impossible for us to convert that value into a number. When dividing numbers, we may not know in advance whether we're attempting a division by zero. When opening a file, it could be missing or corrupted.

When an error is detected during execution, it is called an exception. Exceptions are not necessarily lethal; in fact, we've seen that StopIteration is deeply integrated into the Python generator and iterator mechanisms. Normally, though, if you don't take the necessary precautions, an exception will cause your application to break. Sometimes, this is the desired behavior, but in other cases, we want to prevent and control problems such as these. For example, we may alert the user that the file they're trying to open is corrupted or that it is missing so that they can either fix it or provide another file, without the need for the application to die because of this issue. Let us see an example of a few exceptions:

```bash
# exceptions/first.example.py 
>>> gen = (n for n in range(2)) 
>>> next(gen) 
0 
>>> next(gen) 
1 
>>> next(gen) 
Traceback (most recent call last): 
  File "<stdin>", line 1, in <module> 
StopIteration 

>>> print(undefined_name) 
Traceback (most recent call last): 
  File "<stdin>", line 1, in <module> 
NameError: name 'undefined_name' is not defined 

>>> mylist = [1, 2, 3] 
>>> mylist[5] 
Traceback (most recent call last): 
  File "<stdin>", line 1, in <module> 
IndexError: list index out of range 

>>> mydict = {'a': 'A', 'b': 'B'} 
>>> mydict['c'] 
Traceback (most recent call last): 
  File "<stdin>", line 1, in <module> 
KeyError: 'c' 

>>> 1 / 0 
Traceback (most recent call last): 
  File "<stdin>", line 1, in <module> 
ZeroDivisionError: division by zero
```

As you can see, the Python shell is quite forgiving. We can see Traceback, so that we have information about the error, but the shell itself still runs normally. This is a special behavior; a regular program or a script would normally exit immediately if nothing were done to handle exceptions. Let's see a quick example:
```python
# exceptions/unhandled.py
1 + "one"
print("This line will never be reached")
```

If we run this code, we get the following output:

```bash
$ python exceptions/unhandled.py
Traceback (most recent call last):
  File "exceptions/unhandled.py", line 2, in <module>
    1 + "one"
TypeError: unsupported operand type(s) for +: 'int' and 'str'
```

Because we did nothing to handle the exception, Python immediately exits once an exception occurs (after printing out information about the error).

### Raising exceptions

The exceptions we've seen so far were raised by the Python interpreter when it detected an error. However, you can also raise exceptions yourself, when a situation occurs that your own code considers to be an error. To raise an exception, use the raise statement. For example:
```bash
# exceptions/raising.py
>>> raise NotImplementedError("I'm afraid I can't do that")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
NotImplementedError: I'm afraid I can't do that
```

You can use any exception type you want, but it's a good idea to choose the exception type that best describes the particular error condition that has occurred. You can even define your own exception types (we'll see how to do that in a moment). Notice that the argument we passed to the Exception class is printed out as part of the error message.

Python has too many built-in exceptions to list here, but they are all documented at https://docs.python.org/3.9/library/ exceptions.html#bltin-exceptions.

### Defining your own exceptions

As we mentioned above, you can define your own custom exceptions. To do that, you just have to define a class that inherits from any other exception class. Ultimately, all exceptions derive from BaseException; however, this class is not intended to be directly subclassed, and your custom exceptions should inherit from Exception instead. In fact, nearly all built-in exceptions also inherit from Exception. Exceptions that do not inherit from Exception are meant for internal use by the Python interpreter.

### Tracebacks
The traceback that Python prints might initially look quite intimidating, but it is extremely useful for understanding what happened to cause the exception. Let's look at a traceback and see what it can tell us:

```python
# exceptions/trace.back.py
def squareroot(number):
    if number < 0:
        raise ValueError("No negative numbers please") 
    return number ** .5
 
def quadratic(a, b, c):
    d = b ** 2 - 4 * a * c
    return ((-b - squareroot(d)) / (2 * a),
            (-b + squareroot(d)) / (2 * a))

quadratic(1, 0, 1)  # x**2 + 1 == 0
```

Here we defined a function called quadratic(), which uses the famous quadratic formula to find the solution of a quadratic equation. Instead of using the sqrt() function from the math module, we wrote our own version (squareroot()), which raises an exception if the number is negative. When we call quadratic(1, 0, 1) to solve the equation x2+1=0, we will get a ValueError, because d is negative. When we run this, we get:

```bash
$ python exceptions/trace.back.py
Traceback (most recent call last):
  File "exceptions/trace.back.py", line 12, in <module>
    quadratic(1, 0, 1)  # x**2 + 1 == 0
  File "exceptions/trace.back.py", line 9, in quadratic
    return ((-b - squareroot(d)) / (2 * a),
  File "exceptions/trace.back.py", line 4, in squareroot
    raise ValueError("No negative numbers please")
ValueError: No negative numbers please
```

It is often useful to read tracebacks from bottom to top. On the very last line, we have the error message, telling us what went wrong: ValueError: No negative numbers please. The preceding lines tell us where the exception was raised (line 4 of exceptions/trace.back.py in the squareroot() function). We can also see the sequence of function calls that got us to the point where the exception was raised: squareroot() was called from line 9 in the function quadratic(), which was called from line 12, at the top level of the module. As you can see, the traceback is like a map that shows us the path through the code to where the exception happened. Following that path and examining the code in each function along the way is often very useful when you want to understand why an exception happened.


### Handling exceptions

To handle an exception in Python, you use the try statement. When you enter the try clause, Python will watch out for one or more different types of exceptions (according to how you instruct it), and if they are raised, it will allow you to react. 

The try statement is composed of the try clause, which opens the statement, followed by one or more except clauses that define what to do when an exception is caught. The except clauses may optionally be followed by an else clause, which is executed when the try clause is exited without any exception raised. After except and else clauses we can have a finally clause (also optional), whose code is executed regardless of whatever happened in the other clauses. The finally clause is typically used to clean up resources. You are also allowed to omit the except and else clauses and only have a try clause followed by a finally clause. This is helpful if we want exceptions to be propagated and handled elsewhere, but we do have some cleanup code that must be executed regardless of whether an exception occurs.

The order of the clauses is important. It must be try, except, else, finally. Also, remember that try must be followed by at least one except clause or a finally clause. Let us see an example:

```python
# exceptions/try.syntax.py 
def try_syntax(numerator, denominator): 
    try: 
        print(f'In the try block: {numerator}/{denominator}') 
        result = numerator / denominator 
    except ZeroDivisionError as zde: 
        print(zde) 
    else: 
        print('The result is:', result) 
        return result 
    finally: 
        print('Exiting') 

print(try_syntax(12, 4)) 
print(try_syntax(11, 0))
```

This example defines a simple try_syntax() function. We perform the division of two numbers. We are prepared to catch a ZeroDivisionError exception, which will occur if we call the function with denominator = 0. Initially, the code enters the try block. If denominator is not 0, result is calculated and, after leaving the try block, execution resumes in the else block. We print result and return it. Take a look at the output and you'll notice that just before returning result, which is the exit point of the function, Python executes the finally clause.

When denominator is 0, things change. Our attempt to calculate numerator / denominator raises a ZeroDivisionError. As a result, we enter the except block and print zde. 

The else block is not executed, because an exception was raised in the try block. Before (implicitly) returning None, we still execute the finally block. Take a look at the output and see whether it makes sense to you:

```bash
$ python try.syntax.py 
In the try block: 12/4     # try 
The result is: 3.0         # else 
Exiting                    # finally 
3.0                        # return within else 

In the try block: 11/0     # try 
division by zero           # except 
Exiting                    # finally 
None                       # implicit return end of function
```

When you execute a try block, you may want to catch more than one exception. For example, when calling the divmod() function, you can get a ZeroDivisionError if the second argument is 0, or TypeError if either argument is not a number. If you want to handle both in the same way, you can structure your code like this:

```python
# exceptions/multiple.py
values = (1, 2)

try:
    q, r = divmod(*values)
except (ZeroDivisionError, TypeError) as e:
    print(type(e), e)
```

This code will catch both ZeroDivisionError and TypeError. Try changing values = (1, 2) to values = (1, 0) or values = ('one', 2), and you will see the output change.

If you need to handle different exception types differently, you can just add more except clauses, like this:

```python
# exceptions/multiple.py 
try:
    q, r = divmod(*values)
except ZeroDivisionError:
    print("You tried to divide by zero!")
except TypeError as e:
    print(e)
```

Keep in mind that an exception is handled in the first block that matches that exception class or any of its base classes. Therefore, when you stack multiple except clauses like we've just done, make sure that you put specific exceptions at the top and generic ones at the bottom. In OOP terms, children on top, grandparents at the bottom. Moreover, remember that only one except handler is executed when an exception is raised.

Python does also allow you to use an except clause without specifying any exception type (this is equivalent to writing except BaseException). Doing so is generally not a good idea as it means you will also capture exceptions that are intended for internal use by the interpreter. They include the so-called system-exiting exceptions. These are SystemExit, which is raised when the interpreter exits via a call to the exit() function, and KeyboardInterrupt, which is raised when the user terminates the application by pressing Ctrl + C (or Delete on some systems).

You can also raise exceptions from within an except clause. For example, you might want to replace a built-in exception (or one from a third-party library) with your own custom exception. This is quite a common technique when writing libraries, as it helps shield users from implementation details of the library. Let's see an example:

```bash
# exceptions/replace.py
>>> class NotFoundError(Exception):
...     pass
...
>>> vowels = {'a': 1, 'e': 5, 'i': 9, 'o': 15, 'u': 21}
>>> try:
...     pos = vowels['y']
... except KeyError as e:
...     raise NotFoundError(*e.args)
...
Traceback (most recent call last):
  File "<stdin>", line 2, in <module>
KeyError: 'y'
```

During handling of the above exception, another exception occurred:

```bash
  File "<stdin>", line 4, in <module>
__main__.NotFoundError: y
```

By default, Python assumes that an exception that happens within an except clause is an unexpected error and helpfully prints out tracebacks for both exceptions. We can tell the interpreter that we are deliberately raising the new exception by using a raise from statement:

```bash
# exceptions/replace.py
>>> try:
...     pos = vowels['y']
... except KeyError as e:
...     raise NotFoundError(*e.args) from e
...
Traceback (most recent call last):
  File "<stdin>", line 2, in <module>
KeyError: 'y'
```

The above exception was the direct cause of the following exception:

```bash
Traceback (most recent call last):
  File "<stdin>", line 4, in <module>
__main__.NotFoundError: y
```

The error message has changed but we still get both tracebacks, which is actually very handy for debugging. If you really wanted to completely suppress the original exception, you could use from None instead of from e (try this yourself).

You can also use raise by itself, without specifying a new exception, to re-raise the original exception. This is sometimes useful if you want to log the fact that an exception has occurred, without actually suppressing or replacing the exception.

Programming with exceptions can be very tricky. You could inadvertently hide bugs by trapping exceptions that would have alerted you to their presence. Play it safe by keeping these simple guidelines in mind:

-  Keep the try clause as short as possible. It should contain only the code that may cause the exception(s) that you want to handle.

- Make the except clauses as specific as you can. It may be tempting to just write except Exception, but if you do you will almost certainly end up catching exceptions you did not actually intend to.

- Use tests to ensure that your code handles both expected and unexpected errors correctly. We shall talk more about writing tests in Chapter 10, Testing.

If you follow these suggestions, you will minimize the chance of getting it wrong.

### Not only for errors

Before we talk about context managers, we want to show you an unconventional use of exceptions, just to give you something to help you expand your views on them. Exceptions can be used for more than just errors:

```python
# exceptions/for.loop.py 
n = 100 
found = False 
for a in range(n): 
    if found: break 
    for b in range(n): 
        if found: break 
        for c in range(n): 
            if 42 * a + 17 * b + c == 5096: 
                found = True 
                print(a, b, c)  # 79 99 95
```

The preceding code is quite a common idiom if you deal with numbers. You have to iterate over a few nested ranges and look for a particular combination of a, b, and c that satisfies a condition. In this example, the condition is a trivial linear equation, but imagine something much cooler than that. What bugs us is having to check whether the solution has been found at the beginning of each loop, in order to break out of them as fast as we can when it is. The breakout logic interferes with the rest of the code and we don't like it, so we came up with a different solution for this. Take a look at it, and see whether you can adapt it to other cases too:

```python
# exceptions/for.loop.py 
class ExitLoopException(Exception): 
    pass 
 
try: 
    n = 100 
    for a in range(n): 
        for b in range(n): 
            for c in range(n): 
                if 42 * a + 17 * b + c == 5096: 
                    raise ExitLoopException(a, b, c) 
except ExitLoopException as ele: 
    print(ele.args)  # (79, 99, 95)
```

Can you see how much more elegant it is? Now the breakout logic is entirely handled with a simple exception whose name even hints at its purpose. As soon as the result is found, we raise ExitLoopException with the values that satisfy our condition, and immediately the control is given to the except clause that handles it. Notice that we can use the args attribute of the exception to get the values that were passed to the constructor.

## Context managers

When working with external resources or global state, we often need to perform some cleanup steps, like releasing the resources or restoring the original state, when we are done. Failing to clean up properly could result in all manner of bugs. Therefore, we need to ensure that our cleanup code will be executed even if an exception happens. We could use try/finally statements, but this is not always convenient and could result in a lot of repetition, as we often have to perform similar cleanup steps whenever we work with a particular type of resource. Context managers solve this problem by creating an execution context in which we can work with a resource or modified state and automatically perform any necessary cleanup when we leave that context, even if an exception was raised.

One example of global state that we may want to modify temporarily is the precision for decimal computations. For example, suppose we need to perform a particular computation to a specific precision, but we want to retain the default precision for the rest of our computations. We might do something like the following:

You may recall that the Decimal class allows us to perform arbitrary precision computations with decimal numbers. If not, you may want to review the relevant section of Chapter 2, Built-In Data Types now.

```python
# context/decimal.prec.py
from decimal import Context, Decimal, getcontext, setcontext

one = Decimal("1")
three = Decimal("3")

orig_ctx = getcontext()
ctx = Context(prec=5)
setcontext(ctx)
print(ctx)
print(one / three)
setcontext(orig_ctx)
print(one / three)
```

Notice that we store the current context, set a new context (with a modified precision), perform our calculation, and finally restore the original context. Running this produces the following output:

```bash
$ python decimal.prec.py
Context(prec=5, rounding=ROUND_HALF_EVEN, Emin=-999999,
Emax=999999, capitals=1, clamp=0, flags=[],
traps=[InvalidOperation, DivisionByZero, Overflow])
0.33333
0.3333333333333333333333333333
```

This seems fine, but what if an exception happened before we could restore the original context? We would be stuck with the wrong precision and the results of all subsequent computations would be incorrect! We can fix this by using a try/finally statement:

```python
# context/decimal.prec.py
orig_ctx = getcontext()
ctx = Context(prec=5)
setcontext(ctx)
try:
    print(ctx)
    print(one / three)
finally:
    setcontext(orig_ctx)
print(one / three)
```

That is much safer. Now we can rest assured that regardless of what happens in that try block, we will always restore the original context. It is not very convenient to have to keep writing try/finally like that, though. This is where context managers come to the rescue. The decimal module provides the localcontext context manager, which handles setting and restoring the context for us:

```python
# context/decimal.prec.py
from decimal import localcontext

with localcontext(Context(prec=5)) as ctx:
    print(ctx)
    print(one / three)
    print(one / three)
```

That is much easier to read (and type)! The with statement is used to enter a runtime context defined by a context manager. When exiting the code block delimited by the with statement, any cleanup operation defined by the context manager (in this case, restoring the decimal context) is executed automatically.

It is also possible to combine multiple context managers in one with statement. This is quite useful for situations where you need to work with multiple resources at the same time:

```python
# context/decimal.prec.py
with localcontext(Context(prec=5)), open("out.txt", "w") as out_f:
    out_f.write(f"{one} / {three} = {one / three}\n")
```

Here, we enter a local context and open a file (which acts as a context manager) in one with statement. We perform a calculation and write the result to the file. When we're done, the file is automatically closed and the default decimal context is restored. Don't worry too much about the details of working with files for now; we will learn all about that in Chapter 8, Files and Data Persistence.

Apart from decimal contexts and files, many other objects in the Python standard library can be used as context managers. For example:

- Socket objects, which implement a low-level networking interface, can be used as context managers to automatically close network connections.
- The lock classes used for synchronization in concurrent programming use the context manager protocol to automatically release locks.

In the rest of this chapter, we will show you how you can implement your own context managers.

### Class-based context managers

Context managers work via two magic methods: __enter__() is called just before entering the body of the with statement and __exit__() is called when exiting the with statement body. This means that you can easily create your own context manager simply by writing a class that implements these methods:

```python
# context/manager.class.py
class MyContextManager:
    def __init__(self):
        print("MyContextManager init", id(self))

    def __enter__(self):
        print("Entering 'with' context")
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        print(f"{exc_type=} {exc_val=} {exc_tb=}")
        print("Exiting 'with' context")
        return True
```

Here, we have defined a very simple context manager class called MyContextManager. There are a few interesting things to note about this class. Notice that our  __enter__() method returns self. This is quite common, but by no means required: you can return whatever you want from __enter__(), even None. The return value of the __enter__() method will be assigned to the variable named in the as clause of the with statement. Also notice the exc_type, exc_val, and exc_tb parameters of the __exit__() function. If an exception is raised within the body of the with statement, the interpreter will pass the type, value, and traceback of the exception as arguments through these parameters. If no exception is raised, all three arguments will be None.

Also notice that our __exit__() method returns True. This will cause any exception raised within the with statement body to be suppressed (as if we had handled it in a try/except statement). Had we returned False instead, such an exception would continue to be propagated after our __exit__() method has executed. The ability to suppress exceptions means that a context manager can be used as an exception handler. The benefit of this is that we can write our exception handling logic once and reuse it wherever we need it. This is just one more way in which Python helps us to apply the DRY principle to our code. Let us see our context manager in action:

```python
# context/manager.class.py
ctx_mgr = MyContextManager()
print("About to enter 'with' context")
with ctx_mgr as mgr:
    print("Inside 'with' context")
    print(id(mgr))
    raise Exception("Exception inside 'with' context")
    print("This line will never be reached")
print("After 'with' context")
```

Here, we have instantiated our context manager in a separate statement, before the with statement. We did this to make it easier for you to see what is happening; however, it is much more common for those steps to be combined like with MyContextManager() as mgr. Running this code produces the following output:

```bash
$ python context/manager.class.py
MyContextManager init 140340228792272
About to enter 'with' context
Entering 'with' context
Inside 'with' context
140340228792272
exc_type=<class 'Exception'> exc_val=Exception("Exception inside
'with' context") exc_tb=<traceback object at 0x7fa3817c5340>
Exiting 'with' context
After 'with' context
```

Study this output carefully to make sure you understand what is happening. We have printed some IDs to help verify that the object assigned to mgr is really the same object that we returned from __enter__(). Try changing the return values from the __enter__() and __exit__() methods and see what effect that has.


### Generator-based context managers

If you are implementing a class that represents some resource that needs to be acquired and released, it makes sense to implement that class as a context manager. Sometimes, however, we want to implement context manager behavior, but we do not have a class that it makes sense to attach that behavior to. For example, we may just want to use a context manager to re-use some error handling logic. In such situations, it would be rather tedious to have to write an additional class purely to implement the desired context manager behavior. Fortunately for us, Python has a solution.

The contextlib module in the standard library provides a handy contextmanager decorator that takes a generator function and converts it into a context manager (if you don't remember how generator functions work, you should review Chapter 5, Comprehensions and Generators). Behind the scenes, the decorator wraps the generator in a context manager object. The __enter__() method of this object starts the generator and returns whatever the generator yields. If an exception occurs within the with statement body, the __exit__() method passes the exception into the generator (using the generator's throw method). Otherwise, __exit__() simply calls next on the generator. Note that the generator must only yield once; a RuntimeError will be raised if the generator yields a second time. Let us translate our previous example into a generator-based context manager:

```python
# context/generator.py
from contextlib import contextmanager

@contextmanager
def my_context_manager():
    print("Entering 'with' context")
    val = object()
    print(id(val))
    try:
        yield val
    except Exception as e:
        print(f"{type(e)=} {e=} {e.__traceback__=}")
    finally:
        print("Exiting 'with' context")

print("About to enter 'with' context")
with my_context_manager() as val:
    print("Inside 'with' context")
    print(id(val))
    raise Exception("Exception inside 'with' context")
    print("This line will never be reached")
print("After 'with' context")
```

The output from running this is very similar to the previous example:

```bash
$ python context/generator.py
About to enter 'with' context
Entering 'with' context
139768531985040
Inside 'with' context
139768531985040
type(e)=<class 'Exception'> e=Exception("Exception inside 'with'
context") e.__traceback__=<traceback object at 0x7f1e65a42800>
Exiting 'with' context
After 'with' context
```

Most context manager generators have a similar structure to my_context_manager() in this example. They have some setup code, followed by a yield statement inside a try statement. Here, we yielded an arbitrary object, so that you can see that the same object is made available via the as clause of the with statement. It is also quite common to have just a bare yield with no value (in which case None is yielded). In such cases, the as clause of the with statement will typically be omitted.

One very useful feature of generator-based context managers is that they can also be used as function decorators. This means that if the entire body of a function needs to be inside a with statement context, you could save a level of indentation and just decorate the function instead.

In addition to the contextmanager decorator, the contextlib module also contains many very useful context managers. The documentation also provides several helpful examples of using and implementing context managers. Make sure you read it at https://docs.python.org/3/library/contextlib.html.

The examples we gave in this section were deliberately quite simple. They needed to be simple, to make it easier to see how context managers work. Study these examples carefully until you are confident that you understand them completely. Then, start writing your own context managers (both as classes and generators). Try to convert the try/except statement for breaking out of a nested loop that we saw earlier in this chapter into a context manager. The measure decorator that we wrote in Chapter 6, OOP, Decorators, and Iterators, is also a good candidate for converting to a context manager.

## Summary

In this chapter, we looked at exceptions and context managers.

We saw that exceptions are Python's way of signaling that an error has occurred. We showed you how to catch exceptions so that your program does not fail when errors inevitably do happen. We also showed you how you can raise exceptions yourself when your own code detects an error, and that you can even define your own exception types. We ended our exploration of exceptions by seeing that they are not only useful for signaling errors, but can also be used as a flow-control mechanism.

We ended the chapter with a brief overview of context managers. We saw how to use the with statement to enter a context defined by a context manager that performs cleanup operations when we exit the context. We also showed you how to create your own context managers, either as part of a class or by using a generator function.

We will see more context managers in action in the next chapter on files and data persistence.
