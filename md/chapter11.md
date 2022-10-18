# Debugging and Profiling

    "If debugging is the process of removing software bugs, then programming must be the process of putting them in."
    
    – Edsger W. Dijkstra

In the life of a professional coder, debugging and troubleshooting take up a significant amount of time. Even if you work on the most beautiful code base ever written by a human, there will still be bugs in it; that is guaranteed. We spend an awful lot of time reading other people's code and, in our opinion, a good software developer is someone who keeps an eye out for potential bugs, even when they're reading code that is not reported to be wrong or buggy.

Being able to debug code efficiently and quickly is a skill that every coder needs to keep improving. Like testing, debugging is a skill that is best learned through experience. There are guidelines you can follow, but there is no magic book that will teach you everything you need to know in order to become good at this.

We feel that on this particular subject, we have learned the most from our colleagues. It amazes us to observe someone very skilled attacking a problem. We enjoy seeing the steps they take, the things they verify to exclude possible causes, and the way they consider the suspects that eventually lead them to a solution.

Every colleague we work with can teach us something, or surprise us with a fantastic guess that turns out to be the right one. When that happens, don't just remain in wonderment (or worse, in envy), but seize the moment and ask them how they got to that guess and why. The answer will allow you to see whether there is something you can study in-depth later on so that, maybe next time, you'll be the one who will catch the bug.

Some bugs are very easy to spot. They come out of coarse mistakes and, once you see the effects of those mistakes, it's easy to find a solution that fixes the problem. But there are other bugs that are much more subtle, much more slippery, and require true expertise and a great deal of creativity and out-of-the-box thinking to be dealt with.

The worst of all, at least for us, are the non-deterministic ones. These sometimes happen, and sometimes don't. Some happen only in environment A but not in environment B, even though A and B are supposed to be exactly the same. Those bugs are the truly evil ones, and they can drive you crazy.

And of course, bugs don't just happen in the sandbox, right? With your boss telling you, "Don't worry! Take your time to fix this. Have lunch first!" Nope. They happen on a Friday at half-past five, when your brain is cooked and you just want to go home. It's in those moments when everyone is getting upset in split seconds, when your boss is breathing down your neck, that you have to be able to keep calm. And we do mean it. That's the most important skill to have if you want to be able to fight bugs effectively. If you allow your mind to get stressed, say goodbye to creative thinking, to logical deduction, and to everything you need at that moment. So, take a deep breath, sit properly, and focus.

In this chapter, we will try to demonstrate some useful techniques that you can employ according to the severity of the bug, and a few suggestions that will hopefully boost your weapons against bugs and issues.

Specifically, we're going to look at the following:

- Debugging techniques
- Troubleshooting guidelines
- [Profiling](Profiling)

## Debugging techniques

In this part, we'll introduce you to some of the techniques we use most often. This is not an exhaustive list, but it should give you some useful ideas for where to start when debugging your own Python code.

### Debugging with print

The key to understanding any bug is to understand what your code is doing at the point where the bug occurs. For this reason, we'll be looking at a few different techniques for inspecting the state of a program while it is running.

Probably the easiest technique of all is to add print() calls at various points in your code. This allows you to easily see which parts of your code are executed, and what the values of key variables are at different points during execution. For example, if you are developing a Django website and what happens on a page is not what you would expect, you can fill the view with prints and keep an eye on the console while you reload the page.

There are several drawbacks and limitations to using print() for debugging. To use this technique, you need to be able to modify the source code and run it in a terminal where you can see the output of your print() function calls. This is not a problem in your development environment on your own machine, but it does limit the usefulness of this technique in other environments.

When you scatter calls to print() in your code, you can easily end up duplicating a lot of debugging code. For example, you may want to print timestamps (like we did when we were measuring how fast list comprehensions and generators were), or somehow build up a string with the information that you want to display. Another issue is that it's extremely easy to forget calls to print() in your code.

For these reasons, we sometimes prefer to use a custom debugging function rather than just bare calls to print(). Let's see how.

### Debugging with a custom function

