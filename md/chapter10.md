# Testing

      "Just as the wise accepts gold after testing it by heating, cutting, and rubbing it, so are my words to be accepted after examining them, but not out of respect for me."

     – Buddha

We love this quote by the Buddha. Within the software world, it translates perfectly into the healthy habit of never trusting code just because someone smart wrote it or because it's been working fine for a long time. If it has not been tested, code is not to be trusted.

Why are tests so important? Well, for one, they give you predictability. Or, at least, they help you achieve high predictability. Unfortunately, there is always some bug that sneaks into the code. But we definitely want our code to be as predictable as possible. What we don't want is to have a surprise, in other words, our code behaving in an unpredictable way. Would you be happy to know that the software that checks the sensors of the plane that is taking you on your holiday sometimes goes crazy? No, probably not.

Therefore, we need to test our code; we need to check that its behavior is correct, that it works as expected when it deals with edge cases, that it doesn't hang when the components it's talking to are broken or unreachable, that the performance is well within the acceptable range, and so on.

This chapter is all about that—making sure that your code is prepared to face the scary outside world, that it is fast enough, and that it can deal with unexpected or exceptional conditions.

In this chapter, we're going to explore the following topics:

- General testing guidelines
- Unit testing
- A brief mention of test-driven development

Let's start by understanding what testing is.

# Testing your application

There are many different kinds of tests, so many, in fact, that companies often have a dedicated department, called quality assurance (QA), made up of individuals who spend their day testing the software the company developers produce.

To start making an initial classification, we can divide tests into two broad categories: white-box and black-box tests. White-box tests are those that exercise the internals of the code; they inspect it down to a very fine level of detail. On the other hand, black-box tests are those that consider the software under test as if within a box, the internals of which are ignored. Even the technology, or the language used inside the box, is not important for black-box tests. What they do is plug some input into one end of the box and verify the output at the other end—that's it.

There is also an in-between category, called gray-box testing, which involves testing a system in the same way we do with the black-box approach, but having some knowledge about the algorithms and data structures used to write the software and only partial access to its source code.

There are many different kinds of tests in these categories, each of which serves a different purpose. To give you an idea, here are a few:

- Frontend tests: They make sure that the client side of your application is exposing the information that it should, all the links, the buttons, the advertising, everything that needs to be shown to the client. They may also verify that it is possible to walk a certain path through the user interface.
- Scenario tests: They make use of stories (or scenarios) that help the tester work through a complex problem or test a part of the system.
- Integration tests: They verify the behavior of the various components of your application when they are working together sending messages through interfaces.
- Smoke tests: Particularly useful when you deploy a new update on your application. They check whether the most essential, vital parts of your application are still working as they should and that they are not on fire. This term comes from when engineers tested circuits by making sure nothing was smoking.
- Acceptance tests, or user acceptance testing (UAT): What a developer does with a product owner (for example, in a SCRUM environment) to determine whether the work that was commissioned was carried out correctly.
- Functional tests: They verify the features or functionalities of your software.
- Destructive tests: They take down parts of your system, simulating a failure, to establish how well the remaining parts of the system perform. These kinds of tests are performed extensively by companies that need to provide a highly reliable service.
- Performance tests: They aim to verify how well the system performs under a specific load of data or traffic so that, for example, engineers can get a better understanding of the bottlenecks in the system that could bring it to its knees in a heavy-load situation, or those that prevent scalability.
- Usability tests, and the closely related user experience (UX) tests: They aim to check whether the user interface is simple and easy to understand and use. They also aim to provide input to the designers so that the UX is improved.
- Security and penetration tests: They aim to verify how well the system is protected against attacks and intrusions.
- Unit tests: They help the developer write the code in a robust and consistent way, providing the first line of feedback and defense against coding mistakes, refactoring mistakes, and so on.
- Regression tests: They provide the developer with useful information about a feature being compromised in the system after an update. Some of the causes for a system being said to have a regression are an old bug resurfacing, an existing feature being compromised, or a new issue being introduced.

Many books and articles have been written about testing, and we have to point you to those resources if you're interested in finding out more about all the different kinds of tests. In this chapter, we will concentrate on unit tests, since they are the backbone of software crafting and form the vast majority of tests that are written by a developer.

Testing is an art, an art that you don't learn from books, I'm afraid. You can learn all the definitions (and you should), and try to collect as much knowledge about testing as you can, but you will likely be able to test your software properly only when you have accumulated enough experience.

When you are having trouble refactoring a bit of code, because every little thing you touch makes a test blow up, you learn how to write less rigid and limiting tests, which still verify the correctness of your code but, at the same time, allow you the freedom and joy to play with it, to shape it as you want.

When you are being called too often to fix unexpected bugs in your code, you learn how to write tests more thoroughly, how to come up with a more comprehensive list of edge cases, and strategies to cope with them before they turn into bugs.

When you are spending too much time reading tests and trying to refactor them to change a small feature in the code, you learn to write simpler, shorter, and better- focused tests.
We could go on with this when you... you learn..., but we guess you get the picture. You need to get your hands dirty and build experience. Our suggestion? Study the theory as much as you can, and then experiment using different approaches. Also, try to learn from experienced coders; it's very effective.


## The anatomy of a test
  
Before we concentrate on unit tests, let's see what a test is, and what its purpose is. A test is a piece of code whose purpose is to verify something in our system. It may be that we're calling a function passing two integers, that an object has a property called donald_duck, or that when you place an order on some API, after a minute you can see it dissected into its basic elements, in the database.

A test is typically composed of three sections:

