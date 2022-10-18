# Introduction to API Development

    "The one goal of compassionate communication is to help others suffer less."

    – Thich Nhat Hanh

In this chapter, we are going to learn about the concept of an Application Programming Interface, or API. We are going to explore this important topic by working on a railway project together. As we go along, we will take the opportunity to also touch briefly upon the following topics:

•  HTTP protocol, requests, and responses
•  Python type hinting
•  The Django web framework

There are entire books dedicated to API design, so it would be impossible for us to tell you everything you need to know about this subject within a single chapter. This consideration brought us to the decision of adopting FastAPI as the main technology for this project. It is a well-designed framework that has allowed us to create an API with clean, concise, and expressive code, which we think is quite apt for the purpose of this book.

We heartily suggest you download and take a look at the source code for this chapter, as it will make it easier for you to wrap your head around the project.

We will also discuss the concept of APIs in a more abstract way, and we hope you will want to study it in depth. As a developer, in fact, you will have to deal with APIs, and likely work on some, throughout your career. Technology changes all the time, and there are plenty of choices available at the moment. It is therefore important to put some effort into learning some of the abstract concepts and ideas we need to design an API, so that we will not feel too lost when it is time to adopt another framework.

Let's now start by spending a moment on the foundation upon which all this is based.

## What is the Web?

The World Wide Web (WWW), or simply the Web, is a way of accessing information through the use of a medium called the Internet. The Internet is a huge network of networks, a networking infrastructure. Its purpose is to connect billions of devices together, all around the globe, so that they can communicate with one another. Information travels through the Internet in a rich variety of languages, called protocols, that allow different devices to speak the same tongue in order to share content.

The Web is an information-sharing model, built on top of the Internet, which employs the Hypertext Transfer Protocol (HTTP) as a basis for data communication. The Web, therefore, is just one of several different ways information can be exchanged over the Internet; email, instant messaging, news groups, and so on, all rely on different protocols.

### How does the Web work?

In a nutshell, HTTP is an asymmetric request-response client-server protocol. An HTTP client sends a request message to an HTTP server. The server, in turn, returns a response message. In other words, HTTP is a pull protocol in which the client pulls information from the server (as opposed to a push protocol, in which the server pushes information down to the client). Take a look at the diagram in Figure 14.1:

Figure 14.1: A simplified depiction of the HTTP protocol

HTTP is transmitted via the Transmission Control Protocol/Internet Protocol, or TCP/IP, which provides the tools for a reliable communication exchange over the Internet.

An important feature of the HTTP protocol is that it's stateless. This means that the current request has no knowledge about what happened in previous requests. This is a technical limitation that exists for very good reasons but, in practice, it is actually possible to browse a website with the illusion of being logged in. When a user logs in, a token of user information is saved (most often on the client side, in special files called cookies) so that each request the user makes carries the means for the server to recognize the user and provide a custom interface by showing their name, keeping their basket populated, and so on.

Even though it's very interesting, we're not going to delve into the rich details of HTTP and how it works. HTTP defines a set of methods—also known as verbs—to indicate the desired action to be performed on a given resource. Each of them is different, but some of them share some common features. We won't be discussing all of them here, only the ones we will use in our API:

•  GET: The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
•  POST: The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
•  PUT: The PUT method requests that the target resource creates or updates its state with the state defined by the representation enclosed in the request.
•  DELETE: The DELETE method requests that the target resource deletes its state.

Other methods are HEAD, CONNECT, OPTIONS, TRACE, and PATCH. For a comprehensive explanation of all of these, please refer to this page: https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol.

The API we are going to write, and the two consumers for it (a console-based one, and a simple local website) all work over HTTP, which means we'll have to write code to perform and handle HTTP requests and responses. We won't keep prepending "HTTP" to the terms "request" and "response" from now on, as we trust there won't be any confusion.

### Response status codes

One thing to know about HTTP responses is that they come with a status code, which expresses the outcome of the request in a concise way. Status codes consist of a number and a short description, like 404 Not Found, for example. You can check the complete list of HTTP status codes at https://en.wikipedia.org/wiki/List_of_HTTP_status_codes.

Status codes are classified in this way:

•  1xx informational response: The request was received, continuing process.
•  2xx successful: The request was successfully received, understood, and accepted.
•  3xx redirection: Further action needs to be taken in order to complete the request.
•  4xx client error: The request contains bad syntax or cannot be fulfilled.
•  5xx server error: The server failed to fulfil an apparently valid request.

When consuming the API, we will receive status codes in the responses, so it's important that you at least have an idea about what they mean.

## Type hinting: An overview

Before we start tackling the topic of APIs, let's take a look at Python's type hinting.