Having a custom debugging function in a snippet that you can quickly grab and paste into the code can be very useful. If you're fast, you can always code one on the fly. The important thing is to code it in a way that it won't leave stuff around when you eventually remove the calls and its definition. Therefore, it's important to code it in a way that is completely self-contained. Another good reason for this requirement is that it will avoid potential name clashes with the rest of the code.

Let's see an example of such a function:

```python

# custom.py
def debug(*msg, print_separator=True):
    print(*msg)
    if print_separator:
        print('-' * 40)

debug('Data is ...')
debug('Different', 'Strings', 'Are not a problem')
debug('After while loop', print_separator=False)
```

In this case, we are using a keyword-only argument to be able to print a separator, which is a line of 40 dashes.

The function is very simple. We just redirect whatever is in msg to a call to print() and, if print_separator is True, we print a line separator. Running the code will show the following:

```python

$ python custom.py
Data is ...
----------------------------------------
Different Strings Are not a problem
----------------------------------------
After while loop
```

As you can see, there is no separator after the last line.

This is just one easy way to augment a simple call to the print() function. Let's see how we can calculate a time difference between calls, using one of Python's tricky features to our advantage:

```python
# custom_timestamp.py
from time import sleep

def debug(*msg, timestamp=[None]):
    print(*msg)
    from time import time  # local import
    if timestamp[0] is None:
        timestamp[0] = time()  #1
    else:
        now = time()
        print(
            ' Time elapsed: {:.3f}s'.format(now - timestamp[0])
        )
        timestamp[0] = now  #2

debug('Entering nasty piece of code...')
sleep(.3)
debug('First step done.')
sleep(.5)
debug('Second step done.')
```

This is a bit trickier, but still quite simple. First, notice that we used an import statement inside our debug() function to import the time() function from the time module. This allows us to avoid having to add that import outside of the function, and maybe forget it there.

Take a look at how we defined timestamp. It's a function parameter with a list as its default value. In Chapter 4, Functions, the Building Blocks of Code, we warned against using mutable defaults for parameters, because the default value is initialized when Python parses the function and the same object persists across different calls to the function. Most of the time, this is not the behavior you want. In this case, however, we are taking advantage of this feature to store a timestamp from the previous call to the function, without having to use an external global variable. We borrowed this trick from our studies on closures, a technique that we encourage you to read about. After printing whatever message we had to print and importing time(), we inspect the content of the only item in timestamp. If it is None, we have no previous timestamp, so we set the value to the current time (#1). On the other hand, if we have a previous timestamp, we can calculate a difference (which we neatly format to three decimal digits) and finally, we put the current time in timestamp (#2).

Running this code outputs the following:

```python
$ python custom_timestamp.py
Entering nasty piece of code...
First step done.
 Time elapsed: 0.300s
Second step done.
 Time elapsed: 0.500s
```

Using a custom debug function solves some of the problems associated with just using print(). It reduces duplication of debugging code and makes it easier to remove all your debugging code when you no longer need it. However, it still requires modifying the code and running in a console where you can inspect the output. Later in this chapter, we'll see how we can overcome those difficulties by adding logging to our code.

### Using the Python debugger
  
Another very effective way of debugging Python is to use an interactive debugger. The Python standard library module pdb provides such a debugger; however, we usually prefer to use the third-party pdbpp package. pdbpp is a drop-in replacement for pdb, with a somewhat friendlier user interface and some handy additional tools, our favorite of which is the sticky mode, which allows you to see a whole function while you step through its instructions.

There are a few different ways to activate the debugger (the exact same methods work for both plain pdb and pdbpp). The most common approach is to add a call invoking the debugger to your code. This is known as adding a breakpoint to the code. When the code is run and the interpreter reaches the breakpoint, execution is suspended and you get console access to an interactive debugger session that allows you to inspect all the names in the current scope, and step through the program one line at a time. You can also alter data on the fly to change the flow of the program.

As a toy example, let's pretend we have a parser that is raising KeyError because a key is missing in a dictionary. The dictionary is from a JSON payload that we cannot control, and we just want, for the time being, to cheat and pass that control, since we're interested in what comes afterward. Let's see how we could intercept this moment, inspect the data, fix it, and get to the bottom of it, with the debugger:

```python
# pdebugger.py
# d comes from a JSON payload we don't control
d = {'first': 'v1', 'second': 'v2', 'fourth': 'v4'}
# keys also comes from a JSON payload we don't control
keys = ('first', 'second', 'third', 'fourth')

def do_something_with_value(value):
    print(value)

for key in keys:
    do_something_with_value(d[key])

print('Validation done.')
```

As you can see, this code will break when key gets the 'third' value, which is missing from the dictionary. Remember, we're pretending that both d and keys come dynamically from a JSON payload we don't control, so we need to inspect them in order to fix d and pass the for loop. If we run the code as it is, we get the following:

```bash
$ python pdebugger.py
v1
v2
Traceback (most recent call last):
  File "pdebugger.py", line 11, in <module>
    do_something_with_value(d[key])
KeyError: 'third'
```

So, we see that that key is missing from the dictionary, but since every time we run this code we may get a different dictionary or keys tuple, this information doesn't really help us. Let's inject a call to pdb just before the for loop. You have two options:

```python
import pdb
pdb.set_trace()
```

This is the most common way of doing it. You import pdb and call its set_trace() method. Many developers have macros in their editor to add this line with a keyboard shortcut. As of Python 3.7, though, we can simplify things even further, to this:

```python
breakpoint()
```

The new breakpoint() built-in function calls sys.breakpointhook() under the hood, which is programmed by default to call pdb.set_trace(). However, you can reprogram sys.breakpointhook() to call whatever you want, and therefore breakpoint() will point to that too, which is very convenient.

The code for this example is in the pdebugger_pdb.py module. If we now run this code, things get interesting (note that your output may vary a little and that all the comments in this output were added by us):

```bash
$ python pdebugger_pdb.py
[0] > pdebugger_pdb.py(17)<module>()
-> for key in keys:
(Pdb++) l
 17
 18 -> for key in keys:  # breakpoint comes in
 19 do_something_with_value(d[key])
 20

(Pdb++) keys  # inspecting the keys tuple
('first', 'second', 'third', 'fourth')
(Pdb++) d.keys()  # inspecting keys of 'd'
dict_keys(['first', 'second', 'fourth'])
(Pdb++) d['third'] = 'placeholder'  # add missing item
(Pdb++) c  # continue
v1
v2
placeholder
v4
Validation done.
```

First, note that when you reach a breakpoint, you're served a console that tells you where you are (the Python module) and which line is the next one to be executed. You can, at this point, perform a bunch of exploratory actions, such as inspecting the code before and after the next line, printing a stack trace, and interacting with the objects. In our case, we first inspect the keys tuple. We also inspect the keys of d. We see that 'third' is missing, so we put it in ourselves (could this be dangerous? Think about it). Finally, now that all the keys are in, we type c, which means (c)ontinue.