- Preparation: This is where you set up the scene. You prepare all the data, the objects, and the services you need in the places you need them so that they are ready to be used.
- Execution: This is where you execute the bit of logic that you're checking against. You perform an action using the data and the interfaces you have set up in the preparation phase.
- Verification: This is where you verify the results and make sure they are according to your expectations. You check the returned value of a function, or that some data is in the database, some is not, some has changed, an HTTP request has been made, something has happened, a method has been called, and so on.

While tests usually follow this structure, in a test suite, you will typically find some other constructs that take part in the testing game:

- Setup: This is something quite commonly found in several different tests. It is logic that can be customized to run for every test, class, module, or even for a whole session. In this phase, developers usually set up connections to databases, maybe populate them with data that will be needed there for the test to make sense, and so on.

- Teardown: This is the opposite of the setup; the teardown phase takes place when the tests have been run. Like the setup, it can be customized to run for every test, class or module, or session. Typically, in this phase, we destroy any artifacts that were created for the test suite, and clean up after ourselves. This is important because we don't want to have any lingering objects around and because it helps to make sure that each test starts from a clean slate.

- Fixtures: These are pieces of data used in the tests. By using a specific set of fixtures, outcomes are predictable and therefore tests can perform verifications against them.

In this chapter, we will use the pytest Python library. It is a powerful tool that makes testing easier than it would be if we only used standard library tools. pytest provides plenty of helpers so that the test logic can focus more on the actual testing than the wiring and boilerplate around it. You will see, when we get to the code, that one of the characteristics of pytest is that fixtures, setup, and teardown often blend into one.


## Testing guidelines

Like software, tests can be good or bad, with a whole range of shades in the middle. To write good tests, here are some guidelines:

- Keep them as simple as possible. It's okay to violate some good coding rules, such as hardcoding values or duplicating code. Tests need, first and foremost, to be as readable as possible and easy to understand. When tests are hard to read or understand, you can never be confident they are actually making sure your code is performing correctly.

- Tests should verify one thing and one thing only. It's very important that you keep them short and contained. It's perfectly fine to write multiple tests to exercise a single object or function. Just make sure that each test has one and only one purpose.

- Tests should not make any unnecessary assumptions when verifying data. This is tricky to understand at first, but it is important. Verifying that the result of a function call is [1, 2, 3] is not the same as saying the output is a list that contains the numbers 1, 2, and 3. In the former, we're also assuming the ordering; in the latter, we're only assuming which items are in the list. The differences sometimes are quite subtle, but they are still very important.

- Tests should exercise the "what," rather than the "how." Tests should focus on checking what a function is supposed to do, rather than how it is doing it. For example, focus on the fact that a function is calculating the square root of a number (the what), instead of on the fact that it is calling math.sqrt() to do it (the how). Unless you're writing performance tests or you have a particular need to verify how a certain action is performed, try to avoid this type of testing and focus on the what. Testing the how leads to restrictive tests and makes refactoring hard. Moreover, the type of test you have to write when you concentrate on the how is more likely to degrade the quality of your testing codebase when you amend your software frequently.

- Tests should use the minimal set of fixtures needed to do the job. This is another crucial point. Fixtures have a tendency to grow over time. They also tend to change every now and then. If you use large amounts of fixtures and ignore redundancies in your tests, refactoring will take longer. Spotting bugs will be harder. Try to use a set of fixtures that is big enough for the test to perform correctly, but not any bigger.

- Tests should run as fast as possible. A good test codebase could end up being much longer than the code being tested itself. It varies according to the situation and the developer, but, whatever the length, you'll end up having hundreds, if not thousands, of tests to run, which means the faster they run, the faster you can get back to writing code. When using test-driven development (TDD), for example, you run tests very often, so speed is essential.

- Tests should use as few resources as possible. The reason for this is that every developer who checks out your code should be able to run your tests, no matter how powerful their machine is. It could be a skinny virtual machine or a CircleCI setup; your tests should run without chewing up too many resources.

CircleCI is one of the largest CI/CD (Continuous Integration/ Continuous Delivery) platforms available today. It is very easy to integrate with services like GitHub, for example. You have to add some configuration (typically in the form of a file) in your source code, and CircleCI will run tests when new code is prepared to be merged in the current codebase.


## Unit testing 

Now that we have an idea about what testing is and why we need it, let's introduce the developer's best friend: the unit test.

Before we proceed with the examples, allow us to share some words of caution: we will try to give you the fundamentals about unit testing, but we don't follow any particular school of thought or methodology to the letter. Over the years, we have tried many different testing approaches, eventually coming up with our own way of doing things, which is constantly evolving. To put it as Bruce Lee would have:

    "Absorb what is useful, discard what is useless, and add what is specifically your own."


### Writing a unit test

Unit tests take their name from the fact that they are used to test small units of code. To explain how to write a unit test, let's take a look at a simple snippet:

```python
# data.py
def get_clean_data(source): 
    data = load_data(source) 
    cleaned_data = clean_data(data) 
    return cleaned_data 
```

The get_clean_data() function is responsible for getting data from source, cleaning it, and returning it to the caller. How do we test this function?

One way of doing this is to call it and then make sure that load_data() was called once with source as its only argument. Then we have to verify that clean_data() was called once, with a return value of load_data. And, finally, we would need to make sure that a return value of clean_data is what is returned by the get_clean_data() function as well.

To do this, we need to set up the source and run this code, and this may be a problem. One of the golden rules of unit testing is that anything that crosses the boundaries of your application needs to be simulated. We don't want to talk to a real data source, and we don't want to actually run real functions if they are communicating with anything that is not contained in our application. A few examples would be a database, a search service, an external API, and a file in the filesystem.

