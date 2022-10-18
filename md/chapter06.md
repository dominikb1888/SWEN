# OOP, Decorators, and Iterators

    La classe non è acqua. (Class will out.)

    – Italian saying

We could probably write a whole book about object-oriented programming (OOP) and classes. In this chapter, we face the hard challenge of finding the balance between breadth and depth. There are simply too many things to talk about, and plenty of them would take more than this whole chapter if we described them in depth. Therefore, we will try to give you what we think is a good panoramic view of the fundamentals, plus a few things that may come in handy in the next chapters. Python's official documentation will help in filling the gaps.

In this chapter, we are going to cover the following topics:


---

## Agenda

• Decorators
• OOP with Python
• Iterators

---

## TOC

- [OOP, Decorators, and Iterators](#oop-decorators-and-iterators)
  - [Agenda](#agenda)
  - [TOC](#toc)
  - [Decorators](#decorators)
    - [A decorator factory](#a-decorator-factory)
  - [Object-oriented programming (OOP)](#object-oriented-programming-oop)
    - [The simplest Python class](#the-simplest-python-class)
    - [Class and object namespaces](#class-and-object-namespaces)
    - [Attribute shadowing](#attribute-shadowing)
    - [The self argument](#the-self-argument)
    - [Initializing an instance](#initializing-an-instance)
  - [OOP is about code reuse](#oop-is-about-code-reuse)
    - [Inheritance and composition](#inheritance-and-composition)
    - [Accessing a base class](#accessing-a-base-class)
    - [Multiple inheritance](#multiple-inheritance)
    - [Method resolution order](#method-resolution-order)
  - [Class and static methods](#class-and-static-methods)
    - [Static methods](#static-methods)
    - [Class methods](#class-methods)
    - [Private methods and name mangling](#private-methods-and-name-mangling)
    - [The property decorator](#the-property-decorator)
    - [The cached_property decorator](#the-cached_property-decorator)
    - [Operator overloading](#operator-overloading)
    - [Polymorphism – a brief overview](#polymorphism--a-brief-overview)
    - [Data classes](#data-classes)
  - [Writing a custom iterator](#writing-a-custom-iterator)
  - [Summary](#summary)



--- 

## Decorators

In Chapter 5, Comprehensions and Generators, we measured the execution time of various expressions. If you recall, we had to initialize a variable to the start time and subtract it from the current time after execution in order to calculate the elapsed time. We also printed it on the console after each measurement. That was tedious.

Every time we find ourselves repeating things, an alarm bell should go off. Can we put that code in a function and avoid repetition? The answer most of the time is yes, so let's look at an example:

```python
#decorators/time.measure.start.py
from time import sleep, time

def f():
    sleep(.3)

def g():
    sleep(.5)

t = time()
f()
print('f took:', time() - t)  # f took: 0.3001396656036377

t = time()
g()
print('g took:', time() - t)  # g took: 0.5039339065551758

```

In the preceding code, we defined two functions, f() and g(), which do nothing but sleep (for 0.3 and 0.5 seconds, respectively). We used the sleep() function to suspend the execution for the desired amount of time. Notice how the time measure is pretty accurate. Now, how do we avoid repeating that code and those calculations? One first potential approach could be the following:

```python
# decorators/time.measure.dry.py
from time import sleep, time

def f():
    sleep(.3)

def g():
    sleep(.5)

def measure(func):
    t = time()
    func()
    print(func.__name__, 'took:', time() - t)

measure(f)  # f took: 0.30434322357177734
measure(g)  # g took: 0.5048270225524902
```

Ah, much better now. The whole timing mechanism has been encapsulated in a function so we don't repeat code. We print the function name dynamically and it's easy enough to code. What if we needed to pass any arguments to the function we measure? This code would get just a bit more complicated, so let's see an example:

```python
#decorators/time.measure.arguments.py
from time import sleep, time

def f(sleep_time=0.1):
    sleep(sleep_time)

def measure(func, *args, **kwargs):
    t = time()
    func(*args, **kwargs)
    print(func.__name__, 'took:', time() - t)

measure(f, sleep_time=0.3)  # f took: 0.30056095123291016
measure(f, 0.2)  # f took: 0.2033553123474121
```

Now, f() is expecting to be fed sleep_time (with a default value of 0.1), so we don't need g() anymore. We also had to change the measure() function so that it now accepts a function, any variable positional arguments, and any variable keyword arguments. In this way, whatever we call measure() with, we redirect those arguments to the call to func() we do inside.

This is very good, but we can push it a little bit further. Let's say we somehow want to have that timing behavior built into the f() function, so that we could just call it and have that measure taken. Here's how we could do it:

```python

# ch06/decorators/time.measure.deco1.py
from time import sleep, time

def f(sleep_time=0.1):
    sleep(sleep_time)

def measure(func):
    def wrapper(*args, **kwargs):
        t = time()
        func(*args, **kwargs)
        print(func.__name__, 'took:', time() - t)
    
    return wrapper

f = measure(f)  # decoration point
f(0.2)  # f took: 0.20372915267944336
f(sleep_time=0.3)  # f took: 0.30455899238586426
print(f.__name__)  # wrapper <- ouch!
```

The preceding code is probably not so straightforward. Let's see what happens here. The magic is in the decoration point. We basically reassign f() with whatever is returned by measure() when we call it with f as an argument. Within measure(), we define another function, wrapper(), and then we return it. So, the net effect is that after the decoration point, when we call f(), we're actually calling wrapper() (you can witness this in the last line of code). Since the wrapper() inside is calling func(), which is f(), we are actually closing the loop.

The wrapper() function is, not surprisingly, a wrapper. It takes variable positional and keyword arguments and calls f() with them. It also does the time measurement calculation around the call. This technique is called decoration, and measure() is, effectively, a decorator. This paradigm became so popular and widely used that, in version 2.4, Python added a special syntax for it. You can read the specifics in PEP 318 (https://www.python.org/dev/peps/pep-0318/). In Python 3.9, the decorator syntax was slightly amended, to relax some grammar restrictions; this change was brought about in PEP 614 (https://www.python.org/dev/peps/pep-0614/).

Let's now explore three cases: one decorator, two decorators, and one decorator that takes arguments. First, the single decorator case:

```python
#decorators/syntax.py
def func(arg1, arg2, ...):
    pass
func = decorator(func)

#is equivalent to the following:

@decorator
def func(arg1, arg2, ...):
    pass
```

Basically, instead of manually reassigning the function to what was returned by the decorator, we prepend the definition of the function with the special syntax, @decorator_name.

We can apply multiple decorators to the same function in the following way:

```python

# decorators/syntax.py
def func(arg1, arg2, ...):
    pass
func = deco1(deco2(func))

# is equivalent to the following:

@deco1
@deco2
def func(arg1, arg2, ...):
    pass

```

When applying multiple decorators, it is important to pay attention to the order. In the preceding example, func() is decorated with deco2() first, and the result is decorated with deco1(). A good rule of thumb is the closer the decorator is to the function, the sooner it is applied.

Some decorators can take arguments. This technique is generally used to produce another decorator (in which case, the object would be called a decorator factory). Let's look at the syntax, and then we'll see an example of it:

```python

#decorators/syntax.py
def func(arg1, arg2, ...):
    pass
func = decoarg(arg_a, arg_b)(func)

#is equivalent to the following:

@decoarg(arg_a, arg_b)
def func(arg1, arg2, ...):
    pass

```

As you can see, this case is a bit different. First, decoarg() is called with the given arguments, and then its return value (the actual decorator) is called with func(). Before we give you another example, let's fix one thing that is bothering us. Take a look at this code from our previous example:

```python
#decorators/time.measure.deco1.py
def measure(func):
    def wrapper(*args, **kwargs):
        …
    return wrapper

f = measure(f)  # decoration point
print(f.__name__)  # wrapper <- ouch!
```

We don't want to lose the original function's name and docstring when we decorate it. But because inside our decorator we return wrapper, the decorated function, f(), is reassigned to it and therefore its original attributes are lost, replaced with the attributes of wrapper. There is an easy fix for that from the beautiful functools module. We will fix the last example, and we will also rewrite its syntax to use the @-operator:

```python
#decorators/time.measure.deco2.py
from time import sleep, time
from functools import wraps

def measure(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        t = time()
        func(*args, **kwargs)
        print(func.__name__, 'took:', time() - t)
    return wrapper

@measure
def f(sleep_time=0.1):
    """I'm a cat. I love to sleep! """
    sleep(sleep_time)
```

Let's see another example. We want a decorator that prints an error message when the result of a function is greater than a certain threshold. We will also take this opportunity to show you how to apply two decorators at once:

```python
#decorators/two.decorators.py
from time import time
from functools import wraps

def measure(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        t = time()
        result = func(*args, **kwargs)
        print(func.__name__, 'took:', time() - t)
        return result
    return wrapper

def max_result(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        result = func(*args, **kwargs)
        if result > 100:
            print(
                f'Result is too big ({result}). '
                'Max allowed is 100.'
            )
        return result
    return wrapper

@measure
@max_result
def cube(n):
    return n ** 3

print(cube(2))
print(cube(5))
```

We had to enhance the measure() decorator, so that its wrapper now returns the result of the call to func(). The max_result decorator does that as well, but before returning, it checks that result is not greater than 100, which is the maximum allowed. 

We decorated cube() with both of them. First, max_result() is applied, then measure(). Running this code yields this result:

```bash
$ python two.decorators.py
cube took: 3.0994415283203125e-06
8

Result is too big (125). Max allowed is 100.
cube took: 1.0013580322265625e-05
125
```

For your convenience, we have separated the results of the two calls with a blank line. In the first call, the result is 8, which passes the threshold check. The running time is measured and printed. Finally, we print the result (8).

On the second call, the result is 125, so the error message is printed, the result returned, and then it's the turn of measure(), which prints the running time again, and finally, we print the result (125).

Had we decorated the cube() function with the same two decorators but in a different order, the order of the printed messages would also have been different.

### A decorator factory

Let's simplify this example now, going back to a single decorator: max_result(). We want to make it so that we can decorate different functions with different thresholds, as we don't want to write one decorator for each threshold. Let's therefore amend max_result() so that it allows us to decorate functions by specifying the threshold dynamically:

```python
# decorators/decorators.factory.py
from functools import wraps

def max_result(threshold):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            result = func(*args, **kwargs)
            if result > threshold:
                print(
                    f'Result is too big ({result}). '
                    f'Max allowed is {threshold}.'
                )
            return result
        return wrapper
    return decorator

@max_result(75)
def cube(n):
    return n ** 3

print(cube(5))
```

The preceding code shows you how to write a decorator factory. If you recall, decorating a function with a decorator that takes arguments is the same as writing func = decorator(argA, argB)(func), so when we decorate cube with max_result(75), we're doing cube = max_result(75)(cube).

Let's go through what happens, step by step. When we call max_result(75), we enter its body. A decorator() function is defined inside, which takes a function as its only argument. Inside that function, the usual decorator trick is performed. We define wrapper(), inside of which we check the result of the original function's call. The beauty of this approach is that from the innermost level, we can still refer to both func and threshold, which allows us to set the threshold dynamically.

The wrapper() function returns result, decorator() returns wrapper(), and max_ result() returns decorator(). This means that our cube = max_result(75)(cube) call actually becomes cube = decorator(cube). Not just any decorator() though, but one for which threshold has a value of 75. This is achieved by a mechanism called closure.

Dynamically created functions that are returned by other functions are called closures. Their main feature is that they have full access to the variables and names defined in the local namespace where they were created, even though the enclosing function has returned and finished executing.

Running the last example produces the following result:

$ python decorators.factory.py
Result is too big (125). Max allowed is 75.
125

The preceding code allows us to use the max_result() decorator with different thresholds, like this:

```python

# decorators/decorators.factory.py
@max_result(75)
def cube(n):
    return n ** 3

@max_result(100)
def square(n):
    return n ** 2

@max_result(1000)
def multiply(a, b):
    return a * b
```

Note that every decoration uses a different threshold value.

Decorators are very popular in Python. They are used quite often and they make the code simpler, and more elegant.

## Object-oriented programming (OOP)

It's been quite a long and hopefully nice journey and, by now, we should be ready to explore OOP. We'll use the definition from Kindler, E.; Krivy, I. (2011). Object-oriented simulation of systems with sophisticated control (International Journal of General Systems), and adapt it to Python:

    Object-oriented programming (OOP) is a programming paradigm based on the concept of "objects", which are data structures that contain data, in the form of attributes, and code, in the form of functions known as methods. A distinguishing feature of objects is that an object's method can access and often modify the data attributes of the object with which they are associated (objects have a notion of "self"). In OO programming, computer programs are designed by making them out of objects that interact with one another.

Python has full support for this paradigm. Actually, as we have already said, everything in Python is an object, so this shows that OOP is not just supported by Python, but it's a core feature of the language. The two main players in OOP are objects and classes. Classes are used to create objects (objects are instances of the classes from which they were created), so we could see them as "instance factories." 

When objects are created by a class, they inherit the class attributes and methods. They represent concrete items in the program's domain.


### The simplest Python class

We will start with the simplest class you could ever write in Python:

```python
# oop/simplest.class.py
class Simplest():  # when empty, the braces are optional
    pass

print(type(Simplest))  # what type is this object?
simp = Simplest()  # we create an instance of Simplest: simp
print(type(simp))  # what type is simp?
# is simp an instance of Simplest?
print(type(simp) is Simplest)  # There's a better way to do this
```

Let's run the preceding code and explain it line by line:

```bash
$ python simplest.class.py
<class 'type'>
<class '__main__.Simplest'>
True
```

The Simplest class we defined has only the pass instruction in its body, which means it doesn't have any custom attributes or methods. Brackets after the name are optional if empty. We will print its type (__main__ is the name of the scope in which top-level code executes), and we are aware that, in the highlighted comment, we wrote object instead of class. It turns out that, as you can see by the result of that print statement, classes are actually objects. To be precise, they are instances of type. Explaining this concept would lead us to a talk about metaclasses and metaprogramming, advanced concepts that require a solid grasp of the fundamentals to be understood and are beyond the scope of this chapter. As usual, we mentioned it to leave a pointer for you, for when you are ready to explore more deeply.

Let's go back to the example: we created simp, an instance of the Simplest class. You can see that the syntax to create an instance is the same as the syntax for calling a function. Next, we print what type simp belongs to and we verify that simp is, in fact, an instance of Simplest. We'll show you a better way of doing this later on in the chapter.

Up to now, it's all very simple. What happens when we write `class ClassName(): pass`, though? Well, what Python does is create a class object and assign it a name. This is very similar to what happens when we declare a function using def.


### Class and object namespaces

After the class object has been created (which usually happens when the module is first imported), it basically represents a namespace. We can call that class to create its instances. Each instance inherits the class attributes and methods and is given its own namespace. We already know that in order to walk a namespace, all we need to do is to use the dot (.) operator.

Let's look at another example:

```python
#oop/class.namespaces.py
class Person:
    species = 'Human'

print(Person.species)  # Human
Person.alive = True  # Added dynamically!
print(Person.alive)  # True

man = Person()
print(man.species)  # Human (inherited)
print(man.alive)  # True (inherited)

Person.alive = False
print(man.alive)  # False (inherited)

man.name = 'Darth'
man.surname = 'Vader'
print(man.name, man.surname)  # Darth Vader
```

In the preceding example, we have defined a class attribute called species. Any name defined in the body of a class becomes an attribute that belongs to that class. In the code, we have also defined Person.alive, which is another class attribute. You can see that there is no restriction on accessing that attribute from the class. You can see that man, which is an instance of Person, inherits both of them, and reflects them instantly when they change. man also has two attributes that belong to its own namespace and are therefore called instance attributes: name and surname.

Class attributes are shared among all instances, while instance attributes are not; therefore, you should use class attributes to provide the states and behaviors to be shared by all instances and use instance attributes for data that will be specific to each individual object. 


### Attribute shadowing

When you search for an attribute on an object, if it is not found, Python extends the search to the attributes on the class that was used to create that object (and keeps searching until it's either found or the end of the inheritance chain is reached). This leads to an interesting shadowing behavior. Let's look at an example:

```python
# oop/class.attribute.shadowing.py
class Point:
    x = 10
    y = 7

p = Point()
print(p.x)  # 10 (from class attribute)
print(p.y)  # 7 (from class attribute)

p.x = 12  # p gets its own `x` attribute
print(p.x)  # 12 (now found on the instance)
print(Point.x)  # 10 (class attribute still the same)

del p.x  # we delete instance attribute
print(p.x)  # 10 (now search has to go again to find class attr)

p.z = 3  # let's make it a 3D point
print(p.z)  # 3

print(Point.z)
# AttributeError: type object 'Point' has no attribute 'z'
```

The preceding code is very interesting. We have defined a class called Point with two class attributes, x and y. When we create an instance of Point, p, you can see that we can print both x and y from the p namespace (p.x and p.y). What happens when we do that is that Python doesn't find any x or y attributes on the instance, and therefore searches the class, and finds them there.

Then we give p its own x attribute by assigning p.x = 12. This behavior may appear a bit weird at first, but if you think about it, it's exactly the same as what happens in a function that declares x = 12 when there is a global x = 10 outside (check out the section about scopes in Chapter 4, Functions, the Building Blocks of Code, for a refresher). We know that x = 12 won't affect the global one, and for class and instance attributes, it is exactly the same.

After assigning p.x = 12, when we print it, the search doesn't need to reach the class attributes because x is found on the instance, so we get 12 printed out. We also print Point.x, which refers to x in the class namespace, to show it's still 10.

Then, we delete x from the namespace of p, which means that, on the next line, when we print it again, Python will go again and search for it in the class, because it will no longer be found in the instance.

The last three lines show you that assigning attributes to an instance doesn't mean that they will be found in the class. Instances get whatever is in the class, but the opposite is not true. 

What do you think about putting the x and y coordinates as class attributes? Do you think it was a good idea? What if we created another instance of Point? Would that help to show why instance attributes can be very useful?


### The self argument

From within a class method, we can refer to an instance by means of a special argument, called self by convention. self is always the first attribute of an instance method. Let's examine this behavior together with how we can share not just attributes, but methods with all instances:

```python
# oop/class.self.py
class Square:
    side = 8
    def area(self):  # self is a reference to an instance
        return self.side ** 2

sq = Square()
print(sq.area())  # 64 (side is found on the class)
print(Square.area(sq))  # 64 (equivalent to sq.area())

sq.side = 10
print(sq.area())  # 100 (side is found on the instance)
```

Note how the area method is used by sq. The two calls, Square.area(sq) and sq.area(), are equivalent, and teach us how the mechanism works. Either you pass the instance to the method call (Square.area(sq)), which within the method will take the name self, or you can use a more comfortable syntax, sq.area(), and Python will translate that for you behind the scenes.

Let's look at a better example:

```python
# oop/class.price.py
class Price:
    def final_price(self, vat, discount=0):
        """Returns price after applying vat and fixed discount."""
        return (self.net_price * (100 + vat) / 100) - discount

p1 = Price()
p1.net_price = 100
print(Price.final_price(p1, 20, 10))  # 110 (100 * 1.2 - 10)
print(p1.final_price(20, 10))  # equivalent
```

The preceding code shows you that nothing prevents us from using arguments when declaring methods. We can use the exact same syntax as we used with the function, but we need to remember that the first argument will always be the instance that the method will be bound to. We don't need to necessarily call it self, but it's the convention, and this is one of the few cases where it's very important to abide by it.


### Initializing an instance

Have you noticed how, before calling p1.final_price(...) in the code above, we had to assign net_price to p1? There is a better way to do it. In other languages, this would be called a constructor, but in Python, it's not. It is actually an initializer, since it works on an already created instance, and therefore it's called __init__. It's a magic method, which is run right after the object is created. Python objects also have a __new__ method, which is the actual constructor. In practice, it's not so common to have to override it though; that is a technique that is mostly used when writing metaclasses which, as we mentioned, is a fairly advanced topic that we won't explore in the book. Let's now see an example of how to initialize objects in Python:

```python
# oop/class.init.py
class Rectangle:
    def __init__(self, side_a, side_b):
        self.side_a = side_a
        self.side_b = side_b

    def area(self):
        return self.side_a * self.side_b

r1 = Rectangle(10, 4)

print(r1.side_a, r1.side_b)  # 10 4
print(r1.area())  # 40

r2 = Rectangle(7, 3)
print(r2.area())  # 21
```

Things are finally starting to take shape. When an object is created, the __init__ method is automatically run for us. In this case, we wrote it so that when we create an object (by calling the class name like a function), we pass arguments to the creation call, like we would on any regular function call. The way we pass parameters follows the signature of the __init__ method, and therefore, in the two creation statements, 10 and 7 will be side_a for r1 and r2, respectively, while 4 and 3 will be side_b. You can see that the call to area() from r1 and r2 reflects that they have different instance arguments. Setting up objects in this way is much nicer and more convenient.

In this example, we also declared attributes at the instance level, rather than at the class level, because it made sense to do so.

## OOP is about code reuse

By now, it should be pretty clear: OOP is all about code reuse. We define a class, we create instances, and those instances use methods that are defined only in the class. They will behave differently according to how the instances have been set up by the initializer.

### Inheritance and composition

This is just half of the story though: OOP is much more powerful than just this. We have two main design constructs to use: inheritance and composition. Inheritance means that two objects are related by means of an Is-A type of relationship. On the other hand, composition means that two objects are related by means of a Has-A type of relationship. It's all very easy to explain with an example. Let's declare a few engine types:

```python
# oop/class_inheritance.py
class Engine:
    def start(self):
        pass

    def stop(self):
        pass

class ElectricEngine(Engine):  # Is-A Engine
    pass

class V8Engine(Engine):  # Is-A Engine
    pass
```

Then we want to declare some car types that will use those engines:

```python
class Car:
    engine_cls = Engine

    def __init__(self):
        self.engine = self.engine_cls()  # Has-A Engine

    def start(self):
        print(
            'Starting engine {0} for car {1}... Wroom, wroom!'
            .format(
                self.engine.__class__.__name__,
                self.__class__.__name__)
        )
        self.engine.start()

    def stop(self):
        self.engine.stop()

class RaceCar(Car):  # Is-A Car
    engine_cls = V8Engine

class CityCar(Car):  # Is-A Car
    engine_cls = ElectricEngine

class F1Car(RaceCar):  # Is-A RaceCar and also Is-A Car
    pass  # engine_cls same as parent

car = Car()
racecar = RaceCar()
citycar = CityCar()
f1car = F1Car()
cars = [car, racecar, citycar, f1car]

for car in cars:
    car.start()
```

Running the above prints the following:

Starting engine Engine for car Car... Wroom, wroom!
Starting engine V8Engine for car RaceCar... Wroom, wroom!
Starting engine ElectricEngine for car CityCar... Wroom, wroom!
Starting engine V8Engine for car F1Car... Wroom, wroom!

The preceding example shows you both the Is-A and Has-A types of relationships between objects. First of all, let's consider Engine. It's a simple class that has two methods, start and stop. We then define ElectricEngine and V8Engine, which both inherit from Engine. You can see that by the fact that when we define them, we put Engine within the brackets after the class name.

This means that both ElectricEngine and V8Engine inherit attributes and methods from the Engine class, which is said to be their base class. The same happens with cars. Car is a base class for both RaceCar and CityCar. RaceCar is also the base class for F1Car. Another way of saying this is that F1Car inherits from RaceCar, which inherits from Car. Therefore, F1Car Is-A RaceCar, and RaceCar Is-A Car. Because of the transitive property, we can say that F1Car Is-A Car as well. CityCar, too, Is-A Car. When we define class A(B): pass, we say A is the child of B, and B is the parent of A. The parent and base classes are synonyms, and so are child of and derived from. Also, we say that a class inherits from another class, or that it extends it.

This is the inheritance mechanism.

Let us now go back to the code. Each class has a class attribute, engine_cls, which is a reference to the engine class we want to assign to each type of car. Car has a generic Engine, while the two race cars have a powerful V8 engine, and the city car has an electric one.

When a car is created in the initializer method, __init__(), we create an instance of whatever engine class is assigned to the car, and set it as the engine instance attribute.

It makes sense to have engine_cls shared among all class instances because it's quite likely that all instances of the same car class will have the same kind of engine. On the other hand, it wouldn't be good to have a single engine (an instance of any Engine class) as a class attribute because we would be sharing one engine among all instances, which is incorrect.

The type of relationship between a car and its engine is a Has-A type. A car Has-A engine. This aspect is called composition, and reflects the fact that objects can be made of many other objects. A car Has-A engine, gears, wheels, a frame, doors, seats, and so on.

When designing OOP code, it is important to describe objects in this way so that we can use inheritance and composition correctly, to structure our code in the best way.

Notice how we had to avoid having dots in the class_inheritance.py script name, as dots in module names make imports difficult. Most modules in the source code of the book are meant to be run as standalone scripts, so we chose to add dots to enhance readability when possible, but in general, you want to avoid dots in your module names.

Before we leave this paragraph, let's verify the correctness of what we stated above, with another example:

```python
# oop/class.issubclass.isinstance.py
from class_inheritance import Car, RaceCar, F1Car

car = Car()
racecar = RaceCar()
f1car = F1Car()
cars = [(car, 'car'), (racecar, 'racecar'), (f1car, 'f1car')]
car_classes = [Car, RaceCar, F1Car]

for car, car_name in cars:
    for class_ in car_classes:
        belongs = isinstance(car, class_)
        msg = 'is a' if belongs else 'is not a'
        print(car_name, msg, class_.__name__)

""" Prints:
car is a Car
car is not a RaceCar
car is not a F1Car
racecar is a Car
racecar is a RaceCar
racecar is not a F1Car
f1car is a Car
f1car is a RaceCar
f1car is a F1Car
"""
```

As you can see, car is just an instance of Car, while racecar is an instance of RaceCar (and of Car, by extension) and f1car is an instance of F1Car (and of both RaceCar and Car, by extension). Similarly, a banana is an instance of Banana. But, also, it is a Fruit. Also, it is Food, right? This is the same concept. To check whether an object is an instance of a class, use the isinstance function. It is recommended over sheer type comparison (type(object) is Class).

Notice we have left out the prints you get when instantiating the cars. We saw them in the previous example.

Let's also check inheritance. The same setup, but different logic in the for loops:

```python
# oop/class.issubclass.isinstance.py
for class1 in car_classes:
    for class2 in car_classes:
        is_subclass = issubclass(class1, class2)
        msg = '{0} a subclass of'.format(
            'is' if is_subclass else 'is not')
        print(class1.__name__, msg, class2.__name__)

""" Prints:
Car is a subclass of Car
Car is not a subclass of RaceCar
Car is not a subclass of F1Car
RaceCar is a subclass of Car
RaceCar is a subclass of RaceCar
RaceCar is not a subclass of F1Car
F1Car is a subclass of Car
F1Car is a subclass of RaceCar
F1Car is a subclass of F1Car
"""
```

Interestingly, we learn that a class is a subclass of itself. Check the output of the preceding example to see that it matches the explanation we provided.

One thing to notice about conventions is that class names are always written using CapWords, which means ThisWayIsCorrect, as opposed to functions and methods, which are written in snake case, like this_way_is_correct. Also, when in the code you want to use a name that clashes with a Python-reserved keyword or a built- in function or class, the convention is to add a trailing underscore to the name. In the first for loop example, we are looping through the class names using for class_ in ... because class is a reserved word. But you already knew all this because you have thoroughly studied PEP 8, right?

To help you picture the difference between Is-A and Has-A, take a look at the following diagram:

Figure 6.1: Is-A versus Has-A relationships


### Accessing a base class

We've already seen class declarations, such as class ClassA: pass and class ClassB(BaseClassName): pass. When we don't specify a base class explicitly, Python will set the special object class as the base class for the one we're defining. Ultimately, all classes derive from object. Please remember that, if you don't specify a base class, brackets are optional and in practice are never used.

Therefore, writing class A: pass or class A(): pass or class A(object): pass is exactly the same thing. The object class is a special class in that it hosts the methods that are common to all Python classes, and it doesn't allow you to set any attributes on it.

Let's see how we can access a base class from within a class:

```python
# oop/super.duplication.py
class Book:
    def __init__(self, title, publisher, pages):
        self.title = title
        self.publisher = publisher
        self.pages = pages

class Ebook(Book):
    def __init__(self, title, publisher, pages, format_):
        self.title = title
        self.publisher = publisher
        self.pages = pages
        self.format_ = format_
```

Take a look at the preceding code. Three of the input parameters for Book are duplicated in Ebook. This is quite bad practice because we now have two sets of instructions that are doing the same thing. Moreover, any change in the signature of Book.__init__() will not be reflected in Ebook. We know that Ebook Is-A Book, and therefore we probably want changes to be reflected in the child classes.

Let's see one way to fix this issue:

```python
# oop/super.explicit.py
class Book:
    def __init__(self, title, publisher, pages):
        self.title = title
        self.publisher = publisher
        self.pages = pages

class Ebook(Book):
    def __init__(self, title, publisher, pages, format_):
        Book.__init__(self, title, publisher, pages)
        self.format_ = format_

ebook = Ebook(
    'Learn Python Programming', 'Packt Publishing', 500, 'PDF')
print(ebook.title)  # Learn Python Programming
print(ebook.publisher)  # Packt Publishing
print(ebook.pages)  # 500
print(ebook.format_)  # PDF
```

Now, that's better. We have removed that nasty duplication. Basically, we tell Python to call the __init__() method of the Book class; we feed self to that call, making sure that we bind that call to the present instance.

If we modify the logic within the __init__() method of Book, we don't need to touch Ebook; it will automatically adapt to the change.

This approach is good, but we can still do a bit better. Say that we change the name of Book to Liber, because we've fallen in love with Latin. We would then have to change the __init__() method of Ebook to reflect that change. This can be avoided by using super:

```python
# oop/super.implicit.py
class Book:
    def __init__(self, title, publisher, pages):
        self.title = title
        self.publisher = publisher
        self.pages = pages

class Ebook(Book):
    def __init__(self, title, publisher, pages, format_):
        super().__init__(title, publisher, pages)
        # Another way to do the same thing is:
        # super(Ebook, self).__init__(title, publisher, pages)
        self.format_ = format_

ebook = Ebook(
    'Learn Python Programming', 'Packt Publishing', 500, 'PDF')
print(ebook.title) # Learn Python Programming
print(ebook.publisher) # Packt Publishing
print(ebook.pages) # 500
print(ebook.format_) # PDF
```

super() is a function that returns a proxy object that delegates method calls to a parent or sibling class.

Two classes are siblings if they share the same parents.

In this case, super() will delegate that call to __init__() of the Book class, and the beauty of this approach is that now we're free to change Book to Liber without having to touch the logic in the __init__() method of Ebook.

Now that we know how to access a base class from its child, let's explore Python's multiple inheritance.


### Multiple inheritance

Apart from composing a class using more than one base class, what is of interest here 
is how an attribute search is performed. Take a look at the following diagram:


Figure 6.2: A class inheritance diagram

As you can see, Shape and Plotter act as base classes for all the others. Polygon inherits directly from them, RegularPolygon inherits from Polygon, and both RegularHexagon and Square inherit from RegularPolygon. Note also that Shape and Plotter implicitly inherit from object, so we therefore have what is known as a diamond or, in simpler terms, more than one path to reach a base class. We'll see why this matters in a few moments. Let's translate the diagram into code:

```python
# oop/multiple.inheritance.py
class Shape:
    geometric_type = 'Generic Shape'
    def area(self):  # This acts as placeholder for the interface
        raise NotImplementedError
    
    def get_geometric_type(self):
        return self.geometric_type

class Plotter:
    def plot(self, ratio, topleft):
        # Imagine some nice plotting logic here...
        print('Plotting at {}, ratio {}.'.format(
            topleft, ratio))

class Polygon(Shape, Plotter):  # base class for polygons
    geometric_type = 'Polygon'

class RegularPolygon(Polygon):  # Is-A Polygon
    geometric_type = 'Regular Polygon'
    def __init__(self, side):
        self.side = side

class RegularHexagon(RegularPolygon):  # Is-A RegularPolygon
    geometric_type = 'RegularHexagon'
    def area(self):
        return 1.5 * (3 ** .5 * self.side ** 2)

class Square(RegularPolygon):  # Is-A RegularPolygon
    geometric_type = 'Square'
    def area(self):
        return self.side * self.side

hexagon = RegularHexagon(10)
print(hexagon.area())  # 259.8076211353316
print(hexagon.get_geometric_type())  # RegularHexagon
hexagon.plot(0.8, (75, 77))  # Plotting at (75, 77), ratio 0.8.

square = Square(12)
print(square.area())  # 144
print(square.get_geometric_type())  # Square
square.plot(0.93, (74, 75))  # Plotting at (74, 75), ratio 0.93.
```

Take a look at the preceding code: the Shape class has one attribute, geometric_type, and two methods: area() and get_geometric_type(). It's quite common to use base classes (such as Shape, in our example) to define an interface, a set of methods for which children must provide an implementation. There are different and better ways to do this, but we want to keep this example as simple as possible.

We also have the Plotter class, which adds the plot() method, thereby providing plotting capabilities for any class that inherits from it. Of course, the plot() implementation is just a dummy print in this example. The first interesting class is Polygon, which inherits from both Shape and Plotter.

There are many types of polygons, one of which is the regular one, which is both equiangular (all angles are equal) and equilateral (all sides are equal), so we create the RegularPolygon class that inherits from Polygon. For a regular polygon, where all sides are equal, we can implement a simple __init__() method, which just takes the length of the side. We create the RegularHexagon and Square classes, which both inherit from RegularPolygon.

This structure is quite long, but hopefully gives you an idea of how to specialize the classification of your objects when you design the code.

Now, please take a look at the last eight lines. Note that when we call the area() method on hexagon and square, we get the correct area for both. This is because they both provide the correct implementation logic for it. Also, we can call get_ geometric_type() on both of them, even though it is not defined on their classes, and Python has to go all the way up to Shape to find an implementation for it. Note that, even though the implementation is provided in the Shape class, the self.geometric_ type() used for the return value is correctly taken from the caller instance.

The plot() method calls are also interesting and show you how you can enrich your objects with capabilities they wouldn't otherwise have. This technique is very popular in web frameworks such as Django (which we will explore briefly in Chapter 14, Introduction to API Development), which provides special classes called mixins, whose capabilities you can just use out of the box. All you have to do is to define the desired mixin as one of the base classes for your own, and that's it.

Multiple inheritance is powerful, but can also get really messy, so we need to make sure we understand what happens when we use it.


### Method resolution order

By now, we know that when we ask for someobject.attribute and attribute is not found on that object, Python starts searching in the class that someobject was created from. If it's not there either, Python searches up the inheritance chain until either attribute is found or the object class is reached. This is quite simple to understand if the inheritance chain is only made of single-inheritance steps, which means that classes have only one parent, all the way up to object. However, when multiple inheritance is involved, there are cases when it's not straightforward to predict what will be the next class that will be searched for if an attribute is not found. Python provides a way to always know the order in which classes are searched on attribute lookup: the method resolution order (MRO).

The MRO is the order in which base classes are searched for a member during lookup. Since version 2.3, Python uses an algorithm called C3, which guarantees monotonicity.

In Python 2.2, new-style classes were introduced. The way you write a new-style class in Python 2.* is to define it with an explicit object base class. Classic classes did not inherit from object and have been removed in Python 3. One of the differences between classic and new-style classes in Python 2.* is that new-style classes are searched with the new MRO.

With regard to the previous example, let's see the MRO for the Square class:

```python
# oop/multiple.inheritance.py
print(square.__class__.__mro__)
# prints:
# (<class '__main__.Square'>, <class '__main__.RegularPolygon'>,
# <class '__main__.Polygon'>, <class '__main__.Shape'>,
# <class '__main__.Plotter'>, <class 'object'>)
```

To get to the MRO of a class, we can go from the instance to its __class__ attribute, and from that to its __mro__ attribute. Alternatively, we could have used Square.__ mro__, or Square.mro() directly, but if you have to do it from an instance, you'll have to derive its class dynamically.

Note that the only point of doubt is the bifurcation after Polygon, where the inheritance chain breaks into two ways: one leads to Shape and the other to Plotter. We know by scanning the MRO for the Square class that Shape is searched before Plotter.

Why is this important? Well, consider the following code:

```python
# oop/mro.simple.py
class A:
    label = 'a'

class B(A):
    label = 'b'

class C(A):
    label = 'c'

class D(B, C):
    pass

d = D()
print(d.label)  # Hypothetically this could be either 'b' or 'c'
```

Both B and C inherit from A, and D inherits from both B and C. This means that the lookup for the label attribute can reach the top (A) through either B or C. According to which is reached first, we get a different result.

So, in the preceding example, we get 'b', which is what we were expecting, since B is the leftmost one among the base classes of D. But what happens if we remove the label attribute from B? This would be a confusing situation: will the algorithm go all the way up to A or will it get to C first? Let's find out:

```python
# oop/mro.py
class A:
    label = 'a'

class B(A):
    pass  # was: label = 'b'

class C(A):
    label = 'c'

class D(B, C):
    pass

d = D()
print(d.label)  # 'c'
print(d.__class__.mro())  # notice another way to get the MRO
# prints:
# [<class '__main__.D'>, <class '__main__.B'>,
# <class '__main__.C'>, <class '__main__.A'>, <class 'object'>]
```

So, we learn that the MRO is D-B-C-A-object, which means that when we ask for d.label, we get 'c', which is correct.

In day-to-day programming, it is not common to have to deal with the MRO, but we felt it was important to at least mention it in this paragraph so that, should you get entangled in a complex mixins structure, you will be able to find your way out of it.


## Class and static methods

So far, we have coded classes with attributes in the form of data and instance methods, but there are two other types of methods that we can place inside a class: static methods and class methods.


### Static methods

As you may recall, when you create a class object, Python assigns a name to it. That name acts as a namespace, and sometimes it makes sense to group functionalities under it. Static methods are perfect for this use case. Unlike instance methods, they are not passed any special argument, and therefore we don't need to create an instance of the class in order to call them. Let's look at an example of an imaginary StringUtil class:

```python
# oop/static.methods.py
class StringUtil:

    @staticmethod
    def is_palindrome(s, case_insensitive=True):
        # we allow only letters and numbers
        s = ''.join(c for c in s if c.isalnum())  # Study this!
        # For case insensitive comparison, we lower-case s
        if case_insensitive:
            s = s.lower()
        for c in range(len(s) // 2):
            if s[c] != s[-c -1]:
                return False
          return True

    @staticmethod
    def get_unique_words(sentence):
        return set(sentence.split())

print(StringUtil.is_palindrome(
    'Radar', case_insensitive=False))  # False: Case Sensitive
print(StringUtil.is_palindrome('A nut for a jar of tuna'))  # True
print(StringUtil.is_palindrome('Never Odd, Or Even!'))  # True
print(StringUtil.is_palindrome(
    'In Girum Imus Nocte Et Consumimur Igni')  # Latin! Show-off!
)  # True

print(StringUtil.get_unique_words(
    'I love palindromes. I really really love them!'))
# {'them!', 'palindromes.', 'I', 'really', 'love'}

```

The preceding code is quite interesting. First of all, we learn that static methods are created by simply applying the staticmethod decorator to them. You can see that they aren't passed any special argument so, apart from the decoration, they really just look like functions.

We have a class, StringUtil, that acts as a container for functions. Another approach would be to have a separate module with functions inside. It's really a matter of preference most of the time.

The logic inside is_palindrome() should be straightforward for you to understand by now, but, just in case, let's go through it. First, we remove all characters from s that are neither letters nor numbers. In order to do this, we use the join() method of a string object (an empty string object, in this case). By calling join() on an empty string, the result is that all elements in the iterable you pass to join() will be concatenated together. We feed join() a generator expression that says to take any character from s if the character is either alphanumeric or a number. This is because, in palindrome sentences, we want to discard anything that is not a character or a number.

We then lowercase s if case_insensitive is True, and then we proceed to check whether it is a palindrome. To do this, we compare the first and last characters, then the second and the second to last, and so on. If, at any point, we find a difference, it means the string isn't a palindrome, and therefore we can return False. On the other hand, if we exit the for loop normally, it means no differences were found, and we can therefore say the string is a palindrome.

Notice that this code works correctly regardless of the length of the string; that is, if the length is odd or even. The measure len(s) // 2 reaches half of s, and if s is an odd number of characters long, the middle one won't be checked (for instance, in RaDaR, D is not checked), but we don't care; it would be compared to itself, so it's always passing that check.

The get_unique_words() method is much simpler: it just returns a set to which we feed a list with the words from a sentence. The set class removes any duplication for us, so we don't need to do anything else.

The StringUtil class provides us with a nice container namespace for methods that are meant to work on strings. We could have coded a similar example with a MathUtil class, and some static methods to work on numbers, but we wanted to show you something different.


### Class methods

Class methods are slightly different from static methods in that, like instance methods, they also take a special first argument, but in this case, it is the class object itself, rather than the instance. A very common use case for coding class methods is to provide factory capability to a class, which means to have alternative ways to create instances of the class. Let's see an example:

```python
# oop/class.methods.factory.py
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    @classmethod
    def from_tuple(cls, coords):  # cls is Point
        return cls(*coords)

    @classmethod
    def from_point(cls, point):  # cls is Point
        return cls(point.x, point.y)

p = Point.from_tuple((3, 7))
print(p.x, p.y)  # 3 7
q = Point.from_point(p)
print(q.x, q.y)  # 3 7
```
Let's see an example by refactoring the StringUtil class:

```python
# oop/class.methods.split.py
class StringUtil:

    @classmethod
    def is_palindrome(cls, s, case_insensitive=True):
        s = cls._strip_string(s)
        # For case insensitive comparison, we lower-case s
        if case_insensitive:
            s = s.lower()
        return cls._is_palindrome(s)

    @staticmethod
    def _strip_string(s):
        return ''.join(c for c in s if c.isalnum())

    @staticmethod
    def _is_palindrome(s):
        for c in range(len(s) // 2):
            if s[c] != s[-c -1]:
                return False
        return True

    @staticmethod
    def get_unique_words(sentence):
        return set(sentence.split())

print(StringUtil.is_palindrome('A nut for a jar of tuna'))  # True
print(StringUtil.is_palindrome('A nut for a jar of beans'))  # False
```

Compare this code with the previous version. First of all, note that even though is_palindrome() is now a class method, we call it in the same way we were calling it when it was a static one. The reason why we changed it to a class method is that after factoring out a couple of pieces of logic (_strip_string and _is_palindrome), we need to get a reference to them, and if we have no cls in our method, the only option would be to call them by using the name of the class itself, like so: StringUtil._ strip_string(...) and StringUtil._is_palindrome(...), which is not good practice, because we would hardcode the class name in the is_palindrome() method, thereby putting ourselves in the position of having to modify it whenever we want to change the class name. Using cls means it will act as the class name, which means our code won't need any modifications should the class name change.

Notice how the new logic reads much better than the previous version. Moreover, notice that, by naming the factored-out methods with a leading underscore, we are hinting that those methods are not supposed to be called from outside the class, but this will be the subject of the next paragraph.

---

### Private methods and name mangling

If you have any background with languages like Java, C#, or C++, then you know they allow the programmer to assign a privacy status to attributes (both data and methods). Each language has its own slightly different flavor for this, but the gist is that public attributes are accessible from any point in the code, while private ones are accessible only within the scope they are defined in.

In Python, there is no such thing. Everything is public; therefore, we rely on conventions and, for privacy, on a mechanism called name mangling. The convention is as follows: if an attribute's name has no leading underscores, it is considered public. This means you can access it and modify it freely. When the name has one leading underscore, the attribute is considered private, which means it's probably meant to be used internally and you should not modify it, or call it from the outside. A very common use case for private attributes is helper methods that are supposed to be used by public ones (possibly in call chains in conjunction with other methods), and internal data, such as scaling factors, or any other data that we would ideally put in a constant (a variable that cannot change, but, surprise, surprise, Python doesn't have those either).

This characteristic usually scares people from other backgrounds off; they feel threatened by the lack of both privacy and constraints. To be honest, in our professional experience with Python, we've never heard anyone screaming "oh my God, we have a terrible bug because Python lacks private attributes!" Not once, we swear.

That said, the call for privacy actually makes sense because without it, you risk introducing bugs into your code for real. Let us show you what we mean:

```python
# oop/private.attrs.py
class A:
    def __init__(self, factor):
        self._factor = factor

    def op1(self):
        print('Op1 with factor {}...'.format(self._factor))

class B(A):
    def op2(self, factor):
        self._factor = factor
        print('Op2 with factor {}...'.format(self._factor))

obj = B(100)
obj.op1()    # Op1 with factor 100...
obj.op2(42)  # Op2 with factor 42...
obj.op1()    # Op1 with factor 42... <- This is BAD
```

In the preceding code, we have an attribute called _factor, and let's pretend it's so important that it isn't modified at runtime after the instance is created because op1() depends on it to function correctly. We've named it with a leading underscore, but the issue here is that when we call obj.op2(42), we modify it, and this is reflected in subsequent calls to op1().

Let's fix this undesired behavior by adding another leading underscore:

```python

# oop/private.attrs.fixed.py
class A:
    def __init__(self, factor):
        self.__factor = factor

    def op1(self):
        print('Op1 with factor {}...'.format(self.__factor))

class B(A):
    def op2(self, factor):
        self.__factor = factor
        print('Op2 with factor {}...'.format(self.__factor))

obj = B(100)
obj.op1()    # Op1 with factor 100...
obj.op2(42)  # Op2 with factor 42...
obj.op1()    # Op1 with factor 100... <- Wohoo! Now it's GOOD!

```

Wow, look at that! Now it's working as desired. Python is kind of magic and in this case, what is happening is that the name-mangling mechanism has kicked in.

Name mangling means that any attribute name that has at least two leading underscores and at most one trailing underscore, such as __my_attr, is replaced with a name that includes an underscore and the class name before the actual name, such as _ClassName__my_attr.

This means that when you inherit from a class, the mangling mechanism gives your private attribute two different names in the base and child classes so that name collision is avoided. Every class and instance object stores references to their attributes in a special attribute called __dict__, so let's inspect obj.__dict__ to see name mangling in action:

```python
# oop/private.attrs.py
print(obj.__dict__.keys())
# dict_keys(['_factor'])
```

This is the _factor attribute that we find in the problematic version of this example, 
but look at the one that is using __factor:

```python
# oop/private.attrs.fixed.py
print(obj.__dict__.keys())
# dict_keys(['_A__factor', '_B__factor'])
```

See? obj has two attributes now, _A__factor (mangled within the A class), and _B__factor (mangled within the B class). This is the mechanism that ensures that when you do obj.__factor = 42, __factor in A isn't changed because you're actually touching _B__factor, which leaves _A__factor safe and sound.

If you're designing a library with classes that are meant to be used and extended by other developers, you will need to keep this in mind in order to avoid the unintentional overriding of your attributes. Bugs like these can be pretty subtle and hard to spot.


---

### The property decorator

Another thing that would be a crime not to mention is the property decorator. Imagine that you have an age attribute in a Person class and, at some point, you want to make sure that when you change its value, you're also checking that age is within a proper range, such as [18, 99]. You could write accessor methods, such as get_age() and set_age(...) (also called getters and setters), and put the logic there. get_age() will most likely just return age, while set_age(...) will set its value after checking its validity. The problem is that you may already have a lot of code accessing the age attribute directly, which means you're now up to some tedious refactoring. Languages like Java overcome this problem by using the accessor pattern basically by default. Many Java Integrated Development Environments (IDEs) autocomplete an attribute declaration by writing getter and setter accessor method stubs for you on the fly.

Python is smarter and does this with the property decorator. When you decorate a method with property, you can use the name of the method as if it were a data attribute. Because of this, it's always best to refrain from putting logic that would take a while to complete in such methods because, by accessing them as attributes, we are not expecting to wait.

Let's look at an example:

```python
# oop/property.py
class Person:
    def __init__(self, age):
        self.age = age  # anyone can modify this freely

class PersonWithAccessors:
    def __init__(self, age):
        self._age = age

    def get_age(self):
        return self._age

    def set_age(self, age):
        if 18 <= age <= 99:
            self._age = age
        else:
            raise ValueError('Age must be within [18, 99]')

class PersonPythonic:
    def __init__(self, age):
        self._age = age

    @property
    def age(self):
        return self._age

    @age.setter
    def age(self, age):
        if 18 <= age <= 99:
            self._age = age
        else:
            raise ValueError('Age must be within [18, 99]')

person = PersonPythonic(39)
print(person.age)  # 39 - Notice we access as data attribute

person.age = 42    # Notice we access as data attribute
print(person.age)  # 42
person.age = 100   # ValueError: Age must be within [18, 99]
```

The Person class may be the first version we write. Then we realize we need to put the range logic in place so, with another language, we would have to rewrite Person as the PersonWithAccessors class, and refactor all the code that was using Person.age. In Python, we rewrite Person as PersonPythonic (you normally wouldn't change the name, of course) so that the age is stored in a private _age variable, and we define property getters and setters using the decoration shown, which allows us to keep using the person instances as we were before. A getter is a method that is called when we access an attribute for reading. On the other hand, a setter is a method that is called when we access an attribute to write it. In other languages, such as Java, it's customary to define them as get_age() and set_age(int value), but we find the Python syntax much neater. It allows you to start writing simple code and refactor later on, only when you need it; there is no need to pollute your code with accessors only because they may be helpful in the future.

The property decorator also allows for read-only data (by not writing the setter counterpart) and for special actions when the attribute is deleted. Please refer to the official documentation to dig deeper.

---

### The cached_property decorator

One convenient use of properties is when we need to run some code in order to set up the object we want to use. For example, say we needed to connect to a database (or to an API). 

In both cases, we might have to set up a client object that knows how to talk to the database (or to the API). It is quite common to use a property, in these cases, so that we can hide away the complexity of having to set the client up, and we can just use it. Let us show you a simplified example:

```python
class Client:
    def __init__(self):
        print("Setting up the client...")

    def query(self, **kwargs):
        print(f"Performing a query: {kwargs}")

class Manager:
    @property
    def client(self):
        return Client()

    def perform_query(self, **kwargs):
        return self.client.query(**kwargs)

```

In the preceding example, we have a dummy Client class, which prints the string "Setting up the client…" every time we create a new instance. It also has a pretend query method, that prints a string as well. We then have a class, Manager, which has a client property, which creates a new instance of Client every time it is called (for example, by a call to perform_query). 

If we were to run this code, we would notice that every time we call perform_query on the manager, we see the string "Setting up the client…" being printed. When creating a client is expensive, this code would be wasting resources, so it might be better to cache that client, like this:

```python

class ManualCacheManager:
    @property
    def client(self):
        if not hasattr(self, '_client'):
            self._client = Client()
        return self._client

```

The ManualCacheManager class is a bit smarter: the client property first checks if the attribute _client is on the class, by calling the built-in hasattr function. If not, it assigns _client to a new instance of Client. Finally, it simply returns it. Repeatedly accessing the client property on this class will only create one instance of Client, the first time. From the second call on, _client is returned with no need to create a new one.

This is such a common need that, in Python 3.8, the functools module added the cached_property decorator. The beauty of using that, instead of our manual solution, is that in case we need to refresh the client, we can simply delete the client property, and the next time we call it, it will recreate a brand new Client for us. Let's see an example:

```python

from functools import cached_property

class CachedPropertyManager:
    @cached_property
    def client(self):
        return Client()

    def perform_query(self, **kwargs):
        return self.client.query(**kwargs)

manager = CachedPropertyManager()
manager.perform_query(object_id=42)
manager.perform_query(name_ilike='%Python%')
del manager.client  # This causes a new Client on next call
manager.perform_query(age_gte=18)

```

Running this code gives the following result:

```bash

$ python cached.property.py
Setting up the client...                         # New Client
Performing a query: {'object_id': 42}            # first query
Performing a query: {'name_ilike': '%Python%'}   # second query
Setting up the client...                         # Another Client
Performing a query: {'age_gte': 18}              # Third query

```

As you can see, it's only after we manually delete the manager.client that we get a new one when we invoke manager.perform_query again.

Python 3.9 also introduces a cache decorator, which can be used in conjunction with the property decorator, to cover scenarios for which cached_property is not suitable. As always, we encourage you to read up on all the details in the official Python documentation and experiment.


---

### Operator overloading

We find Python's approach to operator overloading to be brilliant. To overload an operator means to give it a meaning according to the context in which it is used. For example, the + operator means addition when we deal with numbers, but concatenation when we deal with sequences.

In Python, when you use operators, you're most likely calling the special methods of some objects behind the scenes. For example, the a[k] call on a dictionary roughly translates to type(a).__getitem__(a, k). We can override these special methods for our purposes.

As an example, let's create a class that stores a string and evaluates to True if '42' is part of that string, and False otherwise. Also, let's give the class a length property that corresponds to that of the stored string:

```python

# oop/operator.overloading.py
class Weird:

    def __init__(self, s):
        self._s = s

    def __len__(self):
        return len(self._s)

    def __bool__(self):
        return '42' in self._s

weird = Weird('Hello! I am 9 years old!')
print(len(weird))  # 24
print(bool(weird))  # False

weird2 = Weird('Hello! I am 42 years old!')
print(len(weird2))  # 25
print(bool(weird2))  # True

```

That was fun, wasn't it? For the complete list of magic methods that you can override to provide your custom implementation of operators for your classes, please refer to the Python data model in the official documentation.

---

### Polymorphism – a brief overview

The word polymorphism comes from the Greek polys (many, much) and morphē (form, shape), and its meaning is the provision of a single interface for entities of different types.

In our car example, we call engine.start(), regardless of what kind of engine it is. As long as it exposes the start method, we can call it. That's polymorphism in action.

In other languages, such as Java, in order to give a function the ability to accept different types and call a method on them, those types need to be coded in such a way that they share an interface. In this way, the compiler knows that the method will be available regardless of the type of the object the function is fed (as long as it extends the specific interface, of course).

In Python, things are different. Polymorphism is implicit, and nothing prevents you from calling a method on an object; therefore, technically, there is no need to implement interfaces or other patterns. There is a special kind of polymorphism called ad hoc polymorphism, which is what we saw in the last section on operator overloading. This is the ability of an operator to change shape according to the type of data it is fed.

Polymorphism also allows Python programmers to simply use the interface (methods and properties) exposed from an object rather than having to check which class it was instantiated from. This allows the code to be more compact and feel more natural.

We cannot spend too much time on polymorphism, but we encourage you to check it out by yourself; it will expand your understanding of OOP. Good luck!

---

### Data classes

Before we leave the OOP realm, there is one last thing we want to mention: data classes. Introduced in Python 3.7 by PEP 557 (https://www.python.org/dev/peps/ pep-0557/), they can be described as mutable named tuples with defaults. You can brush up on named tuples in Chapter 2, Built-In Data Types. Let's dive straight into an example:

```python

# oop/dataclass.py
from dataclasses import dataclass

@dataclass
class Body:
    '''Class to represent a physical body.'''
    name: str
    mass: float = 0.  # Kg
    speed: float = 1.  # m/s

    def kinetic_energy(self) -> float:
        return (self.mass * self.speed ** 2) / 2

body = Body('Ball', 19, 3.1415)
print(body.kinetic_energy())  # 93.755711375 Joule
print(body)  # Body(name='Ball', mass=19, speed=3.1415)

```

In the previous code, we have created a class to represent a physical body, with one method that allows us to calculate its kinetic energy (using the renowned formula Ek=½mv2). Notice that name is supposed to be a string, while mass and speed are both floats, and both are given a default value. It's also interesting that we didn't have to write any __init__() method; it's done for us by the dataclass decorator, along with methods for comparison and for producing the string representation of the object (implicitly called on the last line by print).

You can read all the specifications in PEP 557 if you are curious, but for now, just remember that data classes might offer a nicer, slightly more powerful alternative to named tuples, in case you need it.

---

## Writing a custom iterator

Now we have all the tools to appreciate how we can write our own custom iterator. Let's first define an iterable and an iterator:

- Iterable: An object is said to be iterable if it's capable of returning its members one at a time. Lists, tuples, strings, and dictionaries are all iterables. Custom objects that define either of the __iter__() or__getitem__() methods are also iterables.
 
- Iterator: An object is said to be an iterator if it represents a stream of data. A custom iterator is required to provide an implementation for the __iter__() method that returns the object itself, and an implementation for the __next__ () method that returns the next item of the data stream until the stream is exhausted, at which point all successive calls to __next__() simply raise the StopIteration exception. Built-in functions, such as iter() and next(), are mapped to call the __iter__() and __next__() methods on an object, behind the scenes.

Let's write an iterator that returns all the odd characters from a string first, and then the even ones:

```python
# iterators/iterator.py
class OddEven:
    def __init__(self, data):
        self._data = data
        self.indexes = (list(range(0, len(data), 2)) +
            list(range(1, len(data), 2)))

    def __iter__(self):
        return self

    def __next__(self):
        if self.indexes:
            return self._data[self.indexes.pop(0)]
        raise StopIteration

oddeven = OddEven('ThIsIsCoOl!')

print(''.join(c for c in oddeven))  # TIICO!hssol

oddeven = OddEven('CiAo')  # or manually...
it = iter(oddeven)  # this calls oddeven.__iter__ internally
print(next(it))  # C
print(next(it))  # A
print(next(it))  # i
print(next(it))  # o
```

So, we needed to provide an implementation for __iter__() that returned the object itself, and then one for __next__(). Let's go through it. What needs to happen is the return of _data[0], _data[2], _data[4], ..., _data[1], _data[3], _data[5], ... until we have returned every item in the data. To do that, we prepare a list of indexes, such as [0, 2, 4, 6, ..., 1, 3, 5, ...], and while there is at least an element in it, we pop the first one out and return the element from the data that is at that position, thereby achieving our goal. When indexes is empty, we raise StopIteration, as required by the iterator protocol.

There are other ways to achieve the same result, so go ahead and try to code a different one yourself. Make sure that the end result works for all edge cases, empty sequences, sequences of lengths of 1, 2, and so on.

---

## Summary

In this chapter, we looked at decorators, discovered the reasons for having them, and covered a few examples using one or more at the same time. We also saw decorators that take arguments, which are usually used as decorator factories.

We have scratched the surface of object-oriented programming in Python. We covered all the basics, so you should now be able to understand the code that will come in future chapters. We talked about all kinds of methods and attributes that you can write in a class; we explored inheritance versus composition, method overriding, properties, operator overloading, and polymorphism.

At the end, we very briefly touched base on iterators, so now you understand generators more deeply.

In the next chapter, we're going to learn about exceptions and context managers.
