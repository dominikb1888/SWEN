# Files and Data Persistence

    "It's not that I'm so smart, it's just that I stay with problems longer."

    – Albert Einstein

In the previous chapters, we have explored several different aspects of Python. As the examples have a didactic purpose, we've run them in a simple Python shell or in the form of a Python module. They ran, maybe printed something on the console, and then they terminated, leaving no trace of their brief existence.

Real-world applications, though, are rather different. Naturally, they still run in memory, but they interact with networks, disks, and databases. They also exchange information with other applications and devices, using formats that are suitable for the situation.

In this chapter, we are going to start closing in on the real world by exploring the following:

-  Files and directories
-  Compression
-  Networks and streams
-  The JSON data-interchange format
-  Data persistence with pickle and shelve, from the standard library
-  Data persistence with SQLAlchemy

As usual, we will try to balance breadth and depth so that by the end of the chapter, you will have a solid grasp of the fundamentals and will know how to fetch further information on the web.

## TOC

- [Files and Data Persistence](#files-and-data-persistence)
- [TOC](#toc)
  - [Working with files and directories](#working-with-files-and-directories)
    - [Opening files](#opening-files)
    - [Using a context manager to open a file](#using-a-context-manager-to-open-a-file)
    - [Reading and writing to a file](#reading-and-writing-to-a-file)
    - [Reading and writing in binary mode](#reading-and-writing-in-binary-mode)
    - [Protecting against overwriting an existing file](#protecting-against-overwriting-an-existing-file)
    - [Checking for file and directory existence](#checking-for-file-and-directory-existence)
    - [Manipulating files and directories](#manipulating-files-and-directories)
    - [Manipulating pathnames](#manipulating-pathnames)
    - [Temporary files and directories](#temporary-files-and-directories)
    - [Directory content](#directory-content)
    - [File and directory compression](#file-and-directory-compression)
  - [Data interchange formats](#data-interchange-formats)
    - [Working with JSON](#working-with-json)
    - [Custom encoding/decoding with JSON](#custom-encodingdecoding-with-json)
  - [I/O, streams, and requests](#io-streams-and-requests)
    - [Using an in-memory stream](#using-an-in-memory-stream)
    - [Making HTTP requests](#making-http-requests)
  - [Persisting data on disk](#persisting-data-on-disk)
    - [Serializing data with pickle](#serializing-data-with-pickle)
    - [Saving data with shelve](#saving-data-with-shelve)
    - [Saving data to a database](#saving-data-to-a-database)
  - [Summary](#summary)


## Working with files and directories

When it comes to files and directories, Python offers plenty of useful tools. In particular, in the following examples, we will leverage the os, pathlib, and shutil modules. As we'll be reading and writing on the disk, we will be using a file, fear.txt, which contains an excerpt from Fear, by Thich Nhat Hanh, as a guinea pig for some of our examples.


### Opening files

Opening a file in Python is very simple and intuitive. In fact, we just need to use the open() function. Let's see a quick example:

```python
# files/open_try.py
fh = open('fear.txt', 'rt')  # r: read, t: text

for line in fh.readlines():
    print(line.strip())  # remove whitespace and print

fh.close()
```

The previous code is very simple. We call open(), passing the filename, and telling open() that we want to read it in text mode. There is no path information before the filename; therefore, open() will assume the file is in the same folder the script is run from. This means that if we run this script from outside the files folder, then fear.txt won't be found.

Once the file has been opened, we obtain a file object back, fh, which we can use to work on the content of the file. In this case, we use the readlines() method to iterate over all the lines in the file, and print them. We call strip() on each line to get rid of any extra spaces around the content, including the line termination character at the end, since print will already add one for us. This is a quick and dirty solution that works in this example, but should the content of the file contain meaningful spaces that need to be preserved, you will have to be slightly more careful in how you sanitize the data. At the end of the script, we close the stream.

Closing a file is very important, as we don't want to risk failing to release the handle we have on it. When that happens, you can encounter issues such as memory leaks, or the annoying "you can't delete this file" pop-up that informs you that some software is still using it. Therefore, we need to apply some precautions, and wrap the previous logic in a try/finally block. This means that, whatever error might occur when we try to open and read the file, we can rest assured that close() will be called:

```python

# files/open_try.py

fh = open('fear.txt', 'rt')

try:
    for line in fh.readlines():
        print(line.strip())
finally:
    fh.close()

```

The logic is exactly the same, but now it is also safe. If you are not familiar with the try/finally block, make sure you go back to Chapter 7, Exceptions and Context Managers, and study it.

We can simplify the previous example further, this way:

```python

# files/open_try.py

fh = open('fear.txt')  # rt is default

try:
    for line in fh:  # we can iterate directly on fh
        print(line.strip())
finally:
    fh.close()

```

As you can see, rt is the default mode for opening files, so we don't need to specify it. Moreover, we can simply iterate on fh, without explicitly calling readlines() on it. Python is very nice and gives us shorthands to make our code shorter and simpler to read.

All the previous examples produce a print of the file on the console (check out the source code to read the whole content):

    An excerpt from Fear - By Thich Nhat Hanh

    The Present Is Free from Fear

    When we are not fully present, we are not really living. We're not 
    really there, either for our loved ones or for ourselves. If we're not 
    there, then where are we? We are running, running, running, even during 
    our sleep. We run because we're trying to escape from our fear.
    ...

### Using a context manager to open a file

Let's admit it: the prospect of having to disseminate our code with try/finally blocks is not one of the best. As usual, Python gives us a much nicer way to open a file in a secure fashion: by using a context manager. Let's see the code first:

```python
# files/open_with.py
with open('fear.txt') as fh:
    for line in fh:
        print(line.strip())
```

This example is equivalent to the previous one, but reads so much better. The open() function is capable of producing a file object when invoked by a context manager, but the true beauty of it lies in the fact that fh.close() will be called automatically for us, even in the case of errors.


### Reading and writing to a file

Now that we know how to open a file, let's see a couple of different ways in which we can read and write to it:

```python
# files/print_file.py
with open('print_example.txt', 'w') as fw:
    print('Hey I am printing into a file!!!', file=fw)
```

A first approach uses the print() function, which we've seen plenty of times in the previous chapters. After obtaining a file object, this time specifying that we intend to write to it ('w'), we can tell the call to print() to direct its output to the file, instead of to the standard output stream as it normally does.
  
In Python, the standard input, output, and error streams are represented by the file objects sys.stdin, sys.stdout, and sys.stderr. Unless input or output is redirected, reading from sys.stdin usually corresponds to reading from the keyboard and writing to sys.stdout or sys.stderr usually prints to the console screen.

The previous code has the effect of creating the print_example.txt file if it doesn't exist, or truncating it in case if, and writes the line Hey I am printing into a file!!! into it.

Truncating a file means erasing its contents without deleting it. After truncation, the file still exists on the filesystem, but it's empty.

This is all nice and easy, but not what we typically do when we want to write to a file. Let's see a much more common approach:

```python
# files/read_write.py
with open('fear.txt') as f:
    lines = [line.rstrip() for line in f]

with open('fear_copy.txt', 'w') as fw:
    fw.write('\n'.join(lines))
```
 
In this example, we first open fear.txt and collect its content into a list, line by line. Notice that this time, we are calling a different method, rstrip(), as an example, to make sure we only strip the whitespace on the right-hand side of every line. In the second part of the snippet, we create a new file, fear_copy.txt, and we write to it all the lines from the original file, joined by a newline, \n. Python is gracious and works by default with universal newlines, which means that even though the original file might have a newline that is different to \n, it will be translated automatically for us before the line is returned. This behavior is, of course, customizable, but normally it is exactly what you want. Speaking of newlines, can you think of one that might be missing in the copy?

### Reading and writing in binary mode

Notice that by opening a file passing t in the options (or omitting it, as it is the default), we're opening the file in text mode. This means that the content of the file is treated and interpreted as text. 

If you wish to write bytes to a file, you can open it in binary mode. This is a common requirement when you deal with files that don't just contain raw text, such as images, audio/video, and, in general, any other proprietary format.

In order to handle files in binary mode, simply specify the b flag when opening them, as in the following example:

```python
# files/read_write_bin.py
with open('example.bin', 'wb') as fw:
    fw.write(b'This is binary data...')

with open('example.bin', 'rb') as f:
    print(f.read())  # prints: b'This is binary data...'

```

In this example, we are still using text as binary data, for simplicity, but it could be anything you want. You can see it's treated as binary by the fact that you get the b'This ...' prefix in the output.


### Protecting against overwriting an existing file

As we have seen, Python gives us the ability to open files for writing. By using the w flag, we open a file and truncate its content. This means the file is overwritten with an empty file, and the original content is lost. If you wish to only open a file for writing if it doesn't already exist, you can use the x flag instead, as in the following example:

```python

# files/write_not_exists.py
with open('write_x.txt', 'x') as fw:  # this succeeds
    fw.write('Writing line 1')

with open('write_x.txt', 'x') as fw:  # this fails
    fw.write('Writing line 2')
```

If you run this snippet, you will find a file called write_x.txt in your directory, containing only one line of text. The second part of the snippet, in fact, fails to execute. This is the output we get on our console (the file path has been shortened for editorial purposes):

```bash
$ python write_not_exists.py 
Traceback (most recent call last):
  File "/…/ch08/files/write_not_exists.py", line 6, in <module>
    with open('write_x.txt', 'x') as fw:
FileExistsError: [Errno 17] File exists: 'write_x.txt'
```


### Checking for file and directory existence

If you want to make sure a file or directory exists (or doesn't), the pathlib module is what you need. Let's see a small example:

```python
# files/existence.py
from pathlib import Path

p = Path('fear.txt')
path = p.parent.absolute()

print(p.is_file())        # True
print(path)               # /Users/fab/srv/lpp3e/ch08/files
print(path.is_dir())      # True

q = Path('/Users/fab/srv/lpp3e/ch08/files')
print(q.is_dir())         # True
```

The preceding snippet is quite interesting. We create a Path object that we set up with the name of the text file we want to inspect. We use the parent() method to retrieve the folder in which the file is contained, and we call the absolute() method on it, to extract the absolute path information.

We check if 'fear.txt' is a file, and the folder in which it is contained is indeed a folder (or directory, which is equivalent).

The old way to do these operations was to use the os.path module from the standard library. While os.path works on strings, pathlib offers classes representing filesystem paths with semantics appropriate for different operating systems. Hence, we suggest using pathlib whenever possible, and reverting to the old way of doing things only when there is no alternative.


### Manipulating files and directories

Let's see a couple of quick examples on how to manipulate files and directories. The first example manipulates the content:

```python

# files/manipulation.py
from collections import Counter
from string import ascii_letters

chars = ascii_letters + ' '

def sanitize(s, chars):
    return ''.join(c for c in s if c in chars)

def reverse(s):
    return s[::-1]

with open('fear.txt') as stream:
    lines = [line.rstrip() for line in stream]

# let's write the mirrored version of the file
with open('raef.txt', 'w') as stream:
    stream.write('\n'.join(reverse(line) for line in lines))

# now we can calculate some statistics
lines = [sanitize(line, chars) for line in lines]
whole = ' '.join(lines)

# we perform comparisons on the lowercased version of 'whole'
cnt = Counter(whole.lower().split())

# we can print the N most common words
print(cnt.most_common(3))
```

This example defines two functions: sanitize() and reverse(). They are simple functions whose purpose is to remove anything that is not a letter or space from a string, and produce the reversed copy of a string, respectively.

We open fear.txt and we read its content into a list. Then we create a new file, raef. txt, which will contain the horizontally-mirrored version of the original one. We write all the content of lines with a single operation, using join on a newline character. Maybe more interesting is the bit at the end. First, we reassign lines to a sanitized version of itself by means of a list comprehension. Then we put the lines together in the whole string, and finally, we pass the result to a Counter object. Notice that we split the lowercased version of the string into a list of words. This way, each word will be counted correctly, regardless of its case, and, thanks to split(), we don't need to worry about extra spaces anywhere. When we print the three most common words, we realize that, truly, Thich Nhat Hanh's focus is on others, as we is the most common word in the text:

```python
$ python manipulation.py
[('we', 17), ('the', 13), ('were', 7)]
```

Let's now see an example of manipulation more oriented to disk operations, in which we put the shutil module to use:

```python
# files/ops_create.py
import shutil
from pathlib import Path

base_path = Path('ops_example')

# let's perform an initial cleanup just in case
if base_path.exists() and base_path.is_dir():
    shutil.rmtree(base_path)

# now we create the directory
base_path.mkdir()

path_b = base_path / 'A' / 'B'
path_c = base_path / 'A' / 'C'
path_d = base_path / 'A' / 'D'

path_b.mkdir(parents=True)
path_c.mkdir()  # no need for parents now, as 'A' has been created

# we add three files in 'ops_example/A/B'
for filename in ('ex1.txt', 'ex2.txt', 'ex3.txt'):
    with open(path_b / filename, 'w') as stream:
        stream.write(f'Some content here in {filename}\n')

shutil.move(path_b, path_d)

# we can also rename files
ex1 = path_d / 'ex1.txt'
ex1.rename(ex1.parent / 'ex1.renamed.txt')
```

In the preceding code, we start by declaring a base path, which will safely contain all the files and folders we're going to create. We then use mkdir() to create two directories: ops_example/A/B and ops_example/A/C. Notice we don't need to specify parents=True when calling path_c.mkdir(), since all the parents have already been created by the previous call on path_b.

We use the / operator to concatenate directory names; pathlib takes care of using the right path separator for us, behind the scenes.

After creating the directories, we use a simple for loop to create three files in directory B. Then, we move directory B and its contents to a different name: D. And finally, we rename ex1.txt to ex1.renamed.txt. If you open that file, you'll see it still contains the original text from the for loop logic. Calling tree on the result produces the following:


```bash

$ tree ops_example/
ops_example/
└── A
    ├── C
    └── D
        ├── ex1.renamed.txt
        ├── ex2.txt
        └── ex3.txt

```
### Manipulating pathnames

Let's explore the abilities of pathlib a little more by means of a simple example:

```python

# files/paths.py
from pathlib import Path

p = Path('fear.txt')

print(p.absolute())
print(p.name)
print(p.parent.absolute())
print(p.suffix)

print(p.parts)
print(p.absolute().parts)

readme_path = p.parent / '..' / '..' / 'README.rst'
print(readme_path.absolute())
print(readme_path.resolve())
```

Reading the result is probably a good enough explanation for this simple example:

```bash
/Users/fab/srv/lpp3e/ch08/files/fear.txt
fear.txt
/Users/fab/srv/lpp3e/ch08/files
.txt
('fear.txt',)
('/', 'Users', 'fab', 'srv', 'lpp3e', 'ch08', 'files', 'fear.txt')
/Users/fab/srv/lpp3e/ch08/files/../../README.rst
/Users/fab/srv/lpp3e/README.rst
```

Note how, in the last two lines, we have two different representations of the same path. The first one (readme_path.absolute()) shows two '..', a single one of which, in path terms, indicates changing to the parent folder. So, by changing to the parent folder twice in a row, from …/lpp3e/ch08/files/ we go back to …/lpp3e/. This is confirmed by the last line in the example, which shows the output of readme_path. resolve().


### Temporary files and directories

Sometimes, it's very useful to be able to create a temporary directory or file when running some code. For example, when writing tests that affect the disk, you can use temporary files and directories to run your logic and assert that it's correct, and to be sure that at the end of the test run, the test folder has no leftovers. Let's see how to do it in Python:

```python
# files/tmp.py
from tempfile import NamedTemporaryFile, TemporaryDirectory

with TemporaryDirectory(dir='.') as td:
    print('Temp directory:', td)
    with NamedTemporaryFile(dir=td) as t:
        name = t.name
        print(name)
```

The preceding example is quite straightforward: we create a temporary directory in the current one ("."), and we create a named temporary file in it. We print the filename, as well as its full path:

```bash
$ python tmp.py 
Temp directory: ./tmpz5i9ne20
/Users/fab/srv/lpp3e/ch08/files/tmpz5i9ne20/tmp2e3j8p78
```

Running this script will produce a different result every time. After all, it's a temporary random name we're creating here, right?


### Directory content

With Python, you can also inspect the contents of a directory. We will show you two ways of doing this. This is the first:

```python
# files/listing.py
from pathlib import Path

p = Path('.')
for entry in p.glob('*'):
    print('File:' if entry.is_file() else 'Folder:', entry)
```

This snippet uses the glob() method of a Path object, applied from the current directory. We iterate on the results, each of which is an instance of a subclass of Path (PosixPath or WindowsPath, according to which OS we are running). For each entry, we inspect if it is a directory, and print accordingly. Running the code yields the following (we omitted a few results for brevity):

```bash
$ python listing.py 
File: existence.py
File: fear.txt
…
Folder: compression
…
File: walking.pathlib.py
…
```

An alternative way to scan a directory tree is given to us by os.walk. Let's see an example:

```python

# files/walking.py
import os

for root, dirs, files in os.walk('.'):
    abs_root = os.path.abspath(root)
    print(abs_root)

    if dirs:
        print('Directories:')
        for dir_ in dirs:
            print(dir_)
        print()

    if files:
        print('Files:')
        for filename in files:
            print(filename)
        print()
```

Running the preceding snippet will produce a list of all the files and directories in the current one, and it will do the same for each sub-directory.

### File and directory compression

Before we leave this section, let us give you an example of how to create a compressed file. In the source code of the book, we have two examples: one creates a .zip file, while the other one creates a tar.gz file. Python allows you to create compressed files in several different ways and formats. Here, we are going to show you how to create the most common one, ZIP:
```python
# files/compression/zip.py
from zipfile import ZipFile

with ZipFile('example.zip', 'w') as zp:
    zp.write('content1.txt')
    zp.write('content2.txt')
    zp.write('subfolder/content3.txt')
    zp.write('subfolder/content4.txt')

with ZipFile('example.zip') as zp:
    zp.extract('content1.txt', 'extract_zip')
    zp.extract('subfolder/content3.txt', 'extract_zip')
```

In the preceding code, we import ZipFile, and then, within a context manager, we write into it four files (two of which are in a sub-folder, to show how ZIP preserves the full path). Afterward, as an example, we open the compressed file and extract a couple of files from it into the extract_zip directory. If you are interested in learning more about data compression, make sure you check out the Data Compression and Archiving section on the standard library (https://docs.python.org/3.9/library/archiving.html), where you'll be able to learn all about this topic.


## Data interchange formats

Modern software architecture tends to split an application into several components. Whether you embrace the service-oriented architecture paradigm, or you push it even further into the microservices realm, these components will have to exchange data. But even if you are coding a monolithic application whose codebase is contained in one project, chances are that you still have to exchange data with APIs, other programs, or simply handle the data flow between the frontend and backend parts of your website, which very likely won't speak the same language.

Choosing the right format in which to exchange information is crucial. A language- specific format has the advantage that the language itself is very likely to provide you with all the tools to make serialization and deserialization a breeze. However, you will lose the ability to talk to other components that have been written in different versions of the same language, or in different languages altogether. Regardless of what the future looks like, going with a language-specific format should only be done if it is the only possible choice for the given situation.

According to Wikipedia (https://en.wikipedia.org/wiki/Serialization):

    In computing, serialization is the process of translating a data structure or object state into a format that can be stored (for example, in a file or memory data buffer) or transmitted (for example, over a computer network) and reconstructed later (possibly in a different computer environment).

A much better approach is to choose a format that is language-agnostic, and can be spoken by all (or at least most) languages. Fabrizio used to lead a team of programmers from England, Poland, South Africa, Spain, Greece, India, and Italy, to mention just a few. They all spoke English, so regardless of their native tongue, they could all understand each other (well... mostly!).

In the software world, some popular formats have become the de facto standard for data interchange. The most famous ones probably are XML, YAML, and JSON. The Python standard library features the xml and json modules, and, on PyPI (https://pypi.org/), you can find a few different packages to work with YAML.

In the Python environment, JSON is perhaps the most commonly used one. It wins over the other two because of being part of the standard library, and for its simplicity. If you have ever worked with XML, you know what a nightmare it can be.

Moreover, when working with a database like PostgreSQL, the ability to use native JSON fields makes a compelling case for adopting JSON in the application as well.

### Working with JSON

JSON is the acronym for JavaScript Object Notation, and it is a subset of the JavaScript language. It has been around for almost two decades now, so it is well known and widely adopted by most languages, even though it is actually language- independent. You can read all about it on its website (https://www.json.org/), but we are going to give you a quick introduction to it now.

JSON is based on two structures: a collection of name/value pairs, and an ordered list of values. It's quite straightforward to realize that these two objects map to the dict and list data types in Python, respectively. As data types, JSON offers strings, numbers, objects, and values consisting of true, false, and null. Let's see a quick example to get us started:
```python
# json_examples/json_basic.py
import sys
import json

data = {
    'big_number': 2 ** 3141,
    'max_float': sys.float_info.max,
    'a_list': [2, 3, 5, 7],
}

json_data = json.dumps(data)
data_out = json.loads(json_data)
assert data == data_out  # json and back, data matches
```

We begin by importing the sys and json modules. Then we create a simple dictionary with some numbers inside and a list. We wanted to test serializing and deserializing using very big numbers, both int and float, so we put 23141 and whatever is the biggest floating point number our system can handle.

We serialize with json.dumps(), which takes data and converts it into a JSON formatted string. That data is then fed into json.loads(), which does the opposite: from a JSON formatted string, it reconstructs the data into Python. On the last line, we make sure that the original data and the result of the serialization/deserialization through JSON match.

Let's see what JSON data would look like if we printed it:
```python
# json_examples/json_basic.py
import json

info = {
    'full_name': 'Sherlock Holmes',
    'address': {
        'street': '221B Baker St',
        'zip': 'NW1 6XE',
        'city': 'London',
        'country': 'UK',
    }
}

print(json.dumps(info, indent=2, sort_keys=True))
```

In this example, we create a dictionary with Sherlock Holmes' data in it. If, like us, you are a fan of Sherlock Holmes, and are in London, you will find his museum at that address (which we recommend visiting; it's small but very nice).

Notice how we call json.dumps, though. We have told it to indent with two spaces, and sort keys alphabetically. The result is this:
```python
$ python json_basic.py
{
  "address": {
    "city": "London",
    "country": "UK",
    "street": "221B Baker St",
    "zip": "NW1 6XE"
  },
  "full_name": "Sherlock Holmes"
}
```

The similarity with Python is evident. The one difference is that if you place a comma on the last element in a dictionary, as is customary in Python, JSON will complain.

Let us show you something interesting:
```python
# json_examples/json_tuple.py
import json

data_in = {
    'a_tuple': (1, 2, 3, 4, 5),
}

json_data = json.dumps(data_in)
print(json_data)  # {"a_tuple": [1, 2, 3, 4, 5]}
data_out = json.loads(json_data)
print(data_out)  # {'a_tuple': [1, 2, 3, 4, 5]}
```

In this example, we have used a tuple instead of a list. The interesting bit is that, conceptually, a tuple is also an ordered list of items. It doesn't have the flexibility of a list, but still, it is considered the same from the perspective of JSON. Therefore, as you can see by the first print, in JSON a tuple is transformed into a list. Naturally then, the information that the original object was a tuple is lost, and when deserialization happens, a_tuple is actually translated to a Python list. It is important that you keep this in mind when dealing with data, as going through a transformation process that involves a format that only comprises a subset of the data structures you can use implies there may be information loss. In this case, we lost the information about the type (tuple versus list).

This is actually a common problem. For example, you can't serialize all Python objects to JSON, as it is not always clear how JSON should revert that object. Think about datetime, for example. An instance of that class is a Python object that JSON won't be able to serialize. If we transform it into a string such as 2018-03-04T12:00:30Z, which is the ISO 8601 representation of a date with time and time zone information, what should JSON do when deserializing? Should it decide that this is actually deserializable into a datetime object, so I'd better do it, or should it simply consider it as a string and leave it as it is? What about data types that can be interpreted in more than one way?

The answer is that when dealing with data interchange, we often need to transform our objects into a simpler format prior to serializing them with JSON. The more we manage to simplify our data, the easier it is to represent that data in a format like JSON, which has limitations.

In some cases, though, and mostly for internal use, it is useful to be able to serialize custom objects, so, just for fun, we are going to show you how with two examples: complex numbers (because we love math) and datetime objects.

### Custom encoding/decoding with JSON

In the JSON world, we can consider terms like encoding/decoding as synonyms for serializing/deserializing. They basically mean transforming to and back from JSON. 

In the following example, we are going to learn how to encode complex numbers – which aren't serializable to JSON by default – by writing a custom encoder:
```python

# json_examples/json_cplx.py
import json

class ComplexEncoder(json.JSONEncoder):
    def default(self, obj):

        print(f"ComplexEncoder.default: {obj=}")
        if isinstance(obj, complex):
            return {
                '_meta': '_complex',
                'num': [obj.real, obj.imag],
            }
        return super().default(obj)

data = {
    'an_int': 42,
    'a_float': 3.14159265,
    'a_complex': 3 + 4j,
}

json_data = json.dumps(data, cls=ComplexEncoder)
print(json_data)

def object_hook(obj):
    print(f"object_hook: {obj=}")
    try:
        if obj['_meta'] == '_complex':
            return complex(*obj['num'])
    except KeyError:
        return obj

data_out = json.loads(json_data, object_hook=object_hook)
print(data_out)
```

We start by defining a ComplexEncoder class as a subclass of the JSONEncoder class. Our class overrides the default method. This method is called whenever the encoder encounters an object that it cannot encode and is expected to return an encodable representation of that object. 

Our default() method checks whether its argument is a complex object, in which case it returns a dictionary with some custom meta information, and a list that contains both the real and the imaginary part of the number. That is all we need to do to avoid losing information for a complex number. If we receive anything other than an instance of complex, we call the default() method from the parent class, which just raises a TypeError. We then call json.dumps(), but this time we use the cls argument to specify our custom encoder. The result is printed:

```python
$ python json_cplx.py 
ComplexEncoder.default: obj=(3+4j) 
{"an_int": 42, "a_float": 3.14159265,
 "a_complex": {"_meta": "_complex", "num": [3.0, 4.0]}}
```

Half the job is done. For the deserialization part, we could have written another class that would inherit from JSONDecoder, but instead we have chosen to use a different technique that is simpler and uses a small function: object_hook.

Within the body of object_hook(), we find a try block. The important part is the two lines within the body of the try block itself. The function receives an object (notice that the function is only called when obj is a dictionary), and if the metadata matches our convention for complex numbers, we pass the real and imaginary parts to the complex() function. The try/except block is there because our function will be called for every dictionary object that is decoded, so we need to handle the case where our _meta key is not present.

The decoding part of the example outputs:

```python
object_hook: obj={'_meta': '_complex', 'num': [3.0, 4.0]}
object_hook: obj={'an_int': 42, 'a_float': 3.14159265, 'a_complex': 
(3+4j)} 
{'an_int': 42, 'a_float': 3.14159265, 'a_complex': (3+4j)}
```

You can see that a_complex has been correctly deserialized.

Let's now consider a slightly more complex (no pun intended) example: dealing with datetime objects. We are going to split the code into two blocks, first the serializing part, and then the deserializing one:

```python# json_examples/json_datetime.py 
import json
from datetime import datetime, timedelta, timezone

now = datetime.now()
now_tz = datetime.now(tz=timezone(timedelta(hours=1)))

class DatetimeEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, datetime):
            try:
                off = obj.utcoffset().seconds
            except AttributeError:
                off = None

            return {
                '_meta': '_datetime',
                'data': obj.timetuple()[:6] + (obj.microsecond, ),
                'utcoffset': off,
            }
        return super().default(obj)

data = {
    'an_int': 42,
    'a_float': 3.14159265,
    'a_datetime': now,
    'a_datetime_tz': now_tz,
}
json_data = json.dumps(data, cls=DatetimeEncoder)
print(json_data)
```

The reason why this example is slightly more complex lies in the fact that datetime objects in Python can be time zone-aware or not; therefore, we need to be more careful. The flow is the same as before, only we are dealing with a different data type. We start by getting the current date and time information, and we do it both without (now) and with (now_tz) time zone awareness, just to make sure our script works. We then proceed to define a custom encoder as before, overriding the default() method. The important bits in that method are how we get the time zone offset (off) information, in seconds, and how we structure the dictionary that returns the data. This time, the metadata says it's datetime information. We save the first six items in the time tuple (year, month, day, hour, minute, and second), plus the microseconds in the data key, and the offset after that. Could you tell that the value of 'data' is a concatenation of tuples? Good job if you could!

When we have our custom encoder, we proceed to create some data, and then we serialize. The print statement outputs the following (we have reformatted the output to make it more readable):

```python
{
    "an_int": 42,
    "a_float": 3.14159265,
    "a_datetime": {
        "_meta": "_datetime",
        "data": [2021, 5, 17, 23, 1, 58, 75097],
        "utcoffset": null
    },
    "a_datetime_tz": {
        "_meta": "_datetime",
        "data": [2021, 5, 17, 23, 1, 58, 75112],
        "utcoffset": 3600
    }
}
```

Interestingly, we find out that None is translated to null, its JavaScript equivalent. Moreover, we can see that our data seems to have been encoded properly. Let's proceed with the second part of the script:

```python
# json_examples/json_datetime.py
def object_hook(obj):
    try:
        if obj['_meta'] == '_datetime':
            if obj['utcoffset'] is None:
                tz = None
            else:
                tz = timezone(timedelta(seconds=obj['utcoffset']))
            return datetime(*obj['data'], tzinfo=tz)
    except KeyError:
        return obj

data_out = json.loads(json_data, object_hook=object_hook)
```

Once again, we first verify that the metadata is telling us it's a datetime, and then we proceed to fetch the time zone information. Once we have that, we pass the 7-tuple (using * to unpack its values in the call) and the time zone information to the datetime() call, getting back our original object. Let's verify it by printing data_out:

```python
{
    'a_datetime': datetime.datetime(
        2021, 5, 17, 23, 10, 2, 830913
    ),
    'a_datetime_tz': datetime.datetime(
        2021, 5, 17, 23, 10, 2, 830927,
        tzinfo=datetime.timezone(datetime.timedelta(seconds=3600))

    ),
    'a_float': 3.14159265,
    'an_int': 42
}
```

As you can see, we got everything back correctly. As an exercise, we would like to challenge you to write the same logic but for a date object, which should be simpler.

Before we move on to the next topic, a word of caution. Perhaps it is counter- intuitive, but working with datetime objects can be one of the trickiest things to do, so although we are pretty sure this code is doing what it is supposed to do, we want to stress that we only tested it lightly. So, if you intend to grab it and use it, please do test it thoroughly. Test for different time zones, test for daylight saving time being on and off, test for dates before the epoch, and so on. You might find that the code in this section needs some modifications to suit your cases.

## I/O, streams, and requests

I/O stands for input/output, and it broadly refers to the communication between a computer and the outside world. There are several different types of I/O, and it is outside the scope of this chapter to explain all of them, but it's worth going through a couple of examples. The first one will introduce the io.StringIO class, which is an in-memory stream for text I/O. The second one instead will escape the locality of our computer, and demonstrate how to perform an HTTP request.

### Using an in-memory stream

In-memory objects can be useful in a multitude of situations. Memory is much faster than a disk, it's always available, and for small amounts of data can be the perfect choice.

Let's see the first example:

```python
# io_examples/string_io.py
import io

stream = io.StringIO()
stream.write('Learning Python Programming.\n')
print('Become a Python ninja!', file=stream)

contents = stream.getvalue()
print(contents)

stream.close()
```

In the preceding code snippet, we import the io module from the standard library. This is a very interesting module that features many tools related to streams and I/O. One of them is StringIO, which is an in-memory buffer in which we're going to write two sentences, using two different methods, as we did with files in the first examples of this chapter. We can either call StringIO.write() or we can use print, telling it to direct the data to our stream.

By calling getvalue(), we can get the content of the stream. We then proceed to print it, and finally we close it. The call to close() causes the text buffer to be immediately discarded.

There is a more elegant way to write the previous code:

```python
# io_examples/string_io.py
with io.StringIO() as stream:
    stream.write('Learning Python Programming.\n')
    print('Become a Python ninja!', file=stream)
    contents = stream.getvalue()
    print(contents)
```

Yes, it is again a context manager. Like the built-in open(), io.StringIO() works well within a context manager block. Notice the similarity with open: in this case too, we don't need to manually close the stream.

When running the script, the output is:

```python
$ python string_io.py
Learning Python Programming.
Become a Python ninja!
```
Let's now proceed with the second example.

### Making HTTP requests

In this section, we explore two examples on HTTP requests. We will use the requests library for these examples, which you can install with pip, and it is included in the requirements file for this chapter.

We're going to perform HTTP requests against the httpbin.org (http://httpbin.org/) API, which, interestingly, was developed by Kenneth Reitz, the creator of the requests library itself. 

This library is among the most widely adopted all over the world:
```python

# io_examples/reqs.py
import requests

urls = {
    "get": "https://httpbin.org/get?t=learn+python+programming",
    "headers": "https://httpbin.org/headers",
    "ip": "https://httpbin.org/ip",
    "user-agent": "https://httpbin.org/user-agent",
    "UUID": "https://httpbin.org/uuid",
    "JSON": "https://httpbin.org/json",
}

def get_content(title, url):
    resp = requests.get(url)
    print(f"Response for {title}")
    print(resp.json())

for title, url in urls.items():
    get_content(title, url)
    print("-" * 40)

```

The preceding snippet should be simple to understand. We declare a dictionary of URLs against which we want to perform HTTP requests. We have encapsulated the code that performs the request into a tiny function, get_content(). As you can see, we perform a GET request (by using requests.get()), and we print the title and the JSON decoded version of the body of the response. Let us spend a few words on this last bit.

When we perform a request to a website, or to an API, we get back a response object, which is, very simply, what was returned by the server we performed the request against. The body of some responses from httpbin.org happens to be JSON encoded, so instead of getting the body as it is (by using resp.text) and manually decoding it calling json.loads() on it, we simply combine the two by leveraging the json() method on the response object. There are plenty of reasons why the requests package has become so widely adopted, and one of them is definitely its ease of use.

Now, when you perform a request in your application, you will want to have a much more robust approach in dealing with errors and so on, but for this chapter, a simple example will do. We will see more examples of requests in Chapter 14, Introduction to API Development.

Going back to our code, in the end, we run a for loop and get all the URLs. When you run it, you will see the result of each call printed on your console, which should look like this (prettified and trimmed for brevity):
```python

$ python reqs.py
Response for get
{
    "args": {"t": "learn python programming"},
    "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip, deflate",
        "Host": "httpbin.org",
        "User-Agent": "python-requests/2.25.1",
        "X-Amzn-Trace-Id": "Root=1-60a42902-3b6093e26ae375244478",
    },
    "origin": "86.8.174.15",
    "url": "https://httpbin.org/get?t=learn+python+programming",
}
... rest of the output omitted ...

```

Notice that you might get a slightly different output in terms of version numbers and IPs, which is fine. Now, GET is only one of the HTTP verbs, albeit one of the most commonly used. Let us also look at how to use the POST verb. This is the type of request you make when you need to send data to the server. Every time you submit a form on the web, you're making a POST request. So, let's try to make one programmatically:

```python
# io_examples/reqs_post.py
import requests

url = 'https://httpbin.org/post'
data = dict(title='Learn Python Programming')

resp = requests.post(url, data=data)
print('Response for POST')
print(resp.json())
```

The preceding code is very similar to what we saw before, only this time we don't call get(), but post(), and because we want to send some data, we specify that in the call. The requests library offers much more than this. It is a project that we encourage you to check out and explore, as it's quite likely you will be using it too.

Running the previous script (and applying some prettifying magic to the output) yields the following:
```python

$ python reqs_post.py
Response for POST
{
    "args": {},
    "data": "",
    "files": {},
    "form": {"title": "Learn Python Programming"},
    "headers": {
        "Accept": "*/*",
        "Accept-Encoding": "gzip, deflate",
        "Content-Length": "30",
        "Content-Type": "application/x-www-form-urlencoded",
        "Host": "httpbin.org",
        "User-Agent": "python-requests/2.25.1",
        "X-Amzn-Trace-Id": "Root=1-60a43131-5032cdbc14db751fe775",
    },
    "json": None,
    "origin": "86.8.174.15",
    "url": "https://httpbin.org/post",
}
```

Notice how the headers are now different, and we find the data we sent in the form key/value pair of the response body.

We hope these short examples are enough to get you started, especially with requests. The web changes every day, so it's worth learning the basics and then brushing up every now and then.

## Persisting data on disk

In this last section of this chapter, we'll look at how to persist data on disk in three different formats. To persist data means that the data is written to non-volatile storage, like a hard drive, for example, and it is not deleted when the process that wrote it ends its life cycle. We will explore pickle and shelve, as well as a short example that will involve accessing a database using SQLAlchemy, perhaps the most widely adopted ORM library in the Python ecosystem.

### Serializing data with pickle

The pickle module, from the Python standard library, offers tools to convert Python objects into byte streams, and vice versa. Even though there is a partial overlap in the API that pickle and json expose, the two are quite different. As we have seen previously in this chapter, JSON is a text format that is human readable, language independent, and supports only a restricted subset of Python data types. The pickle module, on the other hand, is not human readable, translates to bytes, is Python-specific, and, thanks to the wonderful Python introspection capabilities, supports a large number of data types.

Besides the above-mentioned differences between pickle and json, there are also some important security concerns that you need to be aware of if you are considering using pickle. Unpickling erroneous or malicious data from an untrusted source can be dangerous, so if we decide to adopt it in our application, we need to be extra careful.

If you do use pickle, you should consider using a cryptographic signature to ensure that your pickled data has not been tampered with. We will see how to generate cryptographic signatures in Python in Chapter 9, Cryptography and Tokens.

That said, let's see it in action by means of a simple example:

```python
# persistence/pickler.py
import pickle
from dataclasses import dataclass

@dataclass
class Person:
    first_name: str
    last_name: str
    id: int

    def greet(self):
        print(f'Hi, I am {self.first_name} {self.last_name}'
              f' and my ID is {self.id}')

people = [
    Person('Obi-Wan', 'Kenobi', 123),
    Person('Anakin', 'Skywalker', 456),
]

# save data in binary format to a file
with open('data.pickle', 'wb') as stream:
    pickle.dump(people, stream)

# load data from a file
with open('data.pickle', 'rb') as stream:
    peeps = pickle.load(stream)

for person in peeps:
    person.greet()

```

In this example, we create a Person class using the dataclass decorator, which we saw in Chapter 6, OOP, Decorators, and Iterators. The only reason we wrote this example with a dataclass is to show you how effortlessly pickle deals with it, with no need for us to do anything we wouldn't do for a simpler data type.

The class has three attributes: first_name, last_name, and id. It also exposes a greet() method, which simply prints a hello message with the data.

We create a list of instances and save it to a file. In order to do so, we use pickle. dump(), to which we feed the content to be pickled, and the stream to which we want to write. Immediately after that, we read from that same file, using pickle.load() to convert the entire content of the stream back into Python objects. To make sure that the objects have been converted correctly, we call the greet() method on both of them. The result is the following:
```python

$ python pickler.py 
Hi, I am Obi-Wan Kenobi and my ID is 123
Hi, I am Anakin Skywalker and my ID is 456
```

The pickle module also allows you to convert to (and from) byte objects, by means of the dumps() and loads() functions (note the s at the end of both names). In day-to- day applications, pickle is usually used when we need to persist Python data that is not supposed to be exchanged with another application. One example we stumbled upon, a few years ago, was the session manager in a flask plugin, which pickles the session object before storing it in a Redis database. In practice, though, you are unlikely to have to deal with this library very often.

Another tool that is possibly used even less, but that proves to be very useful when 
you are short on resources, is shelve.

### Saving data with shelve

A shelf is a persistent dictionary-like object. The beauty of it is that the values you save into a shelf can be any objects you can pickle, so you're not restricted like you would be if you were using a database. Albeit interesting and useful, the shelve module is used quite rarely in practice. Just for completeness, let's see a quick example of how it works:
```python

# persistence/shelf.py
import shelve

class Person:
    def __init__(self, name, id):
        self.name = name
        self.id = id

with shelve.open('shelf1.shelve') as db:
    db['obi1'] = Person('Obi-Wan', 123)
    db['ani'] = Person('Anakin', 456)
    db['a_list'] = [2, 3, 5]
    db['delete_me'] = 'we will have to delete this one...'
    print(list(db.keys()))  # 'ani', 'delete_me', 'a_list', 'obi1']

    del db['delete_me']  # gone!
    print(list(db.keys()))  # ['ani', 'a_list', 'obi1']
    print('delete_me' in db)  # False
    print('ani' in db)  # True

    a_list = db['a_list']
    a_list.append(7)
    db['a_list'] = a_list
    print(db['a_list'])  # [2, 3, 5, 7]
```

Apart from the wiring and the boilerplate around it, this example resembles an exercise with dictionaries. We create a simple Person class and then we open a shelve file within a context manager. As you can see, we use the dictionary syntax to store four objects: two Person instances, a list, and a string. If we print the keys, we get a list containing the four keys we used. Immediately after printing it, we delete the (aptly named) delete_me key/value pair from the shelf. Printing the keys again shows the deletion has succeeded. We then test a couple of keys for membership and, finally, we append number 7 to a_list. Notice how we have to extract the list from the shelf, modify it, and save it again.

If this behavior is undesired, there is something we can do:
```python

# persistence/shelf.py
with shelve.open('shelf2.shelve', writeback=True) as db:
    db['a_list'] = [11, 13, 17]
    db['a_list'].append(19)  # in-place append!
    print(db['a_list'])  # [11, 13, 17, 19]
```

By opening the shelf with writeback=True, we enable the writeback feature, which allows us to simply append to a_list as if it actually was a value within a regular dictionary. The reason why this feature is not active by default is that it comes with a price that you pay in terms of memory consumption and slower closing of the shelf.

Now that we have paid homage to the standard library modules related to data persistence, let's take a look at one of the most widely adopted ORMs in the Python ecosystem: SQLAlchemy.

### Saving data to a database

For this example, we are going to work with an in-memory database, which will make things simpler for us. In the source code of the book, we have left a couple of comments to show you how to generate a SQLite file, so we hope you'll explore that 
option as well.

You can find a free database browser for SQLite at https:// dbeaver.io. DBeaver is a free multi-platform database tool for developers, database administrators, analysts, and all people who need to work with databases. It supports all popular databases: MySQL, PostgreSQL, SQLite, Oracle, DB2, SQL Server, Sybase, MS Access, Teradata, Firebird, Apache Hive, Phoenix, Presto, and so on.

Before we dive into the code, allow us to briefly introduce the concept of a relational database.

A relational database is a database that allows you to save data following the relational model, invented in 1969 by Edgar F. Codd. In this model, data is stored in one or more tables. Each table has rows (also known as records, or tuples), each of which represents an entry in the table. Tables also have columns (also known as attributes), each of which represents an attribute of the records. Each record is identified through a unique key, more commonly known as the primary key, which is the union of one or more columns in the table. To give you an example: imagine a table called Users, with columns id, username, password, name, and surname. 

Such a table would be perfect to contain users of our system; each row would represent a different user. For example, a row with the values 3, fab, my_wonderful_ pwd, Fabrizio, and Romano would represent Fabrizio's user in the system. The reason why the model is called relational is because you can establish relations between tables. For example, if you added a table called PhoneNumbers to our fictitious database, you could insert phone numbers into it, and then, through a relation, establish which phone number belongs to which user. In order to query a relational database, we need a special language. The main standard is called SQL, which stands for Structured Query Language. It is born out of something called relational algebra, which is a family of algebras used to model data stored according to the relational model and perform queries on it. The most common operations you can perform usually involve filtering on the rows or columns, joining tables, aggregating the results according to some criteria, and so on. To give you an example in English, a query on our imaginary database could be: Fetch all users (username, name, surname) whose username starts with "m", who have at most one phone number. In this query, we are asking for a subset of the columns in the User table. We are filtering on users by taking only those whose username starts with the letter m, and even further, only those who have at most one phone number. Back in the days when Fabrizio was a student in Padova, he spent a whole semester learning both the relational algebra semantics and 
standard SQL (among other things). If it wasn't for a major bicycle accident he had the day of the exam, he would say that this was one of the most fun exams he ever had to take!

Now, each database comes with its own flavor of SQL. They all respect the standard to some extent, but none fully do, and they are all different from one another in some respects. This poses an issue in modern software development. If our application contains SQL code, it is quite likely that if we decided to use a different database engine, or maybe a different version of the same engine, we would find our SQL code needs amending. This can be quite painful, especially since SQL queries can become very complicated quite quickly. In order to alleviate this pain a little, computer scientists have created code that maps objects of a programming language to tables of a relational database. Unsurprisingly, the name of such a tool is Object-Relational Mapping (ORM). In modern application development, you would normally start interacting with a database by using an ORM, and should you find yourself in a situation where you can't perform a query you need to perform through the ORM, you would then resort to using SQL directly. This is a good compromise between having no SQL at all, and using no ORM, which ultimately means specializing the code that interacts with the database, with the aforementioned disadvantages.

In this section, we would like to show an example that leverages SQLAlchemy, one of the most popular third-party Python ORMs. You will have to pip install it into the virtual environment for this chapter. We are going to define two models (Person and Address), each of which maps to a table, and then we're going to populate the database and perform a few queries on it.

Let's start with the model declarations:

```python
# persistence/alchemy_models.py
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import (
    Column, Integer, String, ForeignKey, create_engine)
from sqlalchemy.orm import relationship
```

At the beginning, we import some functions and types. The first thing we need to do then is to create an engine. This engine tells SQLAlchemy about the type of database we have chosen for our example, and how to connect to it:
```python

# persistence/alchemy_models.py
engine = create_engine('sqlite:///:memory:')
Base = declarative_base()

class Person(Base):
    __tablename__ = 'person'

    id = Column(Integer, primary_key=True)
    name = Column(String)
    age = Column(Integer)

    addresses = relationship(
        'Address',
        back_populates='person',
        order_by='Address.email',
        cascade='all, delete-orphan'
    )

    def __repr__(self):
        return f'{self.name}(id={self.id})'

class Address(Base):
    __tablename__ = 'address'
    id = Column(Integer, primary_key=True)
    email = Column(String)
    person_id = Column(ForeignKey('person.id'))
    person = relationship('Person', back_populates='addresses')

    def __str__(self):
        return self.email
    __repr__ = __str__

Base.metadata.create_all(engine)
```

Each model then inherits from the Base table, which in this example simply consists of the default, returned by declarative_base(). We define Person, which maps to a table called person, and exposes the attributes id, name, and age. We also declare a relationship with the Address model, by stating that accessing the addresses attribute will fetch all the entries in the address table that are related to the particular Person instance we're dealing with. The cascade option affects how creation and deletion work, but it is a more advanced concept, so we suggest you ignore it for now and maybe investigate more later on.

The last thing we declare is the __repr__() method, which provides us with the official string representation of an object. This is supposed to be a representation that can be used to completely reconstruct the object, but in this example, we simply use it to provide something in output. Python redirects repr(obj) to a call to obj.__repr__().

We also declare the Address model, which will contain email addresses, and a reference to the person they belong to. You can see the person_id and person attributes are both about setting a relation between the Address and Person instances. Note also how we declare the __str__() method on Address, and then assign an alias to it, called __repr__(). This means that calling either repr() or str() on Address objects will ultimately result in calling the  __str__() method. This is quite a common technique in Python, used to avoid duplicating the same code, so we took the opportunity to show it to you here.

On the last line, we tell the engine to create tables in the database according to our models.

The create_engine() function supports a parameter called echo, which can be set to True, False, or the string "debug", to enable different levels of logging of all statements and the repr() of their parameters. Please refer to the official SQLAlchemy documentation for further information.

A deeper understanding of this code would require more space than we can afford, so we encourage you to read up on database management systems (DBMS), SQL, relational algebra, and SQLAlchemy.

Now that we have our models, let's use them to persist some data!

Take a look at the following example:
```python
# persistence/alchemy.py
from alchemy_models import Person, Address, engine
from sqlalchemy.orm import sessionmaker

Session = sessionmaker(bind=engine)
session = Session()
```

First we create session, which is the object we use to manage the database. Next, we proceed by creating two people:

```python
anakin = Person(name='Anakin Skywalker', age=32)
obi1 = Person(name='Obi-Wan Kenobi', age=40)
```

We then add email addresses to both of them, using two different techniques. One assigns them to a list, and the other one simply appends them:

```python
[obi1 addresses](obi1.addresses) = [
    Address(email='obi1@example.com'),
    Address(email='wanwan@example.com'),
]

anakin.addresses.append(Address(email='ani@example.com'))
anakin.addresses.append(Address(email='evil.dart@example.com'))
anakin.addresses.append(Address(email='vader@example.com'))
```

We haven't touched the database yet. It's only when we use the session object that something actually happens in it:

```python
session.add(anakin)
session.add(obi1)
session.commit()
```

Adding the two Person instances is enough to also add their addresses (this is thanks to the cascading effect). Calling commit() is what actually tells SQLAlchemy to commit the transaction and save the data in the database. A transaction is an operation that provides something like a sandbox, but in a database context. 

As long as the transaction hasn't been committed, we can roll back any modification we have done to the database, and by doing so, revert to the state we were in before starting the transaction itself. SQLAlchemy offers more complex and granular ways to deal with transactions, which you can study in its official documentation, as it is quite an advanced topic. We now query for all the people whose name starts with Obi by using like(), which hooks to the LIKE operator in SQL:
```python

obi1 = session.query(Person).filter(
    Person.name.like('Obi%')
).first()
print(obi1, obi1.addresses)
```

We take the first result of that query (we know we only have Obi-Wan anyway), and print it. We then fetch anakin by using an exact match on his name, just to show you a different way of filtering:

```python
anakin = session.query(Person).filter(
    Person.name=='Anakin Skywalker'
).first()

print(anakin, anakin.addresses)
```

We then capture Anakin's ID, and delete the anakin object from the global frame (this does not delete the entry from the database):

```python
anakin_id = anakin.id
del anakin
```

The reason we do this is because we want to show you how to fetch an object by its ID. Before we do that, we write the display_info() function, which we will use to display the full content of the database (fetched starting from the addresses, in order to demonstrate how to fetch objects by using a relation attribute in SQLAlchemy):

```python
def display_info():
    # get all addresses first
    addresses = session.query(Address).all()

    # display results
    for address in addresses:
        print(f'{address.person.name} <{address.email}>')

    # display how many objects we have in total
    print('people: {}, addresses: {}'.format(
        session.query(Person).count(),
        session.query(Address).count())
    )
```

The display_info() function prints all the addresses, along with the respective person's name, and, at the end, produces a final piece of information regarding the number of objects in the database. We call the function, then we fetch and delete anakin. Finally, we display the info again, to verify that he has actually disappeared from the database:

```python
display_info()

anakin = session.query(Person).get(anakin_id)
session.delete(anakin)
session.commit()

display_info()
```

The output of all these snippets run together is the following (for your convenience, we have separated the output into four blocks, to reflect the four blocks of code that actually produce that output):
```bash

$ python alchemy.py
Obi-Wan Kenobi(id=2) [obi1@example.com, wanwan@example.com]

Anakin Skywalker(id=1) [
    ani@example.com, evil.dart@example.com, vader@example.com
]

Anakin Skywalker <ani@example.com>
Anakin Skywalker <evil.dart@example.com>
Anakin Skywalker <vader@example.com>
Obi-Wan Kenobi <obi1@example.com>
Obi-Wan Kenobi <wanwan@example.com>
people: 2, addresses: 5

Obi-Wan Kenobi <obi1@example.com>
Obi-Wan Kenobi <wanwan@example.com>
people: 1, addresses: 2
```

As you can see from the last two blocks, deleting anakin has deleted one Person object and the three addresses associated with it. Again, this is due to the fact that cascading took place when we deleted anakin.

This concludes our brief introduction to data persistence. It is a vast and, at times, complex domain that we encourage you to explore, learning as much theory as possible. Lack of knowledge or proper understanding, when it comes to database systems, can really bite.

## Summary

In this chapter, we have explored working with files and directories. We have learned how to open files for reading and writing and how to do that more elegantly by using context managers. We also explored directories: how to list their content, both recursively and not. We also learned about paths, which are the gateway to accessing both files and directories.

We then briefly saw how to create a ZIP archive and extract its content. The source code of the book also contains an example with a different compression format: tar.gz.

We talked about data interchange formats, and have explored JSON in some depth. We had some fun writing custom encoders and decoders for specific Python data types.

Then we explored I/O, both with in-memory streams and HTTP requests.

And finally, we saw how to persist data using pickle, shelve, and the SQLAlchemy ORM library.

You should now have a pretty good idea of how to deal with files and data persistence, and we hope you will take the time to explore these topics in much more depth by yourself.

The next chapter will look at cryptography and tokens.