We need these restrictions to act as a shield, so that we can always run our tests safely without the fear of destroying something in a real data source.

Another reason is that it may be quite difficult for a developer to reproduce the whole architecture on their machine. It may require the setting up of databases, APIs, services, files and folders, and so on, and this can be difficult, time-consuming, or sometimes not even possible.

Very simply put, an application programming interface (API) is a set of tools for building software applications. An API expresses a software component in terms of its operations, input and output, and underlying types. For example, if you create software that needs to interface with a data provider service, it's very likely that you will have to go through their API in order to gain access to the data.

Therefore, in our unit tests, we need to simulate all those things in some way. Unit tests need to be run by any developer without the need for the whole system to be set up on their machine.

A different approach, which we favor when it's possible to do so, is to simulate entities not by using fake objects, but using special-purpose test objects instead. For example, if our code talks to a database, instead of faking all the functions and methods that talk to the database and programming the fake objects so that they return what the real ones would, we would rather spawn a test database, set up the tables and data we need, and then patch the connection settings so that our tests are running real code against the test database. This is advantageous because if the underlying libraries change in a way that introduces an issue in our code, this methodology will catch this issue. A test will break. A test with mocks, on the other hand, will blissfully continue to run successfully, because the mocked interface would have no idea about the change in the underlying library. In-memory databases are excellent options for these cases.

One of the applications that allows you to spawn a database for testing is Django. Within the django.test package, you can find several tools that help you write your tests so that you won't have to simulate the dialog with a database. By writing tests this way, you will also be able to check on transactions, encodings, and all other database-related aspects of programming. Another advantage of this approach consists in the ability to check against things that can change from one database to another.

Sometimes, though, it's still not possible. For example, when the software interfaces with an API, and there is no test version of that API, we would need to simulate that API using fakes. In reality, most of the time we end up having to use a hybrid approach, where we use a test version of those technologies that allow this approach, and we use fakes for everything else. Let us now talk about fakes.


### Mock objects and patching

First of all, in Python, these fake objects are called mocks. Up to version 3.3, the mock library was a third-party library that basically every project would install via pip but, from version 3.3, it has been included in the standard library under the unittest module, and rightfully so, given its importance and how widespread it is.

The act of replacing a real object or function (or in general, any piece of data structure) with a mock is called patching. The mock library provides the patch tool, which can act as a function or class decorator, and even as a context manager that you can use to mock things out.


### Assertions

The verification phase is done through the use of assertions. In most cases, an assertion is a function or method that you can use to verify equality between objects, as well as other conditions. When a condition is not met, the assertion will raise an exception that will make your test fail. You can find a list of assertions in the unittest module documentation; however, when using pytest, you will typically use the generic assert statement, which makes things even simpler.


### Testing a CSV generator

Let's now adopt a practical approach. We will show you how to test a piece of code, and we will touch on the rest of the important concepts around unit testing within the context of this example.

We want to write an export function that does the following: it takes a list of dictionaries, each of which represents a user. It creates a CSV file, puts a header in it, and then proceeds to add all the users who are deemed valid according to some rules. The function will take three parameters: the list of user dictionaries, the name of the CSV file to create, and an indication of whether an existing file with the same name should be overwritten.

To be considered valid, and added to the output file, a user dictionary must satisfy the following requirements: each user must have at least an email, a name, and an age. There can also be a fourth field representing the role, but it's optional. The user's email address needs to be valid, the name needs to be non-empty, and the age must be an integer between 18 and 65.
  
This is our task, so now we are going to show you the code, and then we're going to analyze the tests we wrote for it. But, first things first, in the following code snippets, we will be using two third-party libraries: Marshmallow and Pytest. They are both in the requirements of the book's source code, so make sure you have installed them with pip.

Marshmallow is a wonderful library that provides us with the ability to serialize (or dump, in Marshmallow terminology) and deserialize (or load, in Marshmallow terminology) objects and, most importantly, gives us the ability to define a schema that we can use to validate a user dictionary. Pytest is one of the best pieces of software we have ever seen. It is used everywhere now, and has replaced other tools such as nose, for example. It provides us with great tools to write beautiful short tests. But let's get to the code. We called it api.py just because it exposes a function that we can use to do things. We will show it to you in chunks:

```python
# api.py
import os
import csv
from copy import deepcopy

from marshmallow import Schema, fields, pre_load
from marshmallow.validate import Length, Range

class UserSchema(Schema):
    """Represent a *valid* user. """

    email = fields.Email(required=True)
    name = fields.Str(required=True, validate=Length(min=1))
    age = fields.Int(
        required=True, validate=Range(min=18, max=65)
    )
    role = fields.String()

    @pre_load()
    def strip_name(self, data, **kwargs):
        data_copy = deepcopy(data)
        try:
            data_copy['name'] = data_copy['name'].strip()
        except (AttributeError, KeyError, TypeError):
            pass
        return data_copy

schema = UserSchema()
```

This first part is where we import all the modules we need (os, csv, and deepcopy), and some tools from marshmallow, and then we define the schema for the users. As you can see, we inherit from marshmallow.Schema, and then we set four fields. Notice we are using two string fields (Str), an Email, and an integer (Int). These will already provide us with some validation from marshmallow. Notice there is no required=True in the role field.

We need to add a couple of custom bits of code, though. We need to add validation on age to make sure the value is within the range we want. Marshmallow will raise ValidationError if it's not. It will also take care of raising an error should we pass anything but an integer.