One of the reasons we chose FastAPI to create this project is that it is based on type hinting. Type hinting was introduced in Python 3.5, by PEP484 (https://www.python.org/dev/peps/pep-0484/). This PEP builds on top of another functionality, function annotations, that was introduced in Python 3.0 by PEP3107 (https://www.python.org/dev/peps/pep-3107/). PEP484 was strongly inspired by Mypy (http://mypy-lang.org), an optional static type checker for Python. 

Python is both a strongly typed and a dynamically typed language. Strong typing means that variables have a type, and the type matters when we perform operations on variables. Let us explain this concept with an example in an imaginary language:
```python

a = 7
b = "7"
a + b == 14
concatenate(a, b) == "77"

```

In this example, we imagine a language that is weakly typed (the opposite of strongly typed). As you can see, the lines between types blur, and the language understands that when we want to sum the integer 7 with the string "7", it should interpret the latter as a number, yielding 14 as a result of the sum.

On the other hand, a concatenation function called with an integer and a string will do the opposite, and cast the integer to a string, producing the string "77" as a result.

Python does not allow this type of behavior, so if we try to sum an integer with a string, or concatenate them, it will complain that their types are not compatible and therefore it cannot carry out the operation we requested. This is good news, as weakly typed languages suffer from sneaky issues that come from the way they approach types.

Dynamic typing means that the type of a variable is determined during runtime. This results in the ability for a variable to have different types at different times, during execution. Take a look at the next snippet, this time in Python:
```python

a = 7
a * 2 == 14
a = "Hello"
a * 2 == "HelloHello"

```

In this example, we can see how Python allows us to declare a as an integer and perform multiplication by 2 on it. Immediately after that, we re-declare a as a string, and the same instruction is now interpreted as a concatenation, because a's type is a string. We learned in Chapter 2, Built-In Data Types, that a is not really changing. What happens is that the name a is pointed to a different memory location, where the new object—the string "Hello", in this case—is located.

Other languages are different, in that they adopt static typing. Let's see an example of that:
```javascript

String greeting = "Hello";
int m = 7;
float pi = 3.141592;


```
In this example, in a statically typed pretend language, we see that the declaration of variables starts with the specification of their type. This means that they won't be able to ever change type for the whole duration of the program.

There are pros and cons to both approaches. Static typing allows the compiler to notify you about issues like trying to assign a number to a variable declared as a string, before runtime, when the software is being compiled, for example. This is something that Python cannot do because, being dynamically typed, it allows variables to change at will.

On the other hand, static typing can sometimes feel constrictive, whereas dynamic typing allows for more freedom and enables us to employ programming patterns that are not implementable in statically typed languages (or at least, not as easily).

### Why type hinting?

Why is it then, that after so many years, Python was extended to support type annotations?

One benefit is that, by having some knowledge about what type a certain variable might be, modern IDEs like PyCharm, Visual Studio Code, and the like, can provide smarter code completion and error checks.

Another benefit is readability. Knowing the type of a certain variable might make it easier to understand and follow the operations the code is performing on that object.

And, of course, there are lots of tools that, thanks to type hinting, can extract information from the code and automatically create documentation for it. That's one of the ways in which FastAPI leverages type hinting, as we will see later.

In summary, this feature has proved to be quite useful, so it's not surprising that type hinting is now being adopted more and more in the Python community.

### Type hinting in a nutshell

We will now explore examples of type hinting, although we will look at only what is strictly necessary in order to understand the FastAPI code that will come later. Let's start with an example of function annotations. Take a look at the following snippet:
```python

def greet(first_name, last_name, age):
    return f"Greeting {first_name} {last_name} of age {age}"

```

The greet() function takes three arguments and returns a greeting string. We might want to add some annotations to it, so for example we could, in an exaggerated fashion, do this:
```python

def greet(
    first_name: "First name of the person we are greeting",
    last_name: "Last name of the person we are greeting",
    age: "The person's age"
) -> "Returns the greeting sentence":
    return f"Greeting {first_name} {last_name} of age {age}"


```

This potentially weird-looking code is actually valid Python code, functionally equivalent to the previous version without annotations. We can add any expression after the colon, and even after the parameters' declaration, we can annotate on the return value using the arrow notation.

The ability to do the above enabled the introduction of type hinting. Let's now explore an example of that:
```python

def greet(first_name: str, last_name: str, age: int = 18) -> str:
    return f"Greeting {first_name} {last_name} of age {age}"

```

This code is quite significant. It tells us that first_name and last_name parameters are supposed to be strings, while age is supposed to be an int with a default value of 18. It also tells us that the return value is a string. We say "… parameters are supposed to be…" because nothing prevents us from calling this function with whatever type we want. Let's demonstrate it:
```python

greet(123, 456, "hello")
# returns: 'Greeting 123 456 of age hello'

```

Python has no way of enforcing parameters to be of a type that corresponds to their declaration. However, while writing that call, our IDE suggested to us that first_ name and last_name were supposed to be strings, and age an int with default value 18:

Figure 14.2: Visual Studio Code (Jupyter Notebook extension) leveraging type hinting

We can use all the standard Python types: int, str, float, complex, bool, bytes, and so on. There are some Python data structures that can contain other values, such as list, dict, set, and tuple. Their internal values can have their own type too. To declare those types, and the types of their internal values, we need to use the typing module from the standard library.

This module, which you can find in the documentation at https://docs.python.org/3/library/typing.html, is a great place to start when you want to learn more about this topic. Not only does the page document the typing module, it also lists all the PEPs related to type hinting at the beginning, so you can gradually explore them and learn more and more about type hinting, and how and why it came to be.

Let's see another example:
```python

from typing import List
def process_words(words: List[str]):
    for word in words:
        # do something with word

```

In this short snippet, we can immediately understand that words is supposed to be a list of strings.
```python

from typing import Dict
def process_users(users: Dict[str, int]):
    for name, age in users.items():
        # do something with name and age

```

In this example, we can see another container class, Dict. By declaring users this way, we can expect it to be a dictionary where keys represent users' names, and values their ages.

In Python 3.9, Dict and List have been deprecated in favor of just using dict or list, which now support the […] syntax too. 

An important class from the typing module is Optional. Optional is used to declare that an argument has a certain type, but that it could also be None (that is, it's optional):
```python

from typing import Optional
def greet_again(name: Optional[str] = None):
    if name is not None:
        print(f"Hello {name}!")
    else:
        print("Hey dude")

```

Optional plays an important role in FastAPI, as we'll see later, in that it allows us to define parameters that might or might not be used when interrogating an API endpoint.

One last example we want to show you concerns custom types:

class Cat:
    def __init__(self, name: str):
        self.name = name

def call_cat(cat: Cat):
    return f"{cat.name}! Come here!"

The call_cat() function expects a cat argument, which should be an instance of Cat. This feature is very important in FastAPI because it allows programmers to declare schemas that represent query parameters, request bodies, and so on.

You should now be equipped to understand the main part of this chapter. We would encourage you to read up a bit more about type hinting in Python though, as it is becoming more and more common to see it in source code.

## APIs: An introduction

Before we delve into the details of this chapter's specific project, let's spend a moment talking about APIs in general.

### What is an API?
As we mentioned at the beginning of the chapter, API stands for Application Programming Interface. 

This interface is designed to act as a connection layer between computers, or computer programs. It is therefore a type of software interface that provides a service to other pieces of software, in contrast to user interfaces, which instead connect computers to people. 

An API is normally accompanied by a specification document, or standard, which describes how to build the API and how it works. A system that meets the specification is said to implement, or expose, the API. The term API can describe either the implementation or the specification.

An API is normally made of different parts, which are the tools that the programmers who write software use to interface with it. These parts are known by different names, the most common of which are methods, subroutines, or endpoints (we will call them endpoints in this chapter). When we use these parts, the technical term for this is calling them. 

The API specification instructs you on how to call each endpoint, what type of requests to make, which parameters and headers to pass, which addresses to reach, and so on.

### What is the purpose of an API?

There are several reasons to introduce an API into a system. One we have already mentioned is to create the means for different pieces of software to communicate.

Another important reason is to allow access to a system by hiding its internal details and implementation, and exposing to the programmers only the parts that it is safe, and necessary, to expose.

The fact that APIs hide the internals of the systems they are interfaced with provides another benefit: if the internal system changes, in terms of technology, languages, or even workflows, the API can change in the way it connects to it, but still provide a consistent interface to the other side, the one that is exposed to the public. If we put a letter into a letterbox, we don't need to know or control how the postal service will deal with it, as long as the letter arrives at the destination. So, the interface (the letterbox) is kept consistent, while the other side (the mailman, their vehicles, technology, workflows, and so on) is free to change and evolve. Finally, APIs are able to provide necessary features, such as authentication and authorization, and data validation. Being the layer that is exposed to the world, it makes sense that they are in charge of these tasks.

Authentication means the system has the ability to validate user credentials in order to unequivocally identify them. Authorization means the system has the ability to verify what a user has access to. 

Users, systems, and data are checked and validated at the border, and if they pass the check, they can interact with the rest of the system (through the API).

This mechanism is conceptually similar to landing at an airport and having to show the police our passports. After that check is successful, we are free to interact with the system, which is the country we landed in, without having to show our passport again.

Given the above, it should not be surprising to know that basically any electronic device we own today, that is connected to the Web, is talking to a (potentially wide) range of APIs in order to perform its tasks.

### API protocols

There are different types of API. They can be open to the public, or private. They can provide access to data, or services, or both. APIs can be written and designed using very different methods and standards, and they can employ different protocols.

These are the most common protocols:

•  REST (Representational State Transfer) is a Web services API. REST APIs are a key part of modern Web applications such as Netflix, Uber, and Amazon, among several others. In order for an API to be considered RESTful, it has to adhere to a set of rules. These include concepts like being stateless, providing a uniform interface, and client-server independence.

•  SOAP (Simple Object Access Protocol) is a well-established protocol similar to REST in that it's a type of Web API. SOAP was the first to standardize the way applications should use network connections to manage services. Its specification is very strict compared to that of REST. In general, it also requires more bandwidth.

•  RPC (Remote Procedural Call) is the oldest and simplest type of API. The implementation allows programmers to execute code on the server side by remotely calling a procedure (hence, the name). These types of API are tightly coupled with the implementation of the server they allow access to, so normally they are not made for the public, and maintaining them usually proves to be quite a complex task.

There is plenty of information about API protocols online if you are interested in learning more.

### API data-exchange formats

We said that an API is an interface between at least two computer systems. It would be quite unpleasant, when interfacing with another system, to have to shape the data into whatever format that system implements. Therefore, the API, which provides a communication layer between the systems, not only specifies the protocols over which the communication happens, but also which language (or languages) has to be adopted for the data that is exchanged.

The most common data-exchange formats today are JSON, XML, and YAML. We have already used JSON in Chapter 8, Files and Data Persistence, and we will use it as the format for the API of this chapter too. JSON is widely adopted today by many APIs, and many frameworks provide the ability to translate data from and to JSON, out of the box.

## The railway API

Now that we have a working knowledge of what an API is, let's turn to something more concrete.

Before we show you the code, allow us to stress that this code is not production- ready, as that would have been too long and needlessly complex for a book's chapter. However, this code does its job and does it well, and it will allow you to learn quite a lot if you decide to study it, evolve it, and improve it. We will leave suggestions on how to do so at the end of this chapter.

We have a database with some entities that model a railway. We want to allow an external system to perform CRUD operations on the database, so we are going to write an API to serve as the interface to it.

CRUD stands for Create, Read, Update, and Delete. These are the four basic database operations. Many HTTP services also model CRUD operations through REST or REST-like APIs. 

Let's start by taking a look at the project files, so you will have an idea of where things are. You can find them in the folder for this chapter, in the source code:

```bash
$ tree api_code
api_code
├── api
│   ├── __init__.py
│   ├── admin.py
│   ├── config.py
│   ├── crud.py
│   ├── database.py
│   ├── deps.py
│   ├── models.py
│   ├── schemas.py
│   ├── stations.py
│   ├── tickets.py
│   ├── trains.py
│   ├── users.py
│   └── util.py
├── dummy_data.py
├── main.py
├── queries.md
└── train.db
```

Within the api_code folder, you can find all the files belonging to the FastAPI project. The main application module is main.py. We have left the dummy_data.py script in the code, which you can use to generate a new train.db, the database file. Make sure you read the README.md in this chapter's folder for instructions on how to do it. We have also collected a list of queries to the API for you to copy and try out, in queries.md.

Within the api package, we have the application modules. The database models are in models.py, and the schemas used to describe them to the API are in schemas. py. The other modules' purposes should be evident from their names: users.py, stations.py, tickets.py, trains.py, and admin.py all contain the definitions of the corresponding endpoints of the API. util.py contains some utility functions; deps.py defines the dependency providers; config.py holds the configuration settings' boilerplate; and, finally, crud.py contains the functions that perform CRUD operations on the database.

In software engineering, dependency injection is a design pattern in which an object receives other objects that it depends on, called dependencies. The software responsible for constructing and injecting those dependencies is known as the injector, or provider. Hence, a dependency provider is a piece of software that creates and provides a dependency, so that other parts of the software can use it without having to take care of creating it, setting it up, and disposing of it. To learn more about this pattern, please refer to this Wikipedia page: https://en.wikipedia.org/wiki/Dependency_injection. 

### Modeling the database

When preparing the entity-relationship schema for this project, we sought to design something interesting and at the same time simple and well contained. This application considers four entities: Stations, Trains, Tickets, and Users. A Train is a journey from one station to another one. A Ticket is a connection between a Train and a User. Users can be passengers or administrators, according to what they are supposed to be able to do with the API. In Figure 14.3, you can see the entity-relationship (ER) model of the database. It describes the four entities and how they relate to one another:

Figure 14.3: ER model of the project database

ERAlchemy (https://github.com/Alexis-benoist/eralchemy) is a very useful tool for generating entity-relationship diagrams from databases or SQLAlchemy models. We used it to generate the diagram in Figure 14.3. We have defined the database models using SQLAlchemy, and we have chosen SQLite as the DBMS, for simplicity. If you skipped Chapter 8, Files and Data Persistence, this would probably be a good moment to read it, as there you will learn all you need to know in order to understand the models in this chapter's project.

Let's see the models module:

```python

# api_code/api/models.py
import enum
import hashlib
import os
import secrets

from sqlalchemy import (
    Column, DateTime, Enum, Float, ForeignKey, Integer, Unicode
)
from sqlalchemy.orm import relationship

from .database import Base

UNICODE_LEN = 128
SALT_LEN = 64

# Enums
class Classes(str, enum.Enum):
    first = "first"
    second = "second"

class Roles(str, enum.Enum):
    admin = "admin"
    passenger = "passenger"

```

As usual, at the top of the module, we import all the necessary modules. We then define a couple of variables to indicate the default length of Unicode fields (UNICODE_ LEN) and the length of the salt used to hash passwords (SALT_LEN). For a refresher on what a salt is, please refer to Chapter 9, Cryptography and Tokens. We also define two enumerations: Classes and Roles, which will be used in the models' definitions. Let's see the definition of the Station model:

```python
# api_code/api/models.py
class Station(Base):
    __tablename__ = "station"

    id = Column(Integer, primary_key=True)
    code = Column(
        Unicode(UNICODE_LEN), nullable=False, unique=True
    )
    country = Column(Unicode(UNICODE_LEN), nullable=False)
    city = Column(Unicode(UNICODE_LEN), nullable=False)

    departures = relationship(
        "Train",
        foreign_keys="[Train.station_from_id]",
        back_populates="station_from",
    )
    arrivals = relationship(
        "Train",
        foreign_keys="[Train.station_to_id]",
        back_populates="station_to",
    )

    def __repr__(self):
        return f"<{self.code}: id={self.id} city={self.city}>"
    __str__ = __repr__

```

The Station model is pretty straightforward. There are a few attributes: id acts as primary key, and then we have code, country, and city, which combined tell us all we need to know about a station. There are two relationships that link station instances to all the trains departing from, and arriving to, them. The rest of the code defines the __repr__() method, which provides a string representation for an instance, and whose implementation is also assigned to __str__, so the output will be the same whether we call str(station_instance) or repr(station_instance). This technique is quite commonly adopted to avoid code repetition.

Notice we defined a unique constraint on the code field, so we make sure no two stations with the same code can exist in the database. Big cities like Rome, London, and Paris have more than one train station, so the fields city and country will be the same, but each station will have its own unique code.

Following that, we find the definition of the Train model:
```python

# api_code/api/models.py
class Train(Base):
    __tablename__ = "train"

    id = Column(Integer, primary_key=True)
    name = Column(Unicode(UNICODE_LEN), nullable=False)

    station_from_id = Column(
        ForeignKey("station.id"), nullable=False
    )
    station_from = relationship(
        "Station",
        foreign_keys=[station_from_id],
        back_populates="departures",
    )

    station_to_id = Column(
        ForeignKey("station.id"), nullable=False
    )
    station_to = relationship(
        "Station",
        foreign_keys=[station_to_id],
        back_populates="arrivals",
    )

    departs_at = Column(DateTime(timezone=True), nullable=False)
    arrives_at = Column(DateTime(timezone=True), nullable=False)
    first_class = Column(Integer, default=0, nullable=False)
    second_class = Column(Integer, default=0, nullable=False)
    seats_per_car = Column(Integer, default=0, nullable=False)
    tickets = relationship("Ticket", back_populates="train")

    def __repr__(self):
        return f"<{self.name}: id={self.id}>"
    __str__ = __repr__ 

```

In the Train model, we find all the attributes we need to describe a train instance, plus a handy relationship, tickets, that gives us access to all the tickets that have been created against a train instance. The first_class and second_class fields hold how many first- and second-class cars a train has. 

We also added relationships to station instances: station_from and station_to. These allow us to fetch the station instances as objects, instead of only having access to their IDs.

Next up, the Ticket model:
```python

# api_code/api/models.py
class Ticket(Base):
    __tablename__ = "ticket"

    id = Column(Integer, primary_key=True)
    created_at = Column(DateTime(timezone=True), nullable=False)
    user_id = Column(ForeignKey("user.id"), nullable=False)
    user = relationship(
        "User", foreign_keys=[user_id], back_populates="tickets"
    )

    train_id = Column(ForeignKey("train.id"), nullable=False)
    train = relationship(
        "Train", foreign_keys=[train_id], back_populates="tickets"
    )

    price = Column(Float, default=0, nullable=False)
    car_class = Column(Enum(Classes), nullable=False)

    def __repr__(self):
        return (
            f"<id={self.id} user={self.user} train={self.train}>"
        )
    __str__ = __repr__

```

A Ticket instance has some properties too and includes two relationships, user and train, that point respectively to the user who bought the ticket, and to the train the ticket is for.

Notice how we have used the Classes enumeration in the definition of the car_class attribute. This translates to an enumeration field in the database schema definition.

Finally, the User model:
```python

# api_code/api/models.py
class User(Base):
    __tablename__ = "user"
    pwd_separator = "#"

    id = Column(Integer, primary_key=True)
    full_name = Column(Unicode(UNICODE_LEN), nullable=False)
    email = Column(Unicode(256), nullable=False, unique=True)
    password = Column(Unicode(256), nullable=False)
    role = Column(Enum(Roles), nullable=False)

    tickets = relationship("Ticket", back_populates="user")

    def is_valid_password(self, password: str):
        """Tell if password matches the one stored in DB."""
        salt, stored_hash = self.password.split(
            self.pwd_separator
        )
        _, computed_hash = _hash(
            password=password, salt=bytes.fromhex(salt)
        )
        return secrets.compare_digest(stored_hash, computed_hash)

    @classmethod
    def hash_password(cls, password: str, salt: bytes = None):
        salt, hashed = _hash(password=password, salt=salt)
        return f"{salt}{cls.pwd_separator}{hashed}"

    def __repr__(self):
        return (f"<{self.full_name}: id={self.id} "
                f"role={self.role.name}>")
    __str__ = __repr__

```

The User model defines some properties for each user. Note how here we have another enumeration used for the user's role. A user can either be a passenger or an admin. This will allow us to present you with a simple example of how to write an endpoint that allows access only to authorized users.

There are a couple of methods on the User model that are used to hash and validate passwords. You might recall from Chapter 9, Cryptography and Tokens, that passwords should never be stored in a database as they are. So, in our API, when saving a password for a user, we create a hash and store it alongside the salt that was used. In the source code for the book, you will find, at the end of this module, the implementation of the _hash() function, which we have omitted here, for brevity.

### Main setup and configuration

Let us start from the main access point of the application:
```python

 # api_code/main.py
from api import admin, config, stations, tickets, trains, users
from fastapi import FastAPI

settings = config.Settings()
app = FastAPI()

app.include_router(admin.router)
app.include_router(stations.router)
app.include_router(trains.router)
app.include_router(users.router)
app.include_router(tickets.router)

@app.get("/")
def root():
    return {
        "message": f"Welcome to version {settings.api_version} of our API"
    }

```

This is all the code in the main.py module. It imports the various specific endpoint modules, and includes their routers in the main app. By including a router in the main app, we enable the application to serve all the endpoints declared using that specific router. This will be clearer in a moment.

There is only one endpoint in this module, which serves a greeting message. An endpoint is a simple function, in this case, root, which contains the code to be executed when the endpoint is called. When and how this function will be invoked depends on the decoration setup. In this case, we have only two pieces of information: with .get(), we instruct the API to serve this endpoint when called with a GET request; and with "/", we tell the app that this endpoint will be found at the root, which is the base URL on which the app is running. We will see this in more detail later, when we consume the API. But just to explain here briefly: if this API was served at the base URL http://localhost:8000, this endpoint would be called when we requested either http://localhost:8000 or http://localhost:8000/ (notice the difference is in the trailing slash) from a browser, for example. 

### Adding settings

Within the greeting message from the last snippet of code, there is a variable, api_ version, taken from the settings object. All frameworks allow for a collection of settings to be used, in order to configure the app before it runs. We didn't really need to use settings in this example project—we could have just hardcoded those values in the main module—but we thought it was worth showing you how they work:
```python

# api_code/api/config.py
from pydantic import BaseSettings

class Settings(BaseSettings):
    secret_key: str
    debug: bool
    api_version: str

    class Config:
        env_file = ".env"

```

Settings are defined within a Pydantic model (https://github.com/samuelcolvin/pydantic/). Pydantic is a library that provides data validation and settings management using Python type annotations. In this case, we have three pieces of information within the settings:

•  secret_key: Used to sign and verify JSON Web Tokens (JWTs).
•  debug: When set to True, it instructs the SQLAlchemy engine to log verbosely, which is helpful when debugging queries, for example.
•  api_version: The version of the API. We don't really make use of this information, apart from displaying it in the greeting message, but normally the version plays a very important role because the API specification changes according to which version is running.

FastAPI plucks these settings from a .env file, as specified by the nested Config class within the Settings model. Here is how that file looks:
```bash

# api_code/.env
SECRET_KEY="ec604d5610ac4668a44418711be8251f"
DEBUG=false
API_VERSION=1.0.0


```

In order for this to work, FastAPI needs help from a library called python-dotenv. It is part of this chapter's requirements, so if you have installed them in your virtual environment, you're all set.

### Station endpoints

Now we are going to write some FastAPI endpoints. Because this API is CRUD-oriented, there is some repetition in the code. We will therefore show you one example for each of the CRUD operations, and we will do so by using the Station endpoints examples. Please refer to the source code to explore the endpoints related to the other models. You will find that they all follow the same patterns and conventions, and the main difference is that they relate to different database models.

We are going to introduce concepts and technical details gradually, alongside the code examples, so the presentation should flow organically.

### Reading data

Let us start our exploration with the simplest of all request types: GET. In this case, we are going to get all the stations in the database.

```python

# api_code/api/stations.py
from typing import Optional
from fastapi import (
    APIRouter, Depends, HTTPException, Response, status
)
from sqlalchemy.orm import Session
from . import crud
from .deps import get_db
from .schemas import Station, StationCreate, StationUpdate, Train

router = APIRouter(prefix="/stations")

@router.get("", response_model=list[Station], tags=["Stations"])
def get_stations(
    db: Session = Depends(get_db), code: Optional[str] = None
):
    return crud.get_stations(db=db, code=code)

```

Within the stations.py module, we start by importing the necessary objects from the typing module, and from fastapi. We also import Session from sqlalchemy, and a few other tools from the local codebase.

The function get_stations() is decorated with a router object, instead of app like it was in the main file. APIRouter can be thought of as a mini FastAPI class, in that it takes all the same options. We declare router and assign a prefix to it ("/stations", in this case), which means all functions decorated with this router become endpoints that can be called at the address http://localhost:8000/stations. In this case, the empty string fed to the .get() method of the decorator instructs the app to serve this endpoint on the root URL for this router, which will be the concatenation of the base URL and the router prefix, as explained above.

Then we pass response_model, which is a list of Station instances, the implementation of which we will see later. Finally, tags, which is used to organize the documentation (we will see what they do later on).

The function itself takes some arguments, which are a database session, db, and an optional string, code, which when specified will instruct the endpoint to serve only the stations whose code field matches the one provided.

A few things to notice:

•  Data coming with the request, such as query parameters, are specified in the endpoint declaration. If the endpoint function requires data to be sent in the body of the request, this is specified using Pydantic models (in this project, they are defined in the schemas.py module).

•  Whatever an endpoint returns becomes the body of the response. If the response_model parameter is not defined, FastAPI will try to serialize the return data to JSON. However, when the response model is set, serialization goes first through the Pydantic model specified in response_model, and then from the Pydantic model to JSON. In order to use a database session in the body of an endpoint, we are using a dependency provider, which in this case is specified using the Depends class, to which we pass the function get_db(). This function yields a local database session and closes it when the endpoint call terminates.

•  We use the Optional class, from the typing module, to specify all the optional parameters that may or may not be in the request. The body of the get_stations() function simply returns what its homonym from the crud module returns. All the functions that regulate the interaction with the database live in the crud module. 

This was a design choice that should make this code easier to reuse and test. Moreover, it simplifies reading the entry point code a lot. So, let's see the body of 
get_stations():
```python

# api_code/api/crud.py
from datetime import datetime, timezone
from sqlalchemy import delete, update
from sqlalchemy.orm import Session, aliased
from . import models, schemas

def get_stations(db: Session, code: str = None):
    q = db.query(models.Station)
    if code is not None:
        q = q.filter(models.Station.code.ilike(code))
    return q.all()

```

Notice how similar this function's signature is to the one for the endpoint that calls it. get_stations() returns all instances of Station, optionally filtered using code (in case it's not None).

To start the API, you need to activate your virtual environment and run the following command from the api_code folder:
```bash

$ uvicorn main:app --reload

```

Uvicorn is a lightning-fast ASGI server, built on uvloop and httptools. It is the recommended server for FastAPI. It works seamlessly with both normal and asynchronous functions. 

From the ASGI documentation page (https://asgi.readthedocs.io/en/latest/):

    ASGI (Asynchronous Server Gateway Interface) is a spiritual successor to WSGI (Web Server Gateway Interface), intended to provide a standard interface between async-capable Python web servers, frameworks, and applications.

Where WSGI provided a standard for synchronous Python apps, ASGI provides one for both asynchronous and synchronous apps, with a WSGI backwards-compatibility implementation and multiple servers and application frameworks.

For this chapter's project, we have chosen to adopt a simple approach, therefore we haven't written any asynchronous code. 

Please check out how to write asynchronous endpoints, and the scenarios in which that is the recommended approach, on the official FastAPI documentation page: https://fastapi.tiangolo.com.

In the command above, you don't need the --reload flag unless you are working on the API source code and want the server to automatically reload whenever a file is saved. It's quite a handy tool to have when developing the API.

If we called this endpoint, this is what we would see:
```python

$ http http://localhost:8000/stations
HTTP/1.1 200 OK
content-length: 702
content-type: application/json
date: Thu, 19 Aug 2021 22:11:10 GMT
server: uvicorn

[
    {
        "city": "Rome",
        "code": "ROM",
        "country": "Italy",
        "id": 0
    },
    {
        "city": "Paris",
        "code": "PAR",
        "country": "France",
        "id": 1
    },
    ... some entries omitted ...
    {
        "city": "Sofia",
        "code": "SFA",
        "country": "Bulgaria",
        "id": 11
    }
]

```

Notice the command we are using to call the API: http. This is a command that comes with the Httpie utility. 

You can find Httpie at https://httpie.io. Httpie is a user-friendly command-line HTTP client for the API era. It comes with JSON support, syntax highlighting, persistent sessions, wget-like downloads, plugins, and more. There are other tools to perform requests, such as curl, but the choice is up to you, as it makes no difference which tool you use to make your requests from the command line. 

The API is served by default at http://localhost:8000. You can add arguments to the uvicorn command to customize this, but there is no need for that in this case.

The first few lines of the response are information from the API engine. We learn the protocol used was HTTP1.1, and that the request succeeded (status code 200 OK). We have info on the content length, and its type, which is JSON. Finally, a timestamp and the type of server. We are going to omit the part of this information that will repeat, from now on.

The body of the response is a list of Station instances, in their JSON representation, thanks to response_model=list[Station], which we passed to the endpoint decoration.

If we were to search by code, for example using the London station one, we could use the following command:
```bash

$ http http://localhost:8000/stations?code=LDN

```

The above command uses the same URL but adds the code query parameter (after the ?). The result is as follows:
```bash

$ http http://localhost:8000/stations?code=LDN
HTTP/1.1 200 OK
...
[
    {
        "city": "London",
        "code": "LDN",
        "country": "UK",
        "id": 2
    }
]

```

Notice how we got one match, which corresponds to the London station, but still, it is given back within a list, as expected by the type of response_model for this endpoint.

Let us now explore an endpoint dedicated to fetching a single station by ID:

```python

# api_code/api/stations.py
@router.get(
    "/{station_id}", response_model=Station, tags=["Stations"]
)
def get_station(station_id: int, db: Session = Depends(get_db)):
    db_station = crud.get_station(db=db, station_id=station_id)
    if db_station is None:
        raise HTTPException(
            status_code=404,
            detail=f"Station {station_id} not found.",
        )
    return db_station

```

For this endpoint, we configure the router to listen to a GET request, at the URL http://localhost:8000/stations/{station_id}, where station_id will be an integer. Hopefully, the way URLs are constructed is starting to make sense for you. There is the base part, http://localhost:8000, then the prefix for the router,  /stations, and finally, the specific URL information which we feed to each endpoint, which in this case is /{station_id}.

Let's fetch the Kyiv station, with ID 3:
```python

$ http http://localhost:8000/stations/3
HTTP/1.1 200 OK
...

{
    "city": "Kyiv",
    "code": "KYV",
    "country": "Ukraine",
    "id": 3
}
```

Notice how this time we got back one object by itself, instead of it being wrapped in a list like it was in the get_stations() endpoint. This is in accordance with the response model for this endpoint, which is set to Station, and it makes sense, as we are fetching a single object by ID.

The get_station() function takes the station_id, type-hinted as an integer, and the usual db session object. Using type hinting to specify parameters allows FastAPI to do some data validation on the type of the arguments we use when calling an endpoint. 

If we were to pass a non-integer value for station_id, this would happen:
```bash

$ http http://localhost:8000/stations/kyiv
HTTP/1.1 422 Unprocessable Entity
...
{
    "detail": [
        {
            "loc": [
                "path",
                "station_id"
            ],
            "msg": "value is not a valid integer",
            "type": "type_error.integer"
        }
    ]
}
```

FastAPI responds to us with useful information: station_id, from the path, is not a valid integer. Notice also that the status code is 422 Unprocessable Entity, as opposed to a 200 OK, this time. In general, errors in the four hundreds (4xx) express client errors, as opposed to errors in the five hundreds (5xx), which express server errors. In this case, we are making a call using an incorrect URL (we're not using an integer), therefore it is an error on our side. Many API frameworks would return a simple status code 400 Bad Request in the same scenario, but FastAPI returns 422 Unprocessable Entity, which is oddly specific. It is easy though, in FastAPI, to customize which status would be returned upon a bad request; there are a few examples in the official documentation.

Let's see what happens when we try to fetch a station with an ID that doesn't exist:

```bash

$ http http://localhost:8000/stations/100
HTTP/1.1 404 Not Found
...
{
    "detail": "Station 100 not found."
}


```
This time, the URL is correct, in that station_id is an integer; however, there is no station with ID 100. The API returns status 404 Not Found, as the response body tells us.

If you go back to the body of this endpoint, you will notice how straightforward its logic is: provided that the arguments passed are correct—in other words, they respect the type—it tries to fetch the corresponding station from the database, by using another simple function from the crud module. If the station is not found, it raises an HTTPException with the desired status code (404) and a detail that will hopefully help the consumer understand what went wrong. If the station is found, then it is simply returned. The process of returning a JSON serialized version of objects is automatically done for us by FastAPI. The object retrieved from the database is a SQLAlchemy instance of the Station class (models.Station). That instance is fed to the Pydantic Station class (schemas.Station), which is used to produce a JSON representation that is then returned by the endpoint.

This might seem complicated, but it is actually a great example of decoupling. The workflow is already there, taken care of for us, and all we need to do is write the little puzzle pieces we need: request parameters, response models, dependencies, and so on.

### Creating data

Let's now see something a bit more interesting: how to create a station. First, the endpoint:

```python
# api_code/api/stations.py
@router.post(
    "",
    response_model=Station,
    status_code=status.HTTP_201_CREATED,
    tags=["Stations"],
)
def create_station(
    station: StationCreate, db: Session = Depends(get_db)
):
    db_station = crud.get_station_by_code(
        db=db, code=station.code
    )
    if db_station:
        raise HTTPException(
            status_code=400,
            detail=f"Station {station.code} already exists.",
        )
    return crud.create_station(db=db, station=station)
```

This time, we instruct the router that we want to accept a POST request to the root URL (remember: base part, plus router prefix). We specify the response model to be Station, as the endpoint will be returning the newly created object, and we also specify the default status code for the response, which is 201 Created.

The create_station() function takes the usual db session and a station object. The station object is created for us, behind the scenes. FastAPI takes the data from the body of the request and feeds it to the Pydantic schema StationCreate. That schema defines all the pieces of data we need to receive, and the result is the station object.

The logic in the body follows this flow: it tries to get a station using the code provided; if a station is found, we cannot create one with that data. The code field is defined to be unique, therefore creating a station with the same code would result in a database error. Hence, we return status code 400 Bad Request, informing the caller that the station already exists. In the case the station is not found, we can instead proceed to create it and return it. Let's see the declaration of the Pydantic schemas first:

```python
# api_code/api/schemas.py
from pydantic import BaseModel

class StationBase(BaseModel):
    code: str
    country: str
    city: str

class Station(StationBase):
    id: int
    class Config:
        orm_mode = True

class StationCreate(StationBase):
    pass
```

Schema structure leverages inheritance. It is normal practice to have a base schema that provides functionalities common to all children. Then, each child specifies its needs separately. In this case, in the base schema, we find code, country, and city. When fetching stations, we also want to return the id, so we specify that in the Station class. Moreover, since this class is used to translate SQLAlchemy objects, we need to tell the model about it, and we do so within the nested Config class. Remember that SQLAlchemy is an object-relational mapping (ORM) technology, so we need to tell the model to turn on the ORM mode by setting orm_mode = True.

The StationCreate model doesn't need anything extra, so we simply use the pass instruction as a body. Let's now see the CRUD functions for this endpoint:
```python
# api_code/api/crud.py
def get_station_by_code(db: Session, code: str):
    return (
        db.query(models.Station)
        .filter(models.Station.code.ilike(code))
        .first()
    )

def create_station(
    db: Session,
    station: schemas.StationCreate,
):
    db_station = models.Station(**station.dict())
    db.add(db_station)
    db.commit()
    db.refresh(db_station)
    return db_station
```

The get_station_by_code() function is fairly simple. It filters through Station objects with a case-insensitive match on code (that is why we use ilike; the "i" in the prefix means [case-]insensitive).

There are other ways to perform a case-insensitive comparison, which do not involve using ilike. This might be the right way to go when performance is important, but for this chapter's purpose, we found the simplicity of ilike to be exactly what we needed.

More interesting is the create_station() function. It takes a db session and a Pydantic StationCreate instance. First, we get the station data in the form of a Python dictionary. We know all data must be there, otherwise the endpoint would have already failed during the initial Pydantic validation stage.

Using the data from station.dict(), we create an instance of the SQLAlchemy Station model. We add it to the database, commit the transaction, then we refresh the object and return it. We need to refresh the object because we want to return it with its id too, but unless we refresh it—which means re-fetching it from the database—the id won't be there, as it only gets assigned to the object when it is saved to the database.

Let's see this endpoint in action. Notice how we need to specify POST to the http command, which allows us to send data, in JSON format, within the body of the request. Previous requests were of the GET type, which is the default type for the http command. Notice also that we have split the command over two lines due to the book's line length constraints:
```bash

$ http POST http://localhost:8000/stations \
code=TMP country=Temporary-Country city=tmp-city
HTTP/1.1 201 Created
...
{
    "city": "tmp-city",
    "code": "TMP",
    "country": "Temporary-Country",
    "id": 12
}

```

Great! We created a station with that data. Let's now try again, but this time omitting something mandatory, like the code:
```bash
$ http POST http://localhost:8000/stations \
country=Another-Country city=another-city
HTTP/1.1 422 Unprocessable Entity
...
{
    "detail": [
        {
            "loc": [
                "body",
                "code"
            ],
            "msg": "field required",
            "type": "value_error.missing"
        }
    ]
}

```

Brilliant. As expected, we get a status code 422 Unprocessable Entity again, because the Pydantic StationCreate model validation failed, and the response body tells us why: code is missing in the body of the request.

### Updating data

To update a station, the logic gets just a bit more complex, but not too much. Let's go through it together. First, the endpoint:

```python
# api_code/api/stations.py
@router.put("/{station_id}", tags=["Stations"])
def update_station(
    station_id: int,
    station: StationUpdate,
    db: Session = Depends(get_db),
):
    db_station = crud.get_station(db=db, station_id=station_id)
    if db_station is None:
        raise HTTPException(
            status_code=404,
            detail=f"Station {station_id} not found.",
        )
    else:
        crud.update_station(
            db=db, station=station, station_id=station_id
        )
        return Response(status_code=status.HTTP_204_NO_CONTENT)
```

The router now is instructed to listen for a PUT request, which is the type you should use to modify a web resource. The URL terminates with the station_id, which identifies the station we want to update. The function takes the station_id, a Pydantic StationUpdate instance, and the usual db session.

We start by fetching the desired station from the database. If the station is not found in the database, we simply return status code 404 Not Found, as there is nothing to update. Otherwise, we update the station and return status code 204 No Content, which is the common way to handle the response for a PUT request. We also could have returned 200 OK, but in that case, we should have returned the updated resource within the body of the response.

Let's see the code for the CRUD function responsible for updating a station:
```python
# api_code/api/crud.py
from sqlalchemy import delete, update

def update_station(db: Session, station: schemas.StationUpdate, station_id: int):
    stm = (
        update(models.Station)
        .where(models.Station.id == station_id)
        .values(station.dict(exclude_unset=True))
    )
    result = db.execute(stm)
    db.commit()
    return result.rowcount
```

The update_station() function takes the necessary arguments to identify the station to update, and the station data that will be used to update the record in the database, plus the usual db session.

We build a statement using the update() helper, from sqlalchemy. We specify a where clause to filter the station by id, and we specify the new values by asking the Pydantic station object to give us a dict excluding anything that hasn't been passed to the call. This serves the purpose of allowing partial updates to be executed. If we omitted exclude_unset=True from the code, any argument that wasn't passed would end up in the dictionary, set to its default value (None).

Normally we would do partial updates by using a PATCH request, but these days it's pretty common to use PUT for both complete and partial updates. For simplicity and brevity, we have done so too, here.

We execute the statement and return the number of rows affected by this operation. We don't use this information in the endpoint body, but it will be a nice exercise for you to try out. We will see how to make use of that information in the endpoint that deletes a station.

The Pydantic model for a station update is this:

````python
# api_code/api/schemas.py
from typing import Optional

class StationUpdate(StationBase):
    code: Optional[str] = None
    country: Optional[str] = None
    city: Optional[str] = None

```

All properties are declared as optional because we want to allow the caller to pass only what they wish to update. Let's use this endpoint on the brand-new station we created in the previous section, 
with ID 12.
```bash

$ http PUT http://localhost:8000/stations/12 \
code=SMC country=Some-Country city=Some-city
HTTP/1.1 204 No Content
...

```

Brilliant, we got what we expected. Let's verify that the update was successful:

```bash
$ http http://localhost:8000/stations/12
HTTP/1.1 200 OK
...
{
    "city": "Some-city",
    "code": "SMC",
    "country": "Some-Country",
    "id": 12
}

```
It was indeed. All three properties of the object with ID 12 have been changed. Let's 
now try a partial update:
```bash

$ http PUT http://localhost:8000/stations/12 code=xxx
HTTP/1.1 204 No Content
...

```
This time we only updated the code. Let's verify again:

```bash
$ http http://localhost:8000/stations/12
HTTP/1.1 200 OK
...
{
    "city": "Some-city",
    "code": "xxx",
    "country": "Some-Country",
    "id": 12
}
```
Wonderful, only code was changed, as expected.

### Deleting data

Finally, let's explore how to delete a station. As usual, endpoint first:

```python
# api_code/api/stations.py
@router.delete("/{station_id}", tags=["Stations"])
def delete_station(station_id: int, db: Session = Depends(get_db)):
    row_count = crud.delete_station(db=db, station_id=station_id)
    if row_count:
        return Response(status_code=status.HTTP_204_NO_CONTENT)
    return Response(status_code=status.HTTP_404_NOT_FOUND)
```

For the deletion case, we instruct the router to listen for a DELETE request. The URL is the same one we used to get a single station, as well as to update one. The delete_ station() function takes station_id and the db session.

Inside the body of the endpoint, we get the number of affected rows from the operation. In this case, if there was at least one, we return status code 204 No Content, which signals to the caller that the deletion was successful. If there were no rows affected, we return status code 404 Not Found. Notice how we could have written the update method exactly like this, making use of the number of affected rows, but we chose a different style so that you had a different example to learn from.

Let's see the CRUD function:
```python

# api_code/api/crud.py
from sqlalchemy import delete, update

def delete_station(db: Session, station_id: int):
    stm = delete(models.Station).where(
        models.Station.id == station_id
    )
    result = db.execute(stm)
    db.commit()
    return result.rowcount

```

This function makes use of the delete() helper from sqlalchemy. Similar to what we did for the update scenario, we create a statement that identifies a station by ID and instructs for its deletion. We execute the statement and return the number of affected rows.

Let's see this endpoint in action, on a successful scenario first:
```bash

$ http DELETE http://localhost:8000/stations/12
HTTP/1.1 204 No Content
...
```

We got status code 204 No Content, which tells us the deletion was successful. Let's verify it indirectly, by trying to delete the station with ID 12 one more time. This time we expect the station to be gone, so we want to see a status code 404 Not Found, in return:
```bash

$ http DELETE http://localhost:8000/stations/12
HTTP/1.1 404 Not Found
...
```

And indeed, this time we received status code 404 Not Found, which means a station with ID 12 wasn't found, proving that the first attempt to delete it was successful. There are a few more endpoints in the stations.py module, which you should check out.

The other endpoints we have written are there to create, read, update, and delete users, trains, and tickets. Apart from the fact that they act on different database and Pydantic models, they wouldn't really bring much novelty to this exposition. Therefore, let's instead look at an example of how to authenticate a user.

### User authentication

Authentication, in this project, is done via a JSON Web Token. Once again, please refer to Chapter 9, Cryptography and Tokens, for a refresher on JWTs. Let's start from the authentication endpoint, in the users.py module.

```python
# api_code/api/users.py
from .util import InvalidToken, create_token, extract_payload

@router.post("/authenticate", tags=["Auth"])
def authenticate(
    auth: Auth,
    db: Session = Depends(get_db),
    settings: Settings = Depends(get_settings),
):
    db_user = crud.get_user_by_email(db=db, email=auth.email)
    if db_user is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"User {auth.email} not found.",
        )

    if not db_user.is_valid_password(auth.password):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Wrong username/password.",
        )

    payload = {
        "email": auth.email,
        "role": db_user.role.value,
    }
    return create_token(payload, settings.secret_key)
```

This router has the prefix "/users". To authenticate a user, we need to make a POST request to this endpoint. It takes a Pydantic Auth schema, the usual db session, and the settings object, which is needed to provide the secret key that was used to create the token.

If the user is not found, we simply return status code 404 Not Found. If the user is found, but the password provided doesn't correspond to the one in the database record, we can assume wrong credentials, and return status code 401 Unauthorized. Finally, if the user is found and the credentials are correct, we create a token with two simple claims: email and role. We will use the role to perform authorization functions.

The create_token() function is a simple convenience wrapper around jwt.encode() that also adds a couple of timestamps to the payload of the token. It is not worth showing that code here. Let's instead see the Auth model:
```python
# api_code/api/schemas.py
class Auth(BaseModel):
    email: str
    password: str
```

As expected, it is very simple. We authenticate users with their email (which serves as a username) and password. That is why, in the SQLAlchemy User model, we have set up a uniqueness constraint on the email field. We need each user to have a unique username, and email is a commonly used field for this need.

Let us exercise this endpoint:

```bash
$ http POST http://localhost:8000/users/authenticate \
email="fabrizio.romano@example.com" password="f4bPassword"
HTTP/1.1 200 OK
...
"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...g2cQhgyDpmyvCr75Qb_7snYI"
```

Wonderful, we got a token back (we have shortened it for brevity here)!

Now that we have a token, we can put it to use. The user we have authenticated is an admin, so we are going to show you how we could have written the endpoint to delete a station if we wanted to allow only admins to be able to do so. Let's see the code:

```python
# api_code/api/admin.py
...
from .util import is_admin

router = APIRouter(prefix="/admin")

def ensure_admin(settings: Settings, authorization: str):
    if not is_admin(
        settings=settings, authorization=authorization
    ):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail=f"You must be an admin to access this endpoint.",
        )

@router.delete("/stations/{station_id}", tags=["Admin"])
def admin_delete_station(
    station_id: int,
    authorization: Optional[str] = Header(None),
    settings: Settings = Depends(get_settings),
    db: Session = Depends(get_db),
):
    ensure_admin(settings, authorization)
    row_count = crud.delete_station(db=db, station_id=station_id)
    if row_count:
        return Response(status_code=status.HTTP_204_NO_CONTENT)
    return Response(status_code=status.HTTP_404_NOT_FOUND)
```

In this example, you can see that the endpoint declaration and body are practically the same as their naïve counterpart, with one major difference: before attempting to delete anything, we make sure to call ensure_admin(). In the endpoint, we need to grab the authorization header from the request, which is responsible for bearing the token information, so that we can pass it to the ensure_admin() function. We do this by declaring it in the function signature, as an optional string that comes from the Header object. The ensure_admin() function, defined above, delegates to the util. is_admin() function, which unpacks the token, verifies its validity, and inspects the role field within the payload, to see if it is that of an admin. If all the checks are successful, it returns True, or False otherwise. The ensure_admin() function does nothing when the check is successful, but raises an HTTPException with status code 401 Unauthorized when the check is unsuccessful. This means that if, for any reason, the user is not authorized to make this call, the execution of the endpoint's body will immediately stop and return after its first line.

Of course, there are much more sophisticated ways to do authentication and authorization, but it would have been impractical to fit them within the chapter. This simple example, though, is good enough a start to understand how an API would need to be written in order to be secure.

### Documenting the API

Documenting APIs is probably one of the most boring activities on the planet. So, we have great news! You won't have to document your FastAPI project, because the documentation is done for you by the framework. We have to thank Python's type hinting, and Pydantic, for this gift that we receive from FastAPI. Make sure your API is running, then open a browser and hit http://localhost:8000/docs. A page will open that should look something like this:

Figure 14.4: A partial screenshot of FastAPI self-generated documentation

In Figure 14.4 you can see a list of endpoints. They are categorized using the tags argument, which we have specified in each endpoint declaration. The beauty of this page is that not only can you see each endpoint and inspect its details, but you can also exercise them, using a very friendly interface. Make sure you try it out!

### Consuming an API

There are several ways to consume an API. We have seen a very common one: the console. This is a great way to experiment with an API, but it can be a bit cumbersome when the data we need to pass along with the request, including query parameters, headers, and so on, becomes more and more complex. For those situations, there are other options.

One option we often use is to talk to the API from within a Jupyter Notebook. We can use the Requests library (or anything equivalent) to call the endpoints and inspect data from within the Notebook. This solution is quite comfortable.

Another option is to use dedicated tools that provide you with a graphical user interface, like Postman, which comes packed with plenty of functionality. You can find Postman here: https://www.postman.com.

Browsers also now come with extensions that allow you to talk to APIs very easily.

Of course, we could also create a console or a GUI application that we can then use to exercise the API. We did something similar in Chapter 12, GUIs and Scripting. Finally, interaction with an API can be done as part of a consumer application, which has its own business logic and calls the API to fetch or manipulate data. Your smartphone is an obvious example of this, as basically almost every app on it talks to an API (more likely, to several APIs).

In the rest of this chapter, we would like to show you an example of the last of these options.

### Calling the API from Django

If you have already navigated the Python ecosystem, chances are you have stumbled into Django. Django is a web framework, written entirely in Python. It is one of the most widely adopted by the community, and for good reason. It is well written, promotes a sound approach to coding, and it is powerful.

We won't have the space here to fully explain how Django works, so we encourage you to check out its website at https://www.djangoproject.com, and go through the simple tutorial in the documentation. You will be amazed by the capabilities of this framework.

For the purpose of this chapter, we only need to know that Django adopts a pattern called MTV: Model, Template, View.

In short, Django provides an ORM engine, much like SQLAlchemy, that revolves around models. Models describe database tables and allow Django not only to store and handle data, but to provide a self-generated admin panel that interfaces with the database. This is just one of the many incredibly useful features of this framework. Models are the M in MTV. Views are the other layer, the V in MTV. They can be functions or classes, and they 
run when the user navigates to a specific URL. Each URL corresponds to a view. 

Django gives you a plethora of ready-made views, which you can extend by inheritance, to deal with generic pages, forms, lists of objects, and so on. When a view is run, it normally performs a task, which could be to retrieve objects from a database or to interpret data from a form, for example. Normally, the last step of the business logic of a view is to render a page to which it feeds the data that it has collected in its body.

That page, very often, if not always, comes from a template. A template is a file that mixes HTML code (or something equivalent), with Django template language code. This way, a view can collect some data and then pass it to the template. The template will know how to use it, to display it to the users who requested that page. Templates are the T in MTV.

In this short section, we are going to explore a couple of views and templates, written in Django, that are interfaced with our railway API. We are going to use the Requests library, from within Django, to access the API. Make sure you check out the source code in the apic folder if you want to see the Django project in its entirety.

Let's start from the view that displays stations:

```python
# apic/rails/views.py
...
from urllib.parse import urljoin
import requests
from django.conf import settings
from django.views import generic
from requests.exceptions import RequestException

class StationsView(generic.TemplateView):
    template_name = "rails/stations.html"

    def get(self, request, *args, **kwargs):
        context = self.get_context_data(**kwargs)
        api_url = urljoin(settings.BASE_API_URL, "stations")
        try:
            response = requests.get(api_url)
            response.raise_for_status()
        except RequestException as err:
            context["error"] = err
        else:
            context["stations"] = response.json()
        return self.render_to_response(context)
```

This view is a child of TemplateView, one of Django's generic views. In order to render a template with some data, all we need to do is to create a context dictionary and feed it to the render_to_response() method call. The template associated with this view is specified as a class attribute.

Notice how, after having gotten a context using the base class method get_context_ data(), we prepare the URL for the API by using urljoin() from the standard library. This function takes care of all the nasty details you need to be careful of when joining URLs.

We then try to talk to the API—in this case, we ask for all stations—and if the communication is successful, we put the JSON decoded response body in context["stations"], and then we render the template.

If the API responds with an error, we capture it in the err exception, and we put it in the context under the error key. This way, the view won't break and it will still be able to render the page correctly, but the context will be very different than the one for a successful scenario. We need to call raise_for_status() in order to make sure we get all those issues that are not related to our ability to talk to the API. If the communication with the API is successful, but for example we get back a status code 500 Internal Server Error, signaling something went wrong on the API internals, the try/except block won't catch it because as far as the request is concerned, everything worked fine. So, raise_for_status() comes to the rescue, and raises an appropriate exception when the status code is in the 4xx/5xx range.

By using RequestException, which is the base for all custom exceptions of the Requests library, we are sure we catch all errors that have to do with the process of communicating with the API through the Requests library. This might not be the best way to handle some scenarios, but for this simple example it is more than enough.

In order to run the Django project, make you sure activate the virtual environment for this chapter and then run this command from within the apic folder:
```bash
$ python manage.py runserver 8080
```

Notice how we set the port to 8080 instead of allowing Django to use the default, 8000, since the API is already running on that port, which is therefore not free. Please refer to the chapter's README.md file to learn how to set up and run all the code for this chapter.

Visiting http://localhost:8080/stations on a browser yields this result:

Figure 14.5: Fetching the list of stations from Django

In Figure 14.5, you can see the result of rendering the template assigned to the StationsView. Let's now see the main part of the template:

```html
# apic/rails/templates/rails/stations.html
{% extends "rails/base.html" %}
{% block content %}
{% if stations %}
  <h1>Stations</h1>
{% endif %}
{% for station in stations %}
  <div class="{% cycle 'bg_color1' 'bg_color2' %}">
    <p>Id: {{ station.id }} &lt;Code: {{ station.code }}
      ({{ station.city }}, {{ station.country }})&gt;&nbsp;
      <a href="{% url 'departures' station.id %}">Departures</a> -
      <a href="{% url 'arrivals' station.id %}"">Arrivals</a>
      </p>
  </div>
{% empty %}
  {% if error %}
    <div class=" error">
      <h3>Error</h3>
      <p>There was a problem connecting to the API.</p>
      <code>{{ error }}</code>
      <p>
        (<em>The above error is shown to the user as an example.
          For security reasons these errors are normally hidden
          from the user</em>)
      </p>
    </div>
  {% else %}
    <div>
      <p>There are no stations available at this time.</p>
    </div>
  {% endif %}
{% endfor %}
{% endblock %}
```

Notice how Django template tags, which are surrounded by curly braces, are interspersed with the HTML code that designs the structure of the page. The template starts by declaring the fact that it is extending the base template. This is quite common, and it is done so that the base template can host all the common boilerplate, which then doesn't have to be repeated in all templates. The base template declares sections called blocks, which other templates can override. In this case, we override the content block, which represents the main body of the page.

Remember we had a list of station objects, in the context, under the key stations.

So if we have any stations, we display an H1 title, and then we loop over the collection. Each entry gets its own div. We use the cycle Django tag to alternate row colors. We then simply write down the skeleton of each row and use the notation {{ variable }} to render a variable in that part of the template. Tags that represent commands that do something, like for, if, and block, are instead written like this: {% command %}.

Each row also creates two links that take you to the departures and the arrivals for each station, respectively. We don't want to hardcode the URL for those pages in a template, so we use the Django url tag to calculate those for us by inspecting how the URLs are set up within the urls module of the application. This has the advantage that if you change the URL for a certain page, you won't have to amend the templates that link to it, as the links are calculated for you by Django.

Should the collection be empty, the body of the empty tag will execute. In there, if there was an error, we display its content, along with some message for the user. Otherwise, we simply state that there are no stations to display. Notice how the empty tag belongs to the for loop section and helps to keep the logic of this part concise and clean. 

Clicking on the departures link for the Rome station leads to this:

Let's see the code that renders the page depicted in Figure 14.6. First, the view:
```python

# apic/rails/views.py
class DeparturesView(generic.TemplateView):
    template_name = "rails/departures.html"

    def get(self, request, station_id, *args, **kwargs):
        context = self.get_context_data(**kwargs)
        api_url = urljoin(
            settings.BASE_API_URL,
            f"stations/{station_id}/departures",
        )
        try:
            response = requests.get(api_url)
            response.raise_for_status()
        except RequestException as err:
            context["error"] = err
        else:
            trains = prepare_trains(response.json(), "departs_at")
            context["departures"] = trains
        return self.render_to_response(context)

def prepare_trains(trains: list[dict], key: str):
    return list(
        map(
            parse_datetimes,
            sorted(trains, key=itemgetter(key)),
        )
    )

def parse_datetimes(train: dict):
    train["arrives_at"] = datetime.fromisoformat(
        train["arrives_at"]
    )
    train["departs_at"] = datetime.fromisoformat(
        train["departs_at"]
    )
    return train
```

The code is very similar to the one for the StationsView. All that changes is the value of api_url, which now points to a specific station's departures. We do a bit of data manipulation with the datetime information for departures and arrivals. 

We parse that into Python datetime objects, so it's rendered in a nice, human- readable way in the template. The technique to talk to the API, catching any errors, is the same, and the template is also very similar, to a point that it's not worth going through its code here.

This short section was meant to show you a minimal example of how it is possible to consume an API programmatically, from another application. The Django project we wrote for this chapter only shows stations, departures and arrivals, and users. It also has a page that allows users to authenticate against the API. The idea behind this second mini-project we have added to this chapter is for you to be able to get a good grip on what's needed to talk to the API from a Django project, and then have plenty of room to expand and add functionalities yourself, both learning Django and possibly experimenting with API design at the same time.

## Where do we go from here?

Hopefully, by now, you should have wrapped your head around API design. Of course, you will have to check out the source code and spend some time on it, in order to deepen your understanding of this topic. Make sure you experiment, change things, break them, and see what happens. Here are some suggestions for you if you want to learn more on the subject:

•  Learn FastAPI. The website offers a great tutorial for beginners and one for advanced programmers, which we recommend. It will cover all the details and the reasons why things have to be done in a certain way, which we couldn't cover in this chapter.

•  Experiment with the source code for this chapter. We have written it as simply as we could, so there is lots of room for optimization.

•  Learn about WSGI—Web Server Gateway Interface—(https:// en.wikipedia.org/wiki/Web_Server_Gateway_Interface) and, if you are familiar with asynchronous programming, aWSGI, its asynchronous implementation (https://pypi.org/project/awsgi/).

•  Enhance the API, adding capabilities like advanced search, filtering, a more sophisticated auth system, background tasks, pagination, sorting, and so on. You can also expand the admin section by adding other endpoints only for admin users.

•  Learn about middleware in FastAPI, and about things like CORS (Cross- Origin Resource Sharing), which are important to know when we run an API in the real world.

•  Amend the endpoint that books a ticket so that its logic actually checks that there are free seats on the train. Each train specifies how many first- and second-class cars there are, as well as the number of seats per car. We designed the train model this way specifically to allow you to practice with this exercise. When you are done, you should also write tests to verify your code. 

•  Learn Django, and when you feel confident with it, try to recreate part of the Railway API using Django Rest Framework (https://www.django-rest-framework.org), which is a Django-based app used to write APIs.

•  Learn to write APIs using other technologies, such as Falcon (https://falcon.readthedocs.io/). See what they have in common with FastAPI, as that will improve your understanding of the concepts underlying API design, and the theory behind the frameworks.

•  Learn more about REST (Representational State Transfer) APIs, as they are everywhere these days, but there isn't just one way to write them, so you will have to compare your understanding of this topic with that of your colleagues or peers.

•  Learn about APIs in general. Learn about versioning, how to properly work with headers, data formats, and protocols, and so on.

•  Finally—and this might be a step you will be able to take in the future—explore the async side of FastAPI. It is interesting and can be slightly more performant if applied to the right use cases.

Remember to set DEBUG=true in the .env file, when working with the API, so that you get all the database queries logged automatically in your terminal, and you can check if the SQL code they produce actually reflects your intentions. This is quite a handy tool to have when SQLAlchemy operations become a bit more complex.

There are lots of resources on the Web, so you will be able to gather plenty of knowledge for free. API design is now a very important skill to master, so we can't stress enough how essential it is for you to dig deeper into this subject.

## Summary

In this chapter, we have explored the world of APIs. We started with a brief overview of the Web and moved on to a relatively recent Python topic: type hinting. The latter is interesting enough on its own, but in this case, it is a foundational characteristic of the FastAPI framework, so it was important to make sure we covered it in this chapter.

We then discussed APIs in generic terms. We saw different ways to classify them, and the purposes and benefits of their use. We also explored protocols and data- exchange formats.

Finally, we delved into the source code, analyzing a small part of the FastAPI project which was written for this chapter, and exploring different ways in which the resulting API can be consumed.

We concluded the chapter with a series of suggestions for the next steps, which we think are quite important, given the ubiquitous nature of this topic nowadays. 

The next chapter discusses packaging Python applications.