The debugger also gives you the ability to proceed with your code one line at a time using (n)ext, to (s)tep into a function for deeper analysis, or to handle breaks with  (b)reak. For a complete list of commands, please refer to the documentation (which you can find at https://docs.python.org/3.7/library/pdb.html) or type (h)elp in the debugger console.

You can see, from the output of the preceding run, that we could finally get to the end of the validation.

pdb (or pdbpp) is an invaluable tool that we use every day. So, go and have fun, set a breakpoint somewhere and try to inspect it, follow the official documentation, and try the commands in your code to see their effect and learn them well.

Notice that in this example we have assumed you installed pdbpp. If that is not the case, then you might find that some commands behave a bit differently in plain pdb. One example is the letter d, which pdb interprets as the down command. In order to get around that, you would have to add an ! in front of d, to tell pdb that it is meant to be interpreted literally, and not as a command.


### Inspecting logs

Another way of debugging a misbehaving application is to inspect its logs. A log is an ordered list of events that occurred or actions that were taken during the running of an application. If a log is written to a file on disk, it is known as a log file. Using logs for debugging is in some ways similar to adding print() calls or using a custom debug function. The key difference is that we typically add logging to our code from the start, to aid future debugging, rather than adding it during debugging and then removing it again. Another difference is that logging can easily be configured to output to a file or a network location. These two aspects make logging ideal for debugging code that's running on a remote machine that you might not have direct access to.

The fact that logging is usually added to the code before a bug has occurred does pose the challenge of deciding what to log. We would typically expect to find entries in the logs corresponding to the start and completion (and potentially also intermediate steps) of any important process that takes place within the application. The values of important variables should be included in these log entries. Errors also need to be logged, so that if a problem occurs, we can inspect the logs to find out what went wrong.

Nearly every aspect of logging in Python can be configured in various different ways. This gives us a lot of power, as we can change where logs are output to, which log messages are output, and how log messages are formatted, simply by reconfiguring the logging and without changing any other code. The four main types of objects involved in logging in Python are:

- Loggers: Expose the interface that the application code uses directly
- Handlers: Send the log records (created by loggers) to the appropriate destination
- Filters: Provide a finer-grained facility for determining which log records to output
- Formatters: Specify the layout of the log records in the final output

Logging is performed by calling methods on instances of the Logger class. Each line you log has a severity level associated with it. The most commonly used levels are DEBUG, INFO, WARNING, ERROR, and CRITICAL. Loggers use these levels to determine which log messages to output. Anything below the logger's level will be ignored. This means that you have to take care to log at the appropriate level. If you log everything at the DEBUG level, you will need to configure your logger at (or below) the DEBUG level to see any of your messages. This can quickly result in your log files becoming extremely big. A similar problem occurs if you log everything at the CRITICAL level.

Python gives you several choices of where to log to. You can log to a file, a network location, a queue, a console, your operating system's logging facilities, and so on. Where you send your logs will typically depend very much on the context. For example, when you run your code in your development environment, you will typically log to your terminal. If your application runs on a single machine, you might log to a file or send your logs to the operating system's logging facilities. On the other hand, if your application uses a distributed architecture that spans over multiple machines (such as in the case of service-oriented or microservice architectures), it's very useful to implement a centralized solution for logging so that all log messages coming from each service can be stored and investigated in a single place. It helps a lot, otherwise trying to correlate giant files from several different sources to figure out what went wrong can become truly challenging.

A service-oriented architecture (SOA) is an architectural pattern in software design in which application components provide services to other components via a communications protocol, typically over a network. The beauty of this system is that, when coded properly, each service can be written in the most appropriate language to serve its purpose. The only thing that matters is the communication with the other services, which needs to happen via a common format so that data exchange can be done.

Microservice architectures are an evolution of SOAs, but follow a different set of architectural patterns. 

The downside of the configurability of Python's logging is that the logging machinery is somewhat complex. The good news is that you often don't need to configure very much. If you start simple, it's actually not that difficult. To prove that, we will show you a very simple example of logging a few messages to a file:

```python
# log.py
import logging

logging.basicConfig(
    filename='ch11.log',
    level=logging.DEBUG, 
    format='[%(asctime)s] %(levelname)s: %(message)s',
    datefmt='%m/%d/%Y %I:%M:%S %p')

mylist = [1, 2, 3]
logging.info('Starting to process 'mylist'...')

for position in range(4):
    try:
        logging.debug(
            'Value at position %s is %s', position, mylist[position]
        )
    except IndexError:
        logging.exception('Faulty position: %s', position)

logging.info('Done processing 'mylist'.')
```

Let's go through it line by line. First, we import the logging module, then we set up a basic configuration. We specify a filename, configure the logger to output any log messages with level DEBUG or higher, and set the message format. We'll log the date and time information, the level, and the message.

With the configuration in place, we can start logging. We start by logging an info message that tells us we're about to process our list. Inside the loop, we will log the value at each position (we use the debug() function to log at the DEBUG level). We use debug() here so that we can filter out these logs in the future (by setting the minimum level to logging.INFO or more), because we might have to handle very big lists and we don't want to always log all the values.

If we get IndexError (and we do, since we are looping over range(4)), we call logging.exception(), which logs at the ERROR level, but it also outputs the exception traceback.

At the end of the code, we log another info message to say that we are done. After running this code, we will have a new ch11.log file with the following content:

```python
# ch11.log
[07/19/2021 10:32:28 PM] INFO: Starting to process 'mylist'...
[07/19/2021 10:32:28 PM] DEBUG: Value at position 0 is 1
[07/19/2021 10:32:28 PM] DEBUG: Value at position 1 is 2
[07/19/2021 10:32:28 PM] DEBUG: Value at position 2 is 3
[07/19/2021 10:32:28 PM] ERROR: Faulty position: 3
Traceback (most recent call last):
  File "log.py", line 16, in <module>
    'Value at position %s is %s', position, mylist[position]
IndexError: list index out of range
[07/19/2021 10:32:28 PM] INFO: Done processing 'mylist'.
```

This is exactly what we need to be able to debug an application that is running on a remote machine, rather than our own development environment. We can see what went on, the traceback of any exception raised, and so on.

The example presented here only scratches the surface of logging. For a more in-depth explanation, you can find information in the Python HOWTOs section of the official Python documentation: Logging HOWTO and Logging Cookbook.

Logging is an art. You need to find a good balance between logging everything and logging nothing. Ideally, you should log anything that you need to make sure your application is working correctly, and possibly all errors or exceptions.

### Other techniques

We'll end this section on debugging by briefly mentioning a couple of other techniques that you may find useful.

#### Reading tracebacks

Bugs often manifest as unhandled exceptions. The ability to interpret an exception traceback is therefore a crucial skill for successful debugging. Make sure that you have read and understood the section on tracebacks in Chapter 7, Exceptions and Context Managers. If you're trying to understand why an exception happened, it is often useful to inspect the state of your program (using the techniques we discussed above) at the lines mentioned in the traceback.


#### Assertions

Bugs are often the result of incorrect assumptions in our code. Assertions can be very useful for validating those assumptions. If our assumptions are valid, the assertions pass and all proceeds regularly. If they are not, we get an exception telling us which of our assumptions are incorrect. Sometimes, instead of inspecting with a debugger or print() statements, it's quicker to drop a couple of assertions in the code just to exclude possibilities. Let's see an example:

```python
# assertions.py
mylist = [1, 2, 3]  #  pretend this comes from an external source
assert 4 == len(mylist)  # this will break
for position in range(4):
    print(mylist[position])
```

In this example, we pretend that mylist comes from some external source that we don't control (maybe user input). The for loop assumes that mylist has four elements and we have added an assertion to validate that assumption. When we run the code, the result is this:

```python
$ python assertions.py
Traceback (most recent call last):
  File "assertions.py", line 4, in <module>
    assert 4 == len(mylist)  # this will break
AssertionError
```

This tells us exactly where the problem is.

Running a program with the -O flag active will cause Python to ignore all assertions. This is something to keep in mind if our code depends on assertions to work.

Assertions also allow for a longer format that includes a second expression, such as:

`assert expression1, expression2`

Typically, expression2 is a string that is fed to the AssertionError exception raised by the statement. For example, if we changed the assertion in the last example to the following:

`assert 4 == len(mylist), f"Mylist has {len(mylist)} elements"`

The result would be:

```python
$ python assertions.py
Traceback (most recent call last):
  File "assertions.py", line 19, in <module>
    assert 4 == len(mylist), f"Mylist has {len(mylist)} elements"
AssertionError: Mylist has 3 elements
```

#### Where to find information

In the Python official documentation, there is a section dedicated to debugging and profiling, where you can read up about the bdb debugger framework, and about modules such as faulthandler, timeit, trace, tracemalloc, and of course pdb. Just head to the standard library section in the documentation and you'll find all this information very easily.

Let's now explore some troubleshooting guidelines.


## Troubleshooting guidelines

In this short section, we'd like to give you a few tips that come from our troubleshooting experience.

### Where to inspect

Our first suggestion concerns where to place your debugging breakpoints. Regardless of whether you are using print(), a custom function, pdb, or logging, you still have to choose where to place the calls that provide you with the information. Some places are definitely better than others, and there are ways to handle the debugging progression that are better than others.

We normally avoid placing a breakpoint inside an if clause. If the branch containing the breakpoint is not executed, we lose the chance to get the information we wanted. Sometimes it can be difficult to reproduce a bug, or it may take a while for your code to reach the breakpoint, so think carefully before placing them.

Another important thing is where to start. Imagine that you have 100 lines of code that handle your data. Data comes in at line 1, and somehow it's wrong at line 100. You don't know where the bug is, so what do you do? You can place a breakpoint at line 1 and patiently step through all 100 lines, checking your data at every step. In the worst-case scenario, 99 lines (and many cups of coffee) later, you spot the bug. So, consider using a different approach.

Start at line 50, and inspect. If the data is good, it means the bug happens later, in which case you place your next breakpoint at line 75. If the data at line 50 is already bad, you go on by placing a breakpoint at line 25. Then, you repeat. Each time, you move either backward or forward, by half the jump you did last time.

In our worst-case scenario, your debugging would go from 1, 2, 3, ..., 99, in a linear fashion, to a series of jumps such as 50, 75, 87, 93, 96, ..., 99 which is much faster. In fact, it's logarithmic. This searching technique is called binary search; it's based on a divide-and-conquer approach, and it's very effective, so try to master it.

### Using tests to debug

In Chapter 10, Testing, we briefly introduced you to test-driven development (TDD). One TDD practice that you really should adopt, even if you don't subscribe to TDD as a whole, is writing tests that reproduce a bug before you start changing your code to fix the bug. There are a number of reasons for this. If you have a bug and all tests are passing, it means something is wrong or missing in your test code base. Adding these tests will help you ensure that you really do fix the bug: the tests should only pass if the bug is gone. Finally, having these tests will protect you from inadvertently reintroducing the same bug again when you make further changes to your code.

### Monitoring

Monitoring is also very important. Software applications can go completely crazy and have non-deterministic hiccups when they encounter edge-case situations such as the network being down, a queue being full, or an external component being unresponsive. In these cases, it's important to have an idea of what the big picture was when the problem occurred and be able to correlate it to something related to it in a subtle, perhaps mysterious way.

You can monitor API endpoints, processes, web pages' availability and load times, and almost everything that you can code. In general, when starting an application from scratch, it can be very useful to design it keeping in mind how you want to monitor it.

Now let's move on to see how we can profile Python code.


## Profiling Python

Profiling means having the application run while keeping track of several different parameters, such as the number of times a function is called and the amount of time spent inside it. 

Profiling is closely related to debugging. Although the tools and processes used are quite different, both activities involve probing and analyzing your code to understand where the root of a problem lies and then making changes to fix it. The difference is that instead of incorrect output or crashing, the problem we are trying to solve is poor performance.

Sometimes profiling will point to where the performance bottleneck is, at which point you will need to use the debugging techniques we discussed earlier in this chapter to understand why a particular piece of code does not perform as well as it should. For example, faulty logic in a database query might result in loading thousands of rows from a table instead of just hundreds. Profiling might show you that a particular function is called many more times than expected, at which point you would need to use your debugging skills to work out why that is and fix the problem.

There are a few different ways to profile a Python application. If you take a look at the profiling section in the standard library official documentation, you will see that there are two different implementations of the same profiling interface, profile and cProfile:

- cProfile is written in C and adds comparatively little overhead, which makes it suitable for profiling long-running programs
- profile is implemented in pure Python and, as a result, adds significant overhead to profiled programs

This interface does deterministic profiling, which means that all function calls, function returns, and exception events are monitored, and precise timings are made for the intervals between these events. Another approach, called statistical profiling, randomly samples the program's call stack at regular intervals, and deduces where time is being spent.

The latter usually involves less overhead, but provides only approximate results. Moreover, because of the way the Python interpreter runs the code, deterministic profiling doesn't add as much overhead as one would think, so we'll show you a simple example using cProfile from the command line.

There are situations where even the relatively low overhead of cProfile is not acceptable, for example, if you need to profile code on a live production web server because you cannot reproduce the performance problem in your development environment. For such cases, you really do need a statistical profiler. If you are interested in statistical profiling for Python, we suggest you look at py-spy (https://github.com/benfred/py-spy).

We're going to calculate Pythagorean triples (we know, you've missed them...) using the following code:

```python
# profiling/triples.py
def calc_triples(mx):
    triples = []
    for a in range(1, mx + 1):
        for b in range(a, mx + 1):
            hypotenuse = calc_hypotenuse(a, b)
            if is_int(hypotenuse):
                triples.append((a, b, int(hypotenuse)))
    return triples

def calc_hypotenuse(a, b):
    return (a**2 + b**2) ** .5

def is_int(n):  # n is expected to be a float
    return n.is_integer()

triples = calc_triples(1000)
```

The script is extremely simple; we iterate over the interval [1, mx] with a and b (avoiding repetition of pairs by setting b >= a) and we check whether they belong to a right triangle. We use calc_hypotenuse() to get hypotenuse for a and b, and then, with is_int(), we check whether it is an integer, which means (a, b, hypotenuse) is a Pythagorean triple. When we profile this script, we get information in a tabular form. 

The columns are ncalls (the number of calls to the function), tottime (the total time spent in each function), percall (the average time spent in each function per call), cumtime (the cumulative time spent in a function plus all functions it calls), percall (the average cumulative time spent per call), and filename:lineno(function). We'll trim a couple of columns to save space, so if you run the profiling yourself don't worry if you get a different result. Here is the result we got:

```bash
$ python -m cProfile profiling/triples.py
1502538 function calls in 0.489 seconds
Ordered by: standard name

ncalls  tottime  cumtime  filename:lineno(function)
500500    0.282    0.282  triples.py:13(calc_hypotenuse)
500500    0.065    0.086  triples.py:17(is_int)
     1    0.000    0.489  triples.py:3(<module>)
     1    0.121    0.489  triples.py:3(calc_triples)
     1    0.000    0.489  {built-in method builtins.exec}
  1034    0.000    0.000  {method 'append' of 'list' objects}
     1    0.000    0.000  {method 'disable' of '_lsprof.Profile...
500500    0.021    0.021  {method 'is_integer' of 'float' objects}
```

Even with this limited amount of data, we can still infer some useful information about this code. First, we can see that the time complexity of the algorithm we have chosen grows with the square of the input size. The number of calls to calc_hypotenuse() is exactly mx (mx + 1) / 2. We run the script with mx = 1000 and we got exactly 500,500 calls. Three main things happen inside that loop: we call calc_hypotenuse(), we call is_int(), and, if the condition is met, we append it to the triples list.

Taking a look at the cumulative times in the profiling report, we notice that the algorithm has spent 0.282 seconds inside calc_hypotenuse(), which is a lot more than the 0.086 seconds spent inside is_int(). Given that they were called the same number of times, let's see whether we can boost calc_hypotenuse() a little.

As it turns out, we can. As we mentioned earlier in this book, the ** power operator is quite expensive, and in calc_hypotenuse(), we're using it three times. Fortunately, we can easily transform two of those into simple multiplications, like this:

```python
def calc_hypotenuse(a, b): 
    return (a*a + b*b) ** .5 
```

This simple change should improve things. If we run the profiling again, we see that 0.282 is now down to 0.084. Not bad! This means now we're spending only about 29% as much time as before inside calc_hypotenuse().

Let's see whether we can improve is_int() as well, by changing it like this:

```python
def is_int(n): 
    return n == int(n) 
```

This implementation is different, and the advantage is that it also works when n is an integer. When we run the profiling against it, we see that the time taken inside the is_int() function (the cumtime) has gone down to 0.068 seconds. Interestingly, the total time spent in is_int() (excluding the time spent in the n.is_integer() method), has increased slightly, but by less time than we used to spend in  n.is_integer(). You will find the three versions in the source code for the book.

This example was trivial, of course, but enough to show you how you could profile an application. Having the number of calls that are made to a function helps us better understand the time complexity of our algorithms. For example, you wouldn't believe how many coders fail to see that those two for loops run proportionally to the square of the input size.

One thing to mention: the results of profiling will quite likely differ depending on what system you're running on. Therefore, it's quite important to be able to profile software on a system that is as close as possible to the one the software is deployed on, if not actually on it.


### When to profile

Profiling is super cool, but we need to know when it is appropriate to do it, and what to do with the results we get. Donald Knuth once said, "premature optimization is the root of all evil," and, although we wouldn't have put it quite so strongly, we do agree with him. After all, who are we to disagree with the man who gave us The Art of Computer Programming, TeX, and some of the coolest algorithms we ever studied at university? So, first and foremost: correctness. You want your code to deliver the correct results, therefore write tests, find edge cases, and stress your code in every way you think makes sense. Don't be protective, don't put things in the back of your brain for later because you think they're not likely to happen. Be thorough.

Second, take care of coding best practices. Remember the following: readability, extensibility, loose coupling, modularity, and design. Apply OOP principles: encapsulation, abstraction, single responsibility, open/closed, and so on. Read up on these concepts. They will open horizons for you, and they will expand the way you think about code. Third, refactor like a beast! The Boy Scouts rule says:

    "Always leave the campground cleaner than you found it."

Apply this rule to your code.

And, finally, when all of this has been taken care of, then and only then take care of optimizing and profiling.

Run your profiler and identify bottlenecks. When you have an idea of the bottlenecks you need to address, start with the worst one first. Sometimes, fixing a bottleneck causes a ripple effect that will expand and change the way the rest of the code works. Sometimes this is only a little, sometimes a bit more, according to how your code was designed and implemented. Therefore, start with the biggest issue first.

One of the reasons Python is so popular is that it is possible to implement it in many different ways. So, if you find yourself having trouble boosting up some part of your code using sheer Python, nothing prevents you from rolling up your sleeves, buying 200 liters of coffee, and rewriting the slow piece of code in C—guaranteed to be fun!

### Measuring execution time

Before we finish this chapter, let's briefly touch on the topic of measuring the execution time of code. Sometimes it is helpful to measure the performance of small pieces of code to compare their performance. For example, if you have different ways of implementing some operation and you really need the fastest version, you may want to compare their performance without profiling your entire application.

We've already seen some examples of measuring and comparing execution times earlier in this book, for example, in Chapter 5, Comprehensions and Generators, when we compared the performance of for loops, list comprehensions, and the map() function. At this point, we would like to introduce you to a better approach, using the timeit module. This module uses techniques such as timing many repeated executions of the code to improve measurement accuracy.

The timeit module can be a bit tricky to use. We recommend that you read about it in the official Python documentation and experiment with the examples there until you understand how to use it. Here we will just give a brief demonstration of using the command-line interface to time our two different versions of calc_hypotenuse() from the previous example:

```bash
$ python -m timeit -s 'a=2; b=3' '(a**2 + b**2) ** .5'
500000 loops, best of 5: 633 nsec per loop
```

Here we are running the timeit module, initializing variables a = 2 and b = 3, before timing the execution of (a**2 + b**2) ** .5. In the output, we can see that timeit ran 5 repetitions timing 500,000 loop iterations executing our calculation. Out of those 5 repetitions, the best average execution time over 500,000 iterations was 633 nanoseconds. Let's see how the alternative calculation, (a*a + b*b) ** .5, performs:

```bash
$ python -m timeit -s 'a=2; b=3' '(a*a + b*b) ** .5'
2000000 loops, best of 5: 126 nsec per loop
```

This time, we get 2,000,000 loop iterations with an average of 126 nanoseconds per loop. This confirms again that the second version is significantly faster. The reason we get more loop iterations in this case is that timeit automatically chooses the number of iterations to ensure the total running time is at least 0.2 seconds. This helps to improve accuracy by reducing the relative impact of measurement overhead.

For further information about measuring Python performance, make sure you check out pyperf (https://github.com/psf/pyperf) and pyperformance (https://github.com/python/pyperformance).


## Summary

In this short chapter, we looked at different techniques and suggestions for debugging, troubleshooting, and profiling our code. Debugging is an activity that is always part of a software developer's work, so it's important to be good at it.

If approached with the correct attitude, it can be fun and rewarding.

We explored techniques to inspect our code using custom functions, logging, debuggers, traceback information, profiling, and assertions. We saw simple examples of most of them and we also talked about some guidelines that will help when it comes to facing the fire. Just remember always to stay calm and focused, and debugging will be much easier. This, too, is a skill that needs to be learned and it's the most important. An agitated and stressed mind cannot work properly, logically, and creatively, therefore, if you don't strengthen it, it will be hard for you to put all of your knowledge to good use. So, when facing a difficult bug, if you have the opportunity, make sure you go for a short walk, or take a power nap—relax. Often, the solution presents itself after a good break.

In the next chapter, we are going to explore GUIs and scripts, taking an interesting detour from the more common web application scenario.