We also add validation on name, because the fact that there is a name key in a dictionary doesn't guarantee that the value of that name is actually non-empty. We validate that the length of the field's value is at least one. Notice we don't need to add anything for the email field. This is because marshmallow will validate it for us.

After the fields declarations, we write another method, strip_name(), which is decorated with the pre_load() Marshmallow helper. This method will be run before Marshmallow deserializes (loads) the data. As you can see, we make a copy of data first, as in this context it is not a good idea to work directly on a mutable object, and then make sure we strip leading and trailing spaces away from data['name']. That key represents the name field we just declared above. We make sure we do this within a try/except block, so deserialization can run smoothly even in case of errors. The method returns the modified copy of data, and Marshmallow does the rest.

We then instantiate schema, so that we can use it to validate data. So, let's write the export function:

```python
# api.py
def export(filename, users, overwrite=True):
    """Export a CSV file.

    Create a CSV file and fill with valid users.  If 'overwrite'
    is False and file already exists, raise IOError.
    """
    if not overwrite and os.path.isfile(filename):
        raise IOError(f"'{filename}' already exists.")

    valid_users = get_valid_users(users)
    write_csv(filename, valid_users)
```

As you see, its internals are quite straightforward. If overwrite is False and the file already exists, we raise IOError with a message saying the file already exists. Otherwise, if we can proceed, we simply get the list of valid users and feed it to write_csv(), which is responsible for actually doing the job. Let's see how all these functions are defined:

```python
# api.py
def get_valid_users(users):
    """Yield one valid user at a time from users. """
    yield from filter(is_valid, users)

def is_valid(user):
    """Return whether or not the user is valid. """
    return not schema.validate(user)
```

Turns out we coded get_valid_users() as a generator, as there is no need to make a potentially big list in order to put it in a file. We can validate and save them one by one. The heart of validation is simply a delegation to schema.validate(), which uses the marshmallow validation engine. This method returns a dictionary, which is empty if the data is valid according to the schema or else it will contain error information. We don't really care about collecting the error information for this task, so we simply ignore it, and our is_valid() function simply returns True if the return value from schema.validate() is empty, and False otherwise.

One last piece is missing; here it is:

```python
# api.py
def write_csv(filename, users):
    """Write a CSV given a filename and a list of users.

    The users are assumed to be valid for the given CSV structure.
    """
    fieldnames = ['email', 'name', 'age', 'role']

    with open(filename, 'w', newline='') as csvfile:
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()

        for user in users:
            writer.writerow(user)
```

Again, the logic is straightforward. We define the header in fieldnames, then we open filename for writing, and we specify newline='', which is recommended in the documentation when dealing with CSV files. When the file has been created, we get a writer object by using the csv.DictWriter class. The beauty of this tool is that it is capable of mapping the user dictionaries to the field names, so we don't need to take care of the ordering.

We write the header first, and then we loop over the users and add them one by one. Notice, this function assumes it is fed a list of valid users, and it may break if that assumption is false (with the default values, it would break if any user dictionary had extra fields).

That's the whole code you have to keep in mind. We suggest you spend a moment going through it again. There is no need to memorize it, and the fact that we have used small helper functions with meaningful names will enable you to follow the testing along more easily.

Let's now get to the interesting part: testing our export() function. Once again, we will show you the code in chunks:

```python
# tests/test_api.py
import re
from unittest.mock import patch, mock_open, call
import pytest
from ch10.api import is_valid, export, write_csv
```

Let's start from the imports: first we bring in some tools from unittest.mock, then pytest, and, finally, we fetch the three functions that we want to actually test: is_valid(), export(), and write_csv(). We also import the re module from the standard library, as it will be needed in one of the tests.

Before we can write tests, though, we need to make a few fixtures. As you will see, a fixture is a function that is decorated with the pytest.fixture decorator. They are run before each test to which they are applied. In most cases, we expect fixture to return something so that we can use it in a test. We have some requirements for a user dictionary, so let's write a couple of users: one with minimal requirements, and one with full requirements. Both need to be valid. Here is the code:

```python
# tests/test_api.py
@pytest.fixture
def min_user():
    """Represents a valid user with minimal data. """
    return {
        'email': 'minimal@example.com',
        'name': 'Primus Minimus',
        'age': 18,
    }

@pytest.fixture
def full_user():
    """Represents a valid user with full data. """
    return {
        'email': 'full@example.com',
        'name': 'Maximus Plenus',
        'age': 65,
        'role': 'emperor',
    }
```

In this example, the only difference between the users is the presence of the role key, but it's enough to show you the point, we hope.

Notice that instead of simply declaring dictionaries at a module level, we have actually written two functions that return a dictionary, and we have decorated them with the @pytest.fixture decorator. This is because when you declare a dictionary that is supposed to be used in your tests at the module level, you need to make sure you copy it at the beginning of every test. If you don't, you may have a test that modifies it, and this will affect all tests that follow it, compromising their integrity. By using these fixtures, pytest will give us a new dictionary every test run, so we don't need to go through that copy procedure. This helps to respect the principle of independence, which says that each test should be self-contained and independent.

If a fixture returns another type instead of dict, then that is what you will get in the test. Fixtures are also composable, which means they can be used in one another, which is a very powerful feature of pytest. To show you this, let's write a fixture for a list of users, in which we put the two we already have, plus one that would fail validation because it has no age. Let's take a look at the following code:

```python
# tests/test_api.py
@pytest.fixture
def users(min_user, full_user):
    """List of users, two valid and one invalid. """
    bad_user = {
        'email': 'invalid@example.com',
        'name': 'Horribilis',
    }
    return [min_user, bad_user, full_user]
```

So, now we have two users that we can use individually, and we also have a list of three users.

The first round of tests will be testing how we validate a user. We will group all the tests for this task within a class. This helps to give related tests a namespace, a place to be. As we'll see later on, it also allows us to declare class-level fixtures, which are defined just for the tests belonging to the class. One of the perks of declaring a fixture at a class level is that you can easily override one with the same name that lives outside the scope of the class. Take a look at this code:

```python
# tests/test_api.py
class TestIsValid:
    """Test how code verifies whether a user is valid or not. """

    def test_minimal(self, min_user):
        assert is_valid(min_user)

    def test_full(self, full_user):
        assert is_valid(full_user)
```

We start very simply by making sure our fixtures actually pass validation. This helps ensure that our code will correctly validate users that we know to be valid, with minimal as well as full data. Notice that we gave each test function a parameter matching the name of a fixture. This has the effect of activating the fixture for the test. When pytest runs the tests, it will inspect the parameters of each test and pass the return values of the corresponding fixture functions as arguments to the test.

Next, we are going to test the age. Two things to notice here: we will not repeat the class signature, so the code that follows is indented by four spaces because these are all methods within the same class. Also, we're going to use parametrization. Parametrization is a technique that enables us to run the same test multiple times, but feeding different data to it. It is very useful as it allows us to write the test only once with no repetition, and the result will be very intelligently handled by pytest, which will run all those tests as if they were actually separate, thus providing us with clear error messages when they fail. Another solution would be to write one test with a for loop inside that runs through all the pieces of data we want to test against. The latter solution is of much lower quality though, as the framework won't be able to give you specific information as if you were running separate tests. Moreover, should any of the for loop iterations fail, there would be no information about what would have happened after that, as subsequent iterations would not happen. Finally, the body of the test would get more difficult to understand, due to the for loop extra logic. Therefore, parametrization is a far superior choice for this use case. 

It spares us from having to write a bunch of almost identical tests to exhaust all possible scenarios. Let's see how we test the age: 

```python
# tests/test_api.py
    @pytest.mark.parametrize('age', range(18))
    def test_invalid_age_too_young(self, age, min_user):
        min_user['age'] = age
        assert not is_valid(min_user)
```

Right, so we start by writing a test to check that validation fails when the user is too young. According to our rule, a user is too young when they are younger than 18. We check for every age between 0 and 17, by using range().

If you take a look at how the parametrization works, you see that we declare the name of an object, which we then pass to the signature of the method, and then we specify which values this object will take. For each value, the test will be run once. In the case of this first test, the object's name is age, and the values are all those returned by range(18), which means all integer numbers from 0 to 17 are included. Notice how we feed age to the test method, right after self.

We also use the min_user fixture in this test. In this case, we change the age within the min_user dictionary, and then we verify that the result of is_valid(min_ user) is False. We do this last bit by asserting on the fact that not False is True. In pytest, this is how you check for something. You simply assert that something is truthy. If that is the case, the test has succeeded. Should it instead be the opposite, the test will fail.

Note that pytest will re-evaluate the fixture function for each test run that uses it, so we are free to modify the fixture data within the test without affecting any other tests.

Let's proceed and add all the tests needed to make validation fail on the age:
```python

# tests/test_api.py
    @pytest.mark.parametrize('age', range(66, 100))
    def test_invalid_age_too_old(self, age, min_user):
        min_user['age'] = age
        assert not is_valid(min_user)

    @pytest.mark.parametrize('age', ['NaN', 3.1415, None])
    def test_invalid_age_wrong_type(self, age, min_user):
        min_user['age'] = age
        assert not is_valid(min_user)
```

So, another two tests. One takes care of the other end of the spectrum, from 66 years of age to 99. And the second one instead makes sure that age is invalid when it's not an integer number, so we pass some values, such as a string, a float, and None, just to make sure. Notice how the structure of the test is basically always the same, but, thanks to the parametrization, we feed very different input arguments to it.

Now that we have the age-failing all sorted out, let's add a test that actually checks the age is within the valid range:

```python

# tests/test_api.py
    @pytest.mark.parametrize('age', range(18, 66))
    def test_valid_age(self, age, min_user):
        min_user['age'] = age

        assert is_valid(min_user)
```

It's as easy as that. We pass the correct range, from 18 to 65, and remove the not in the assertion. Notice how all tests start with the test_ prefix, so that pytest can discover them, and have a different name.

We can consider the age as being taken care of. Let's move on to write tests on mandatory fields:

```python

# tests/test_api.py
    @pytest.mark.parametrize('field', ['email', 'name', 'age'])
    def test_mandatory_fields(self, field, min_user):
        del min_user[field]
        assert not is_valid(min_user)

    @pytest.mark.parametrize('field', ['email', 'name', 'age'])
    def test_mandatory_fields_empty(self, field, min_user):
        min_user[field] = ''
        assert not is_valid(min_user)

    def test_name_whitespace_only(self, min_user):
        min_user['name'] = ' \n\t'
        assert not is_valid(min_user)
```

These three tests still belong to the same class. The first one tests whether a user is invalid when one of the mandatory fields is missing. Notice that at every test run, the min_user fixture is restored, so we only have one missing field per test run, which is the appropriate way to check for mandatory fields. We simply remove the key from the dictionary. This time the parametrization object takes the name field, and, by looking at the first test, you see all the mandatory fields in the parametrization decorator: email, name, and age.

In the second one, things are a little different. Instead of removing keys, we simply set them (one at a time) to the empty string. Finally, in the third one, we check for the name to be made of whitespace only.

The previous tests take care of mandatory fields being there and being non-empty, and of the formatting around the name key of a user. Good. Let's now write the last two tests for this class. We want to check that email is valid, and in the second one, the type for email, name, and the role:

```python
# tests/test_api.py
    @pytest.mark.parametrize(
        'email, outcome',
        [
            ('missing_at.com', False),
            ('@missing_start.com', False),
            ('missing_end@', False),
            ('missing_dot@example', False),
            ('good.one@example.com', True),
            ('δοκιμή@παράδειγμα.δοκιμή', True),
            ('аджай@экзампл.рус', True),
        ]
    )
    def test_email(self, email, outcome, min_user):
        min_user['email'] = email
        assert is_valid(min_user) == outcome
```

This time, the parametrization is slightly more complex. We define two objects (email and outcome) and then we pass a list of tuples, instead of a simple list, to the decorator. Each time the test is run, one of those tuples will be unpacked to fill the values of email and outcome, respectively. This allows us to write one test for both valid and invalid email addresses, instead of two separate ones. We define an email address, and we specify the outcome we expect from validation. The first four are invalid email addresses, but the last three are actually valid. We have used a couple of examples with non-ASCII characters, just to make sure we're not forgetting to include our friends from all over the world in the validation.

Notice how the validation is done, asserting that the result of the call needs to match the outcome we have set.

Let's now write a simple test to make sure validation fails when we feed the wrong type to the fields (again, the age has been taken care of separately before):

```python
# tests/test_api.py
    @pytest.mark.parametrize(
        'field, value',
        [
            ('email', None),
            ('email', 3.1415),
            ('email', {}),
            ('name', None),
            ('name', 3.1415),
            ('name', {}),
            ('role', None),
            ('role', 3.1415),
            ('role', {}),
        ]
    )
  def test_invalid_types(self, field, value, min_user):
        min_user[field] = value
        assert not is_valid(min_user)
```

As we did before, just for fun, we pass three different values, none of which is actually a string. This test could be expanded to include more values, but, honestly, we shouldn't need to write tests such as this one. We have included it here just to show you what's possible, but normally you would need to focus on making sure the code considers valid types those that have to be considered valid, and that should be enough.

Before we move to the next test class, let take a moment to talk a bit more about something we briefly touched on when testing the age.


### Boundaries and granularity

While checking for the age, we wrote three tests to cover the three ranges: 0-17 (fail), 18-65 (success), and 66-99 (fail). Why did we do this? The answer lies in the fact that we are dealing with two boundaries: 18 and 65. So our testing needs to focus on the three regions those two boundaries define: before 18, within 18 and 65, and after 65. How you do it is not crucial, as long as you make sure you test the boundaries correctly. This means if someone changes the validation in the schema from 18 <= value <= 65 to 18 <= value < 65 (notice the second <= is now <), there must be a test that fails on 65. This concept is known as a boundary, and it's very important that you recognize them in your code so that you can test against them.

Another important thing is to understand which zoom level we want in order to get close to the boundaries. In other words, which unit should I use to move around them? 

In the case of age, we're dealing with integers, so a unit of 1 will be the perfect choice (which is why we used 16, 17, 18, 19, 20, ...). But what if you were testing for a timestamp? Well, in that case, the correct granularity will likely be different. If the code has to act differently according to your timestamp and that timestamp represents seconds, then the granularity of your tests should zoom down to seconds. If the timestamp represents years, then years should be the unit you use. We hope you get the picture. This concept is known as granularity and needs to be combined with that of boundaries, so that by going around the boundaries with the correct granularity, you can make sure your tests are not leaving anything to chance.

Let's now continue with our example, and test the export function.

### Testing the export function

In the same test module, we defined another class that represents a test suite for 
the export() function. Here it is:
```python

# tests/test_api.py
class TestExport:
    """Test behavior of 'export' function. """

    @pytest.fixture
    def csv_file(self, tmp_path):
        """Yield a filename in a temporary folder.
        Due to how pytest 'tmp_path' fixture works, the file does
        not exist yet.
        """
        yield tmp_path / "out.csv"

    @pytest.fixture
    def existing_file(self, tmp_path):
        """Create a temporary file and put some content in it. """
        existing = tmp_path / 'existing.csv'
        existing.write_text('Please leave me alone...')
        yield existing
```

Let's start by analyzing the fixtures. We have defined them at the class level this time, which means they will be alive only for as long as the tests in the class are running. We don't need these fixtures outside of this class, so it doesn't make sense to declare them at a module level like we've done with the user ones.

So, we need two files. If you recall what we wrote at the beginning of this chapter, when it comes to interaction with databases, disks, networks, and so on, we should mock everything out. However, when possible, we prefer to use a different technique. In this case, we will employ temporary folders, which will be created and deleted within the fixture. We are much happier if we can avoid mocking. To create temporary folders, we employ the tmp_path fixture, from pytest, which is a pathlib.Path object.

Now, the first fixture, csv_file, provides a reference to a temporary folder. We can consider the logic up to and including the yield as the setup phase. The fixture itself, in terms of data, is represented by the temporary filename. The file itself is not present yet. When a test runs, the fixture is created, and at the end of the test, the rest of the fixture code (the part after yield, if any) is executed. That part can be considered the teardown phase. In the case of the csv_file fixture, it consists of exiting the body of the function, which means the temporary folder is deleted, along with all its content. You can put much more in each phase of any fixture, and with experience, you will master the art of doing setup and teardown this way. It actually comes very naturally quite quickly.

The second fixture is very similar to the first one, but we'll use it to test that we can prevent overwriting when we call export with overwrite=False. So, we create a file in the temporary folder, and we put some content into it, just to have the means to verify it hasn't been touched.

Let's now see the tests (as we did before, we indent to remind you they are defined in the same class):

```python
# tests/test_api.py
    def test_export(self, users, csv_file):
        export(csv_file, users)
        text = csv_file.read_text()

        assert (
            'email,name,age,role\n'
            'minimal@example.com,Primus Minimus,18,\n'
            'full@example.com,Maximus Plenus,65,emperor\n'
         ) == text
```

This test employs the users and csv_file fixtures, and immediately calls export() with them. We expect that a file has been created, and populated with the two valid users we have (remember the list contains three users, but one is invalid).

To verify that, we open the temporary file, and collect all its text into a string. We then compare the content of the file with what we expect to be in it. Notice we only put the header, and the two valid users, in the correct order.

Now we need another test to make sure that if there is a comma in one of the values, our CSV is still generated correctly. Being a comma-separated values (CSV) file, we need to make sure that a comma in the data doesn't break things up:

```python
# tests/test_api.py
    def test_export_quoting(self, min_user, csv_file):
        min_user['name'] = 'A name, with a comma'
        export(csv_file, [min_user])
        text = csv_file.read_text()

        assert (
            'email,name,age,role\n'
            'minimal@example.com,"A name, with a comma",18,\n'
         ) == text
```

This time, we don't need the whole users list; we just need one, as we're testing a specific thing and we have the previous test to make sure we're generating the file correctly with all the users. Remember, always try to minimize the work you do within a test.

So, we use min_user, and put a nice comma in its name. We then repeat the procedure, which is very similar to that of the previous test, and finally we make sure that the name is put in the CSV file surrounded by double quotes. This is enough for any good CSV parser to understand that they don't have to break on the comma inside the double quotes.

Now, we want one more test, which needs to check that when the file already exists and we don't want to override it, our code won't do that:

```python
# tests/test_api.py
    def test_does_not_overwrite(self, users, existing_file):
        with pytest.raises(IOError) as err:
            export(existing_file, users, overwrite=False)
        err.match(
            r"'{}' already exists\.".format(
                re.escape(str(existing_file))
            )
        )
        # let's also verify the file is still intact
        assert existing_file.read() == 'Please leave me alone...'
```

This is a beautiful test, because it allows us to show you how you can tell pytest that you expect a function call to raise an exception. We do it in the context manager given to us by pytest.raises, to which we feed the exception we expect from the call we make inside the body of that context manager. If the exception is not raised, the test will fail.

We like to be thorough in our tests, so we don't want to stop there. We also assert on the message, by using the convenient err.match helper. Notice that we don't need to use an assert statement when calling err.match. If the argument doesn't match, the call will raise an AssertionError, causing the test to fail. We also need to escape the string version of existing_file because on Windows, paths have backslashes, which would confuse the regular expression we feed to err.match().

Finally, we make sure that the file still contains its original content (which is why we created the existing_file fixture) by opening it, and comparing all of its content to the string it should be.


### Final considerations

Before we move on to the next topic, let's wrap up with some considerations.

First, we hope you have noticed that we haven't tested all the functions we wrote. Specifically, we didn't test get_valid_users, validate, and write_csv. The reason is that these functions are already implicitly tested by our test suite. We have tested is_ valid() and export(), which is more than enough to make sure our schema is validating users correctly, and the export() function is dealing with filtering out invalid users correctly, respecting existing files when needed, and writing a proper CSV. The functions we haven't tested are the internals; they provide logic that participates in doing something that we have thoroughly tested anyway. Would adding extra tests for those functions be good or bad? Think about it for a moment.

The answer is actually difficult. The more we test, the less easily we can refactor that code. As it is now, we could easily decide to rename validate(), and we wouldn't have to change any of the tests we wrote. If you think about it, it makes sense, because as long as validate() provides correct validation to the  get_valid_users() function, we don't really need to know about it. 

If, instead, we had written tests for the validate() function, then we would have to change them had we decided to rename it (or to change its signature, for example).

So, what is the right thing to do? Tests or no tests? It will be up to you. You have to find the right balance. Our personal take on this matter is that everything needs to be thoroughly tested, either directly or indirectly. And we try to write the smallest possible test suite that guarantees that. This way, we will have a great test suite in terms of coverage, but not any bigger than necessary. We need to maintain those tests!

We hope this example made sense to you; we think it has allowed us to touch on the important topics.

If you check out the source code for the book, in the test_api.py module, you will find a couple of extra test classes that will show you how different testing would have been had we decided to go all the way with the mocks. Make sure you read that code and understand it well. It is quite straightforward and will offer you a good comparison with the approach we have shown you here.

Now, how about we run those tests?

```bash

$ pytest tests
========================= test session starts==========================
platform darwin -- Python 3.9.4, pytest-6.2.4, py-1.10.0, pluggy-0.13.1
rootdir: /Users/fab/.../ch10
collected 132 items

tests/test_api.py .............................................. [ 34%]
................................................................ [ 83%]
......................                                           [100%]

========================= 132 passed in 0.31s==========================

```

Make sure you run $ pytest test from within the ch10 folder (add the -vv flag for a verbose output that will show you how parametrization modifies the names of your tests). pytest scans your files and folders, searching for modules that start or end with test_, like test_*.py, or *_test.py. Within those modules, it grabs test- prefixed functions or test-prefixed methods inside Test-prefixed classes (you can read the full specification in the pytest documentation). As you can see, 132 tests were run in less than half a second, and they all succeeded. We strongly suggest you check out this code and play with it. Change something in the code and see whether any test is breaking. Understand why it is breaking. Is it something important that means the test isn't good enough? Or is it something silly that shouldn't cause the test to break? All these apparently innocuous questions will help you gain deep insight into the art of testing.

We also suggest you study the unittest module, and the pytest library too. These are tools you will use all the time, so you need to be very familiar with them.

Let's now check out test-driven development!

## Test-driven development

Let's talk briefly about test-driven development (TDD). It is a methodology that was rediscovered by Kent Beck, who wrote Test-Driven Development by Example, Addison Wesley, 2002, which we encourage you to read if you want to learn about the fundamentals of this subject.

TDD is a software development methodology that is based on the continuous repetition of a very short development cycle.

First, the developer writes a test, and makes it run. The test is supposed to check a feature that is not yet part of the code. Maybe it is a new feature to be added, or something to be removed or amended. Running the test will make it fail and, because of this, this phase is called Red.

When the test has failed, the developer writes the minimal amount of code to make it pass. When running the test succeeds, we have the so-called Green phase. In this phase, it is okay to write code that cheats, just to make the test pass. This technique is called fake it 'til you make it. In a second iteration of the TDD cycle, tests are enriched with different edge cases, and the cheating code then has to be rewritten with proper logic. Adding other test cases is sometimes called triangulation.

The last piece of the cycle is where the developer takes care of refactoring code and tests until they are in the desired state. This last phase is called Refactor. The TDD mantra therefore is Red-Green-Refactor.

At first, it might feel weird to write tests before the code, and we must confess it took us a while to get used to it. If you stick to it, though, and force yourself to learn this slightly counterintuitive way of working, at some point something almost magical happens, and you will see the quality of your code increase in a way that wouldn't be possible otherwise.

When we write our code before the tests, we have to take care of what the code has to do and how it has to do it, both at the same time. On the other hand, when we write tests before the code, we can concentrate on the what part alone while we are writing them. When we write the code afterward, we will mostly have to take care of how the code has to implement what is required by the tests. This shift in focus allows our minds to concentrate on the what and how parts separately, yielding a brainpower boost that can feel quite surprising.

There are several other benefits that come from the adoption of this technique:

- You will refactor with much more confidence: Tests will break if you introduce bugs. Moreover, an architectural refactor will also benefit from having tests that act as guardians.

- The code will be more readable: This is crucial in a time when coding is a social activity and every professional developer spends much more time reading code than writing it.

- The code will be more loosely coupled and easier to test and maintain: Writing the tests first forces you to think more deeply about code structure.

- Writing tests first requires you to have a better understanding of the business requirements: If your understanding of the requirements is lacking, you'll find writing a test extremely challenging and this situation acts as a sentinel for you.

- Having everything unit tested means the code will be easier to debug: Moreover, small tests are perfect for providing alternative documentation. English can be misleading, but five lines of Python in a simple test are very hard to misunderstand.

- Higher speed: It's faster to write tests and code than it is to write the code first and then lose time debugging it. If you don't write tests, you will probably deliver the code sooner, but then you will have to track the bugs down and solve them (and, rest assured, there will be bugs). The combined time taken to write the code and then debug it is usually longer than the time taken to develop the code with TDD, where having tests running before the code is written ensures that the number of bugs in it will be much lower than in the other case.

On the other hand, there are some shortcomings of this technique:

- The whole company needs to believe in it: Otherwise, you will have to constantly argue with your boss, who will not understand why it takes you so long to deliver. The truth is, it may take you a bit longer to deliver in the short term, but in the long term, you gain a lot with TDD. However, it is quite hard to see the long term because it's not under our noses like the short term is. We have fought battles with stubborn bosses in our career to be able to code using TDD. Sometimes it has been painful, but always well worth it, and we have never regretted it because, in the end, the quality of the result has always been appreciated.
-If you fail to understand the business requirements, this will reflect in the tests you write, and it will therefore reflect in the code too: This kind of problem is quite hard to spot until you do user acceptance testing, but one thing that you can do to reduce the likelihood of it happening is to pair with another developer. Pairing will inevitably require discussions about the business requirements, and discussion will bring clarification, which will help with writing correct tests.

- Badly written tests are hard to maintain: This is a fact. Tests with too many mocks or with extra assumptions or badly structured data will soon become a burden. Don't let this discourage you; just keep experimenting and change the way you write them until you find a way that doesn't require a huge amount of work every time you touch your code.

We are quite passionate about TDD. When we interview for a job, one of the questions we ask is whether the company adopts it. We encourage you to check it out and use it. Use it until you feel something clicking in your mind. When that happens, then you will be free to use it according to your own judgment of the situation. Regardless of which order you write code and tests in, though, the most important thing is that you always test your code!

## Summary

In this chapter, we explored the world of testing.

We tried to give you a fairly comprehensive overview of testing, especially unit testing, which is the kind of testing that a developer mostly does. We hope we have succeeded in conveying the message that testing is not something that is perfectly defined and that you can learn from a book. You need to experiment with it a lot before you get comfortable. Of all the efforts a coder must make in terms of study and experimentation, we would say testing is the one that is the most important.

In the next chapter, we're going to explore debugging and profiling, which are techniques that go hand in hand with testing, and are crucial to learn well.

We are aware that we gave you a lot of pointers in this chapter, with no links or directions. This was by choice. As a coder, there won't be a single day at work when you won't have to look something up on a documentation page, in a manual, on a website, and so on. We think it's vital for a coder to be able to search effectively for the information they need, so we hope you'll forgive us for this extra training. After all, it is for your benefit.
