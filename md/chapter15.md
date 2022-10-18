# Packaging Python Applications

    "Do you have any cheese at all?"

    "No."

    – Monty Python, the "Cheese Shop" sketch

In this final chapter, we're going to learn how to create an installable package for your Python project and publish it for others to use. There are many reasons why you should publish your code. In Chapter 1, A Gentle Introduction to Python, we said that one of the benefits of Python is the vast ecosystem of third-party packages that you can install for free, using pip. Most of these were created by developers just like you and by contributing with your own projects, you will be helping to ensure that the community keeps thriving. In the long term, it will also help to improve your code, since exposing it to more users means bugs might be discovered sooner. Finally, if you are trying to get a job as a software developer, it really helps to be able to point to projects that you have worked on.

The best way to learn about packaging is to go through the process of creating a package and publishing it. That is exactly what we are going to do in this chapter. To make things more interesting, the project that we'll be working with will be a train schedule application built around the trains API from Chapter 14, Introduction to API Development. 

In this chapter, we are going to learn:

•  How to create a distribution package for your project
•  How to publish your package
•  About different tools for packaging

Before we dive into our train schedule project, we'll give you a brief introduction to the Python Package Index and some important terminology around Python packaging.

## The Python Package Index 

The Python Package Index (PyPI) is an online repository of Python packages, hosted at https://pypi.org. It has a web interface that can be used to browse or search for packages and view their details. It also has APIs for tools like pip to find and download packages to install. PyPI is open to anybody to register and distribute their projects for free. Anybody can also install any package from PyPI for free. The repository is organized into projects, releases, and distribution packages. A project is a library or script or application with its associated data or resources. For example, FastAPI, Requests, Pandas, SQLAlchemy, and our train schedule application are all projects. pip itself is a project as well. A release is a particular version (or snapshot in time) of a project. Releases are identified by version numbers. For example, pip 21.2.4 is a release of the pip project. Releases are distributed in the form of distribution packages. These are archive files, tagged with the release version, that contain the Python modules, data files, and so on, that make up the release. Distribution packages also contain metadata about the project and release, such as the name of the project, the authors, the release version, and dependencies that also need to be installed. Distribution packages are also referred to as distributions or just packages.

In Python, the word package is also used to refer to an importable module that can contain other modules, usually in the form of a folder containing a __init__.py file. It is important not to confuse this type of importable package with a distribution package. In this chapter we'll mostly use the term package to refer to a distribution package. Where there is ambiguity, we'll use the terms importable package or distribution package. 

Distribution packages can be either source distributions (also known as sdists), which require a build step before they can be installed, or built distributions, which only require the archive contents to be moved to the correct locations for an install. The format of source distributions is defined by PEP 517 (https://www.python.org/ dev/peps/pep-0517/). The standard built distribution format is called a wheel and was originally defined in PEP 427. The current version of the wheel specification can be found at https://packaging.python.org/specifications/binary-distribution-format/. The wheel format replaced the (now deprecated) egg built distribution format.

The Python Package Index was initially nicknamed the Cheese Shop, after the famous Monty Python sketch that we quoted from at the beginning of the chapter. Thus, the wheel distribution format is not named after the wheels of a car, but after wheels of cheese.

To help understand all of this, let's go through a quick example of what happens when we run pip install:

```bash
$ pip install --no-cache -v -v -v requests==2.26.0
```

We're telling pip to install release 2.26.0 of the requests project. By passing the -v command line option three times, we're telling pip to make its output as verbose as possible. We've also added the --no-cache command-line option to force pip to download packages from PyPI and not use any locally cached packages it might have. The output looks something like this (note that we've trimmed the output to fit on the page and omitted several lines):
```bash

...
1 location(s) to search for versions of requests:
* https://pypi.org/simple/requests/

```

Pip tells us that it has found information about the requests project at https:// pypi.org/simple/requests/. The output continues with a list of all the available distributions of the requests project:

  Found link https://.../requests-0.2.0.tar.gz..., version: 0.2.0
  ...
  Found link https://.../requests-2.26.0-py2.py3-none-any.whl...,
  version: 2.26.0
  Found link https://.../requests-2.26.0.tar.gz..., version:
  2.26.0

Now, pip collects the distributions for the release we've requested and downloads the most appropriate package. In this case, that is the wheel requests-2.26.0-py2. py3-none-any.whl:
```bash

Collecting requests==2.26.0
  ...
  Downloading requests-2.26.0-py2.py3-none-any.whl (62 kB)


```

Next, pip extracts the list of dependencies from the package and proceeds to find and download them all in the same way. Once all the required packages have been downloaded, they can be installed:
```python

Installing collected packages: urllib3, idna, charset-normalizer,
certifi, requests
...
Successfully installed certifi-2021.5.30 charset-normalizer-2.0.4
idna-3.2 requests-2.26.0 urllib3-1.26.6

```

If pip had downloaded a source distribution for any of the packages (which might happen if no suitable wheel is available), it would have needed to build the package before installing it.

Now that we know the difference between a project, a release, and a package, let's meet our train schedule project so that we can start working on packaging a release and publishing it.

## The train schedule project

The project we'll be working on is a simple application for showing train arrival and departure schedules. It allows you to select a station and see a list of all trains departing from or arriving at that station. All the data in the application comes from the trains API we built in Chapter 14, Introduction to API Development. The application provides both a tkinter GUI and a command-line interface (CLI), both of which were created using the tools and techniques we studied in Chapter 12, GUIs and Scripting.

You will need to have the trains API from Chapter 14 running for the train schedule application to work. We suggest that you open up a second console window and keep the API running there while you work through this chapter. 

The project lives in the train-project sub-folder in the source code for this chapter. The main (importable) package is called train_schedule. We won't go through the code in detail here. Instead, we'll explain how it is structured and you can study the code in more detail yourself. It is all based on the techniques and concepts that you have learned in the book and we have left comments throughout the code to help you understand what each part does. Later on in the chapter, we will look at particular parts of the code in more detail, to explain how they relate to packaging.

Let's use the tree command to see how the code is organized:
```bash

$ tree train_schedule
train_schedule
├── api
│   ├── __init__.py
│   └── schemas.py
├── models
│   ├── __init__.py
│   ├── event.py
│   ├── stations.py
│   └── trains.py
├── views
│   ├── __init__.py
│   ├── about.py
│   ├── config.py
│   ├── dialog.py
│   ├── formatters.py
│   ├── main.py
│   ├── stations.py
│   └── trains.py
├── __init__.py
├── __main__.py
├── cli.py
├── config.py
├── gui.py
├── icon.png
├── metadata.py
└── resources.py

```

We can see that train_schedule is an importable Python package with three sub-packages: api, models, and views.

The api package defines our interface to the trains API. Within the api/__init__.py file you will find a TrainAPIClient class, which is responsible for all communication with the API. The schemas.py module defines pydantic schemas to represent the Train and Station objects that we receive from the API. The GUI application is structured according to the Model-View-Controller (MVC) design pattern. The models package contains the models that are responsible for managing the station and train data (in the models/stations.py and models/trains. py modules). The models/events.py module implements a callback mechanism that allows other parts of the application to be notified when the data in the models are updated.

The MVC pattern is a commonly used pattern for developing graphical user interfaces and web applications. You can read more about it at https://en.wikipedia.org/wiki/Model-view-
controller. 

The views package implements the views that are responsible for presenting the data to the user. The views/stations.py module defines a StationChooser class that displays stations in a drop-down list so that the user can select the station they're interested in. In views/trains.py we have a TrainsView class that displays information about trains in a tabular format. views/formatters.py defines some helper functions for formatting data. The main application window is defined in views/main.py, while views/dialog.py contains a base class for dialogs that is used in views/about.py and views/config.py to create an About dialog and a Configuration dialog.

The gui.py module defines a TrainApp class, which is the controller that is responsible for coordinating interaction between the models and the views.

The command-line interface of the application is defined in cli.py. In config.py we define a pydantic settings class for handling application configuration. To make our code more portable, we use the very handy platformdirs package (see https:// pypi.org/project/platformdirs/) to store a configuration file in an appropriate location for whatever platform we are running on.

The resource.py and metadata.py modules contain some code for dealing with data files (like icon.png) and metadata included in the distribution package. We'll talk about these in more detail later on in the chapter.

The only module that is left to talk about is __main__.py. Within an importable Python package, the module name __main__.py gets special treatment. It allows the package to be executed as a script. 

Let's see what this file looks like and what happens when we run it:
```python

# train/train_schedule/__main__.py
import sys

from .cli import main as cli_main
from .gui import main as gui_main

if __name__ == "__main__":
    if len(sys.argv) > 1:
        cli_main()
    else:
        gui_main()

```

We have some imports to get the standard library sys module and to load the main functions from our cli and gui modules, aliasing them as cli_main and gui_main. Then we have the same if __name__ == "__main__" check that we saw in Chapter 12. Finally, we resort to a simple trick to allow us to run either the CLI or GUI versions of the application, depending on whether we have command-line arguments or not. We do this by checking the length of sys.argv, which is a list containing the command-line arguments to our script. The length is always at least 1, since sys. argv[0] contains the name our script was run with. If the length is greater than 1, it means we do have arguments and we invoke cli_main, otherwise we run gui_main. 

Let's see it in action:
```bash

$ python -m train_schedule

```

To execute a package with a __main__.py file as a script, we have to use the -m option to the Python interpreter. This tells the interpreter to import the package first before executing it. Without this option, the interpreter will attempt to run the __main__.py file as a standalone script, which will fail because it won't be able to import other modules from the package. You can read more about this at https://docs.python.org/3/using/cmdline.html. 

You will need to install the dependencies listed in the requirements/main.txt file in the chapter's source code to be able to run the application. You can ignore the requirements/build.txt file for now. It lists dependencies that we will need later in the chapter to build and publish our distribution packages.

The first time we run the application, we're presented with a configuration dialog prompting us to enter the URL for the train API.

Figure 15.1 shows what this looks like on a Windows machine:

Figure 15.1: The train schedule configuration dialog

Type in http://localhost:8000 and click OK to proceed. The URL will be saved to a configuration file, so next time we run the app we won't be asked to enter it again. If we want to change the URL, we can open the configuration dialog again by clicking on Preferences… under the Edit menu. Now we can select a station from the drop-down list to see arrivals and departures. Figure 15.2 shows what it looks like when we select the station in Rome:

```bash

$ python -m train_schedule stations
0: Rome, Italy (ROM)
1: Paris, France (PAR)
2: London, UK (LDN)
3: Kyiv, Ukraine (KYV)
4: Stockholm, Sweden (STK)
5: Warsaw, Poland (WSW)
6: Moskow, Russia (MSK)
7: Amsterdam, Netherlands (AMD)
8: Edinburgh, Scotland (EDB)
9: Budapest, Hungary (BDP)
10: Bucharest, Romania (BCR)
11: Sofia, Bulgaria (SFA)

```
As you can see, we get a printout with the id, city, country, and code of each station.

We'll leave it up to you to explore further. If there's something you don't understand, look it up in the official Python documentation or refer back to previous chapters of the book. Some of the techniques we discussed in Chapter 11, Debugging and Profiling, such as adding print statements (or a custom debug function), can also be useful to help you understand what's happening while the code is running.

Now that we're familiar with our project, we can start working on preparing a release and building distribution packages.

## Packaging with setuptools

We will be using the setuptools library to package our project. setuptools is currently the most popular packaging tool for Python. It is an extension of the original, standard library distutils packaging system. setuptools has many features that are not available in distutils and is more actively maintained. Direct use of distutils has been actively discouraged for many years. The distutils module will be deprecated in Python 3.10 and removed from the standard library in Python 3.12.

In this section, we'll be looking at how to set up our project to build packages with setuptools.

### Required files
To build a package and publish it, we need to add some files to our project. If you look at the content of the train-project folder in the chapter source code, you will see the following files alongside the train_schedule folder:
```bash

CHANGELOG.md LICENSE MANIFEST.in README.md pyproject.toml setup.cfg 
setup.py

```

We'll discuss each of these in turn, starting with pyproject.toml. 

#### pyproject.toml

This file was introduced by PEP 518 (https://www.python.org/dev/peps/pep-0518/) 
and extended by PEP 517 (https://www.python.org/dev/peps/pep-0517/). The 
aim of these PEPs was to define standards to allow projects to specify their build 
dependencies, and to specify what build tool should be used to build their packages. 
For a project using setuptools, this looks like:
```python

# train-project/pyproject.toml
[build-system]
requires = ["setuptools>=51.0.0", "wheel"]
build-backend = "setuptools.build_meta"

```

Here we have specified that we require at least version 51.0.0 of setuptools and any release of the wheel project, which is the reference implementation of the wheel distribution format. Note that the requires field here does not list dependencies for running our code, only for building a distribution package. We'll talk about how to specify dependencies for running our project later.

The build-backend specifies the Python object that will be used to build packages. For setuptools, this is the build_meta module in the setuptools (importable) package.

The pyproject.toml file uses the TOML configuration file format. You can learn more about TOML at https://toml.io/en/.

PEP 518 also allows putting configuration for other development tools in the pyproject.toml file. Of course, the tools in question also need to support reading their configuration from this file.
```python

# train-project/pyproject.toml
[tool.black]
line-length = 66

[tool.isort]
profile = 'black'
line_length = 66


```
We have added configuration for black, a popular code formatting tool, and isort, a tool for sorting imports alphabetically, to our pyproject.toml file. We've configured both tools to use a line length of 66 characters to ensure our code will fit on a book page. We've also configured isort to maintain compatibility with black.

You can learn more about black and isort on their websites at https://black.readthedocs.io/en/stable/index.html and https://pycqa.github.io/isort/index.html.

### License

You should include a license that defines the terms under which your code is distributed. There are many software licenses that you can choose from. If you're not sure which to use, the website https://choosealicense.com/ is a useful resource to help you. However, if you are at all in doubt about the legal implications of any particular license or need advice, you should consult a legal professional.

We are distributing our train schedule project under the MIT license. This is a simple license that allows anyone to use, distribute, or modify the code as long as they include our original copyright notice and license.

By convention, the license is included in a text file named LICENSE or LICENSE.txt although some projects also use other names such as COPYING.

### README

Your project should also include a README file describing the project, why it exists, and even some basic usage instructions. The file can be in plain text format or use a markup syntax like reStructuredText or Markdown. The file is typically called README or README.txt if it is a plain text file, README.rst for reStructuredText, or README.md for Markdown. 

Our README.md file contains a short paragraph describing the purpose of the project and some simple usage instructions.

### Changelog
  
Although it is not required, it is considered good practice to include a changelog file with your project. This file summarizes the changes made in each release of your project. The changelog is useful for informing your users of new features that are available or of changes in the behavior of your software that they need to be aware of.

Our changelog file is named CHANGELOG.md and is written in Markdown format.

### setup.cfg

The setup.cfg file is the configuration file for setuptools. It is an INI style configuration file with key = value entries organized into sections, each of which starts with a name in square brackets, for example [metadata].

setup.cfg can be used to configure all your project's metadata. It can also define which (importable) packages, modules, and data files need to be included in your distribution packages. We will go through the contents of our setup.cfg file in the next few sections of this chapter.

Like pyproject.toml, setup.cfg can also be used to configure other tools. We have the following bit of configuration for the flake8 tool at the bottom of our setup.cfg file:
```python

# train-project/setup.cfg
[flake8]
max-line-length = 66

```

flake8 is a Python code style checker that can help to point out PEP 8 violations in our code. We have configured it to warn us if any lines in our code are longer than 66 characters. As we've mentioned before, this is shorter than the 80 characters mandated by PEP 8, to make sure our code fits onto the pages of this book.

### setup.py

Since setuptools implemented PEP 517, the setup.py file is no longer required. It is only needed to support legacy tools that are not PEP 517-compliant. If the file exists, it must be a Python script that calls the setuptools.setup() function. Our version looks like this:
```python

# train-project/setup.py
import setuptools

setuptools.setup()

```

All the options that can be configured in setup.cfg can also be passed as keyword arguments to the setuptools.setup() function instead. This can be useful for projects where some package options or metadata need to be computed dynamically and cannot be statically configured in setup.cfg. This should generally be avoided, though, and as much as possible should be configured via setup.cfg.

Older versions of setuptools did not support configuring package options via setup.cfg, so projects had to pass everything as setup arguments in setup.py. Many projects still do this, since they started using setuptools before configuration via setup. cfg was supported and they have not had a compelling reason to change. 

The main reason for still including a setup.py file in a new project is to allow us to do an editable install of our project. Instead of building a wheel and installing that in your virtual environment, an editable install just puts a link to your project source folder in your virtual environment. This means that Python will behave as if your package has been installed from an sdist or a wheel, but any changes you make to the code will take effect without needing to rebuild and reinstall.

The main benefit of installing your project in a virtual environment during development is that your code will behave more similarly to when someone else installs it from a distribution package. For example, you won't need to run Python inside your project folder to import your code. This will make it easier for you to spot bugs that can occur if your code makes invalid assumptions about the environment it executes in. An editable install makes it easier to do this, because you won't have to reinstall every time you change something.

The reason we need a setup.py file to do an editable install is that PEP 517 does not support such installs. Therefore, pip has to fall back to the legacy behavior of executing the setup.py script directly.

Let's try it. Navigate to the ch15 folder in the book's source code, activate the virtual environment, and run the following command:
```python

$ python -m train_schedule

```

You should get an error like this:
```bash

/.../ch15/.venv/bin/python: No module named train_schedule


```
Now, let's install and try again. We can do an editable install by passing the -e option to pip install:
```bash

$ pip install -e ./train-project


```
After pip has run successfully, we can run our application again:
```bash

$ python -m train_schedule


```

This time, it should work. You can verify that we really have an editable install by stopping the application, changing something in the code, running it again, and seeing what happens.

If you look at the contents of the train-project folder after running pip install -e, you will see there is a new train_ schedule.egg-info folder. This folder contains the metadata for our editable install. The egg-info suffix in the name is a leftover from the old egg distribution format. 

### MANIFEST.in

By default, only a limited set of files will be included in a source distribution. The MANIFEST.in file can be used to add any other files that may be needed to build and install your package from an sdist. This is often required if you have configured setuptools to read some package metadata from files included in your project. We'll show you an example of what this looks like in a moment.

You can find details of the exact list of files that are automatically included in source distributions along with details of the syntax of the MANIFEST.in file at https://packaging.python.org/guides/using-manifest-in/.

Now that we've covered all the files that need to be added to our project, let's take a closer look at the contents of setup.cfg.

### Package metadata

The first section of our setup.cfg file defines our package metadata. Let's go through it a few entries at a time:
```yaml

# train-project/setup.cfg
[metadata]
name = train-schedule
author = Heinrich Kruger, Fabrizio Romano
author_email = heinrich@example.com, fabrizio@example.com
```

The metadata section starts with the [metadata] heading. Our first few metadata entries are quite simple. We define the name of our project, identify the authors using the author field, and list the author email addresses in the author_email field. 

We've used fake email addresses for this example project, but for a real project you should use your real email address.

PyPI requires that all projects must have unique names. It's a good idea to check this when you start your project, to make sure that no other project has already used the name you want. It's also advisable to make sure your project name won't easily be confused with another; this will reduce the chances of anyone accidentally installing the wrong package.

Next, we have the release version:
```yaml

# train-project/setup.cfg
[metadata]
...
version = 1.0.0

```

The version field specifies the version number of your release. You can choose any versioning scheme that makes sense for your project, but it must comply with the rules defined in PEP 440 (https://www.python.org/dev/peps/pep-0440/). A PEP 440-compatible version consists of a sequence of numbers, separated by dots, followed by optional pre-release, post-release, or developmental release indicators. A pre-release indicator can consist of the letter a (for alpha), b (for beta), or rc (for release- candidate), followed by a number. A post-release indicator consists of the word post followed by a number. A developmental release indicator consists of the word dev followed by a number. A version number without a release indicator is referred to as a final release. For example:

•  1.0.0.dev1 would be the first developmental release of version 1.0.0 of our project
•  1.0.0.a1 would be our first alpha release
•  1.0.0.b1 is a beta release
•  1.0.0.rc1 is our first release candidate
•  1.0.0 is the final release of version 1.0.0
•  1.0.0.post1 is our first post-release

Developmental releases, pre-releases, final releases, and post-releases with the same main version number are ordered as in the list above. Popular versioning schemes include semantic versioning, which aims to convey information about compatibility between releases through the versioning scheme, and date-based versioning, which typically uses the year and month of a release to indicate the version.

Semantic versioning uses a version number consisting of three numbers, called the major, minor, and patch versions, separated by dots. This results in a version that looks like major.minor.patch. If a new release is completely compatible with its predecessor, only the patch number is incremented; usually such a release only contains small bug fixes. For a release that adds new functionality without breaking compatibility with previous releases, the minor number should be incremented. The major number should be incremented if the release is incompatible with older versions. You can read all about semantic versioning at https://semver.org/.

The next few entries in our setup.cfg are descriptions of our project:
```yaml

# train-project/setup.cfg
[metadata]
...
description = A train app to demonstrate Python packaging
long_description = file: README.md, CHANGELOG.md
long_description_content_type = text/markdown

```

The description field should be a short, single sentence summary of the project, while long_description can contain a longer, more detailed description. We've told setuptools to use the concatenation of our README.md and CHANGELOG.md files as the long description. The long_description_content_type specifies the format of the long description; in our case we use text/markdown to specify that we used Markdown format.

We need our README.md and CHANGELOG.md files to be included in the source distribution, so that they can be added to the long description in the package metadata when we build or install from a source distribution. README.md is one of the filenames that setuptools automatically includes in the source distribution, but CHANGELOG.md is not. Therefore, we have to explicitly include it in our MANIFEST.in file:
```bash
# train-project/MANIFEST.in
include CHANGELOG.md
```
The next metadata entries in our setup.cfg file are some URLs for our project:

```yaml
# train-project/setup.cfg
[metadata]
...
url = https://github.com/PacktPublishing/Learn-Python-Programming-
Third-Edition
project_urls =
    Learn Python Programming Book = https://www.packtpub.com/...

```

The url field contains the homepage for your project. It's quite common for projects to use this to link to their source code on a code hosting service such as GitHub or GitLab, which is what we've done here. The project_url field can be used to specify any number of additional URLs. The URLs can either be entered on a single line as comma-separated key = value pairs, or as one key = value pair per line as we have done. This is often used to link to online documentation, bug trackers, or other websites related to the project. We've used this field to add a link to information about our book on the publisher's website.

The project's license should also be specified in the metadata: 
```yaml

# train-project/setup.cfg
[metadata]
...
license = MIT License
license_files = LICENSE

```

The license field is used to name the license the project is distributed under, while the license_files field lists files related to the project license that should be included in the distribution. Any files listed here are automatically included in the distribution and do not need to be added to the MANIFEST.in file.

Our last couple of metadata entries are meant to help potential users find our project on PyPI:
```yaml

# train-project/setup.cfg
[metadata]
...
classifiers =
    Intended Audience :: End Users/Desktop
    License :: OSI Approved :: MIT License
    Operating System :: MacOS
    Operating System :: Microsoft :: Windows
    Operating System :: POSIX :: Linux
    Programming Language :: Python :: 3
    Programming Language :: Python :: 3.8
    Programming Language :: Python :: 3.9
    Programming Language :: Python :: 3.10
keywords = trains, packaging example
```

The classifiers field can be used to specify a list of trove classifiers, which are used to categorize projects on PyPI. The PyPI website allows users to filter by trove classifier when searching for projects. Your project's classifiers must be chosen from the list of official classifiers at https://pypi.org/classifiers/. 

We've used classifiers to indicate that our project is aimed at desktop end users, that it's released under the MIT license, that it works on macOS, Windows, and Linux, and that it's compatible with Python 3 (specifically versions 3.8, 3.9, and 3.10). Note that the classifiers are there purely to provide information to users and help them find your project on the PyPI website. They have no impact on which operating systems or Python versions your package can actually be installed on.

The keywords field can be used to provide additional keywords to help users find your project. Unlike the classifiers, there are no restrictions on what keywords you can use.

## Accessing metadata in your code

It is often useful to be able to access your distribution metadata in your code. For example, many libraries expose their release version as a __version__ or VERSION attribute. This allows other packages to adapt themselves to the version of the library that is installed (for example, to work around a known bug in a particular version). Having access to the metadata in the code is one way of doing this, without needing to keep the version up to date in two places.

In our project, we use the project name, author, description version, and license information from the metadata in the About dialog of our application. Let's see how we did it:
```python

# train-project/train_schedule/metadata.py
from importlib.metadata import PackageNotFoundError, metadata

def get_metadata():
    try:
        meta = metadata(__package__)
    except PackageNotFoundError:
        meta = {
            "Name": __package__.replace("_", "-"),
            "Summary": "description",
            "Author": "author",
            "Version": "version",
            "License": "license",
        }
    return meta


```
We use the metadata function from the importlib.metadata standard library module to load the package metadata. 

To get the metadata, we have to give the metadata the name of our (importable) package. The interpreter makes this available to us via the __package__ global variable.

The importlib.metadata module was added in Python 3.8. If you need to support older versions of Python, check out the importlib-metadata project: https://pypi.org/project/importlib-metadata/.

The metadata function returns a dict-like object with the metadata. The keys are similar to, but not quite the same as, the names of the setup.cfg entries we use to define the metadata. Details of all the keys and their meanings can be found in the metadata specification at https://packaging.python.org/specifications/core-metadata/. 

The metadata can only be accessed if the package has been installed. If the package is not installed, we get a PackageNotFoundError. To make sure it's possible to run the code even if it is not installed, we catch the exception and supply some dummy values for metadata keys that we know we want to use. We use the metadata in our __init__.py file, to set some global variables:

```python
# train-project/train_schedule/__init__.py
from .metadata import get_metadata

_metadata = get_metadata()

APP_NAME = _metadata["Name"]
APP_TITLE = APP_NAME.title()
VERSION = _metadata["Version"]
AUTHOR = _metadata["Author"]
DESCRIPTION = _metadata["Summary"]
LICENSE = _metadata["License"]

ABOUT_TEXT = f"""{APP_TITLE}

{DESCRIPTION}

Version: {VERSION}
Authors: {AUTHOR}
License: {LICENSE}
Copyright: © 2021 {AUTHOR}"""

```

If we run the application without installing it first, and we click on About… in the Help menu, we can see our dummy values in the About dialog. Figure 15.3 shows what this looks like:

Figure 15.3: The train schedule About dialog

It's not ideal, but during development it's at least good enough for us to be able to see that the dialog works as it should and that we're displaying the correct dummy values in the correct places. It would be better to see the dialog as our users will see it when they have installed our package from PyPI. That way we can be sure that the metadata is read and displayed correctly. If we do an editable install and run the application again, the About dialog looks like the image in Figure 15.4:

Figure 15.4: The About dialog when the app has been installed

That looks much better. This is also what our users will see when they install and run the application.

That brings us to the end of the metadata section of our setup.cfg file. Next up we have the [options] section, which is used to define the package contents and dependencies. Let's start by looking at how to specify the package contents.

## Defining the package contents

The most important content we need to add to our distribution is, of course, our code. We do this by using the packages and py_modules options for setuptools. The packages option takes a list of (importable) packages that need to be added to the distribution. You have to list both the top-level packages (for example train_ schedule) and every sub-package (like train_schedule.api, train_schedule.models, and so on). In bigger projects this can be a serious nightmare, especially if you decide to rename or reorganize your packages. Fortunately, there is a solution. You can use the find: directive to tell setuptools to automatically include any importable packages it finds in your project folder.

Let's see what this looks like:
```yaml
# train-project/setup.cfg
[options]
packages = find:
```

That's quite easy, isn't it? If you need to, you also have the option of configuring where in your project folder setuptools looks for packages and specify packages that should be included or excluded from the search. For more detail, please consult the setuptools documentation on this topic at https://setuptools.readthedocs.io/en/latest/userguide/package_discovery.html.

The py_modules option lets you specify a list of Python modules that do not belong to importable packages, that should be included in your distribution. This should be a list of top-level Python files, without the .py extension. The py_modules option does not support a find: directive like the packages option does. This is not really a problem, because it is quite rare for projects to have more than a couple of modules that do not belong to packages (our project doesn't have any).

Sometimes, you also need to include non-code files in your distribution. For our project, we want to include the icon.png file, so that we can display it in the title bar of our windows and in the About dialog. The easiest way to do this is to use the include_package_data option in your setup.cfg file:
```yaml
# train-project/setup.cfg
[options]
...
include_package_data = True

```
You also have to list the files you want to distribute in your MANIFEST.in file:
```yaml

# train-project/MANIFEST.in
include train_schedule/icon.png

```

Note that the data files that you include must be placed inside an importable package.

## Accessing package data files

Now that we can distribute data files in our package, we need to know how to access those files when our package is installed in a virtual environment on a user's computer. In your development environment, it is easy and therefore quite tempting to just hard-code the path to the file relative to your project folder. This won't work when your code is installed in a virtual environment and run from outside your project folder. To make matters worse, it is not even guaranteed that the data file will exist as a separate file on the filesystem. For example, Python has the ability to import packages and modules from ZIP archives.

Fortunately, Python does provide a mechanism to access data files (referred to as resources) from within packages. The tools to do this can be found in the importlib. resources module. Let's see an example from our project:
```python

# train-project/train_schedule/resources.py
from importlib import resources

def load_binary_resource(name):
    return resources.read_binary(__package__, name)

```

We have defined our own load_binary_resource() helper function, which uses importlib.resources.read_binary() to read binary data from a package resource. To do this, we have to supply the name of the package and the name of the resource, which is just the name of the data file. This function is analogous to:

```python
with open(resource, "rb") as stream:
    return stream.read()
```

except that Python is doing more work behind the scenes, since it's not guaranteed that the resource actually exists as a file on disk.

Let's see how we use our load_binary_resource() helper function in our train_schedule/views/main.py module to load the icon.png file (note that we've omitted some code here to only show you the relevant bits):
```python

# train-project/train_schedule/views/main.py
import tkinter as tk
from ..resources import load_binary_resource

ICON_FILENAME = "icon.png"

class MainWindow:
    def __init__(self):
        self.root = tk.Tk()
        self._set_icon()

    def _set_icon(self):
        """Set the window icon"""
        self.icon = tk.PhotoImage(
            data=load_binary_resource(ICON_FILENAME)
        )
        self.root.iconphoto(True, self.icon)
```

We import the tkinter module and our load_binary_resource() function. We use a global variable ICON_FILENAME to define the name of the icon file. The _set_icon() method of the MainWindow class calls load_binary_resource() to read the image data and uses it to create a tkinter PhotoImage object. We pass this PhotoImage object to the self.root Tk object's iconphoto() method to set the window icon.

As you can see, it's just as easy to load data from a package data file in our distribution as it is to read from any other file. The importlib.resources module has several other useful functions; we encourage you to read about them in the standard library documentation. Right now, though, it's time for us to see how to specify dependencies for our package.

## Specifying dependencies

As we saw at the beginning of the chapter, a distribution package can provide a list of projects it depends on and pip will ensure that releases of those projects are installed when it installs the package. Of course, this means that we need to specify our dependencies to setuptools when we build a package. 

We do this by using the install_requires option:
```yaml

# train-project/setup.cfg
[options]
...
install_requires =
    platformdirs>=2.0
    pydantic>=1.8.2,<2.0
    requests~=2.0

```

As you can see, our project depends on the platformdirs, pydantic, and requests projects. We can also use version specifiers to indicate which releases of dependencies we require. Besides the normal Python comparison operators, version specifiers can also use ~= to indicate a compatible release. The compatible release specifier is a way of indicating releases that may be expected to be compatible under a semantic versioning scheme. For example, in our case requests~=2.0 means that we require any 2.x version of the requests project, from 2.0 up to 3.0 (not included). A version specifier can also take a comma-separated list of version clauses that must all be satisfied. For example, pydantic>=1.8.2,<2.0 means that we want at least version 1.8.2 of pydantic, but not version 2.0 or greater. Note that this is not the same as pydantic~=1.8.2, which would mean at least version 1.8.2, but not version 1.9 or greater. For the full details of the dependency syntax and how versions are matched, please refer to PEP 508 (https://www.python.org/dev/peps/pep-0508/).

You should be careful not to make your install_requires version specifiers too strict. Bear in mind that your package is likely to be installed alongside various other packages in the same virtual environment. This is particularly true of libraries or tools for developers. Allowing as much freedom as possible in the required versions of your dependencies means that projects that depend on yours are less likely to encounter dependency conflicts between your package and those of other projects they depend on. Making your version specifiers too restrictive also means that your users won't benefit from bug fixes or security patches in one of your dependencies unless you also publish a new release to update your version specifier.

Apart from dependencies on other projects, you can also specify which versions of Python your project requires. In our project, we use features that were added in Python 3.8, so we specify that we require at least Python 3.8:

# train-project/setup.cfg
[options]
...
python_requires = >=3.8

As with install_requires, it is generally best to avoid limiting the Python versions you support too much. You should only restrict the Python version if you know your code won't work on all actively supported versions of Python 3.

You can find a list of Active Python Releases on the official Python download page: https://www.python.org/downloads/. 

You should make sure that your code does actually work on all the versions of Python and the dependencies that you support in your setup configuration. One way of doing this is to create several virtual environments with different Python versions and different versions of your dependencies installed. Then you can run your test suite in all these environments. Doing this manually would be very time consuming. Fortunately, there are tools that will automate this process for you. The most well- known of these tools is called tox. You can find out more about it at https://tox.readthedocs.io/en/latest/.

You can also specify optional dependencies for your package. Pip will only install such dependencies if a user specifically requests it. This is useful if some dependencies are only required for a feature that many users are not likely to need. Users who want the extra feature can then install the optional dependency and everyone else gets to save disk space and network bandwidth. For example, the PyJWT project, which we used in Chapter 9, Cryptography and Tokens, depends on the cryptography project to sign JWTs using asymmetric keys. Many users of PyJWT do not use this feature, so the developers made cryptography an optional dependency.

Optional (or extra) dependencies are specified in the [options.extras_require] section of the setup.cfg file. This section can contain any number of named lists of optional dependencies. These lists are referred to as extras. In our project we have one extra, called dev:
```yaml

# train-project/setup.cfg
[options.extras_require]
dev =
    black
    flake8
    isort
    pdbpp

```

This is a common convention for listing tools that are useful during development of a project as optional dependencies. Many projects also have an extra test dependency, for installing packages that are only needed to run the project's test suite.

To include optional dependencies when installing a package, you have to add the names of the extras you want in square brackets when you run pip install. For example, to do an editable install of our project with the dev dependencies included, you can run:
```bash
$ pip install -e ./train-project[dev]
```

We are nearly done with our setuptools configuration. There's only one more section to add before we can build our package and publish it.

## Entry points

So far, we've been running our application by typing:

```bash
$ python -m train_schedule
```

This is not particularly user-friendly. It would be much better if we could run our application by just typing:

```bash
$ train-schedule
```
The good news is that this is possible. We can achieve this by configuring script entry points for our distribution. A script entry point is a function that we want to be able to execute as a command-line or GUI script. When our package is installed, pip will automatically generate scripts that import the specified functions and run them.

We configure entry points in the [options.entry_points] section of the setup.cfg file. Let's see what ours looks like:
```yaml

# train-project/setup.cfg
[options.entry_points]
console_scripts =
    train-schedule-cli = train_schedule.cli:main
gui_scripts =
    train-schedule = train_schedule.gui:main

```

The entry_points configuration contains a number of groups, each of which contains a mapping of names to object references. The console_scripts group is used to define command-line, or console, scripts, while the gui_scripts group defines GUI scripts. We've defined a console script called train-schedule-cli, mapped to the main() function in the train_schedule.cli module; and a GUI script called train-schedule, mapped to the main() function in the train_schedule.gui module.

The Windows operating system treats console and GUI applications differently. Console applications are launched in a console window and can print to the screen and read keyboard input through the console. GUI applications are launched without a console window. On other operating systems, there is no difference between console_scripts and gui_scripts. 

When pip installs our package, it will generate scripts called train-schedule and train-schedule-cli and put them in the bin folder of our virtual environment (or Scripts, if you're using Windows).

The console_scripts and gui_scripts entry point group names have special meanings, but you can also use other group names. Pip won't generate scripts for entry points in other groups, but they can be useful for other purposes. Specifically, many projects that support extending their functionality via plugins use particular entry point group names for plugin discovery. This is a more advanced subject that we won't discuss in detail here, but if you are interested you can read about it in the setuptools documentation: https://setuptools.readthedocs.io/en/latest/userguide/entry_point.html. 

That brings us to the end of the preparation phase. The setuptools configuration is complete and we have everything we need to start building our distribution packages.

## Building and publishing packages

We are going to be using the package builder provided by the build project (https:// pypi.org/project/build/) to build our distribution package. We're also going to need the twine (https://pypi.org/project/twine/) utility to upload our packages to PyPI. You can install these tools from the requirements/build.txt file provided with the source code of this chapter. We recommend installing these into a new virtual environment.

Because project names on PyPI have to be unique, you won't be able to upload the train-schedule project without changing the name first. You should change the name in the setup.cfg file to something unique before building distribution packages. Bear in mind that this means that the filenames of your distribution packages will also be different from ours. 

### Building

The build project provides a simple script for building packages according to the PEP 517 specification. It will take care of all the details of building distribution packages for us. When we run it, build will do the following:

1.  Create a virtual environment.
2. Install the build requirements listed in the pyproject.toml file into the virtual environment. 
3. Import the build backend specified in the pyproject.toml file and run it to build a source distribution.
4.  Create another virtual environment and install the build requirements.
5. Import the build backend and use it to build a wheel from the source distribution built in step 3.

Let's see it in action. Enter the train-project folder in the chapter's source code, and run the following command:
```bash

$ python -m build
* Creating venv isolated environment...
* Installing packages in isolated environment...
 (setuptools>=51.0.0, wheel)
* Getting dependencies for sdist...
...
* Building sdist...
...
* Building wheel from sdist
* Creating venv isolated environment...
* Installing packages in isolated environment...
 (setuptools>=51.0.0, wheel)
* Getting dependencies for wheel...
...
* Installing packages in isolated environment... (wheel)
* Building wheel...
...
Successfully built train-schedule-1.0.0.tar.gz and
 train_schedule-1.0.0-py3-none-any.whl
```

We've removed a lot of lines from the output to make it easier to see how it follows the steps we listed above. If you look at the content of your train-project folder, you'll notice there is a new folder called dist with two files: train-schedule- 1.0.0.tar.gz is our source distribution, and train_schedule-1.0.0-py3-none-any.whl is the wheel.

Before uploading your package, it's a good idea to do a couple of checks to make sure it built correctly. First, we can use twine to verify that the long_description will render correctly on the PyPI website:
```bash
$ twine check dist/*
Checking dist/train_schedule-1.0.0-py3-none-any.whl: PASSED
Checking dist/train-schedule-1.0.0.tar.gz: PASSED
```

If twine reports any problems, you should fix them and rebuild the package. In our case the checks passed, so let's install our wheel and make sure it works. In a separate virtual environment, run:
```bash

$ pip install dist/train_schedule-1.0.0-py3-none-any.whl

```

With the wheel installed in your virtual environment, try to run the application, preferably from outside the project directory. If you encounter any errors during installation or when running your code, check your setup configuration carefully for typos. setuptools will ignore any sections or options it doesn't recognize, so misspelled names can result in broken wheels. Also make sure that any data files your project relies on are correctly listed in your MANIFEST.in file. Our package seems to have built successfully, so let's move on to publishing it.

### Publishing

Since this is only an example project, we'll upload it to TestPyPI instead of the real PyPI. This is a separate instance of the package index that was created specifically to allow developers to test package uploads and experiment with packaging tools and processes.

Before you can upload packages, you will need to register an account. You can do this now, by going to the TestPyPI website at https://test.pypi.org and clicking on Register. Once you've completed the registration process and verified your email address, you will need to generate an API token. You can do so on the Account Settings page of the TestPyPI website. Make sure you copy the token and save it before closing the page. You should save your token to a file named .pypirc in your user home directory. The file should look like this:
```yaml

[testpypi]
  username = __token__
  password = pypi-...
```
The password value should of course be replaced with your actual token. We strongly recommend that you enable two-factor authentication for both your TestPyPI account and especially your real PyPI account. 

Now you are ready to run twine to upload your distribution packages:
```bash

$ twine upload --repository testpypi dist/*
Uploading distributions to https://test.pypi.org/legacy/
Uploading train_schedule-1.0.0-py3-none-any.whl
100%|████████████████████████| 28.0k/28.0k [00:01<00:00, 19.8kB/s]
Uploading train-schedule-1.0.0.tar.gz
100%|████████████████████████| 23.0k/23.0k [00:01<00:00, 23.6kB/s]

View at:
https://test.pypi.org/project/train-schedule/1.0.0/
```

twine displays handy progress bars to show how the uploads are progressing. Once the upload is complete, it prints out a URL where you can see details of your package. Open it in your browser and you should see our project description with the contents of our README.md and CHANGELOG.md files. On the left of the page, you'll see links for the project URLs, the author details, license information, keywords, and classifiers.

Figure 15.5 shows what this page looks like for our train-schedule project. You should check all the information on the page carefully and make sure it matches what you expect to see. If not, you will have to fix the metadata in your setup.cfg, rebuild, and re-upload.

PyPI won't let you re-upload distributions with the same filenames as previously uploaded packages. To fix your metadata, you will have to increment the version of your package. For this reason, it is a good idea to use developmental release numbers until you are 100% sure everything is correct. 

Now we can install our package from the TestPyPI repository. Run the following in a new virtual environment:
```bash

pip install --index-url https://test.pypi.org/simple/ \
    --extra-index-url https://pypi.org/simple/ train-schedule==1.0.0

```

The --index-url option tells pip to use https://test.pypi.org/simple/ as the main package index. We use --extra-index-url https://pypi.org/simple/ to tell pip to also look up packages in the standard PyPI index, so that dependencies that aren't available in the TestPyPI index can be installed. The package installs successfully, which confirms that our package was built and uploaded correctly.

If this were a real project, we would now proceed to uploading to the real PyPI. The process is the same as for TestPyPI. When you save your PyPI API key, you should add it to your existing .pypirc file under the heading [pypi], like this: 
```yaml

[pypi]
  username = __token__
  password = pypi-…

```

You also don't need to use the --repository option to upload your package to the real PyPI; you can just run the following:
```bash

$ twine upload dist/*

```

As you can see, it's not very difficult to package and publish a project, but there are quite a few details to take care of. The good news is that most of the work only needs to be done once, when you publish your first release. For subsequent releases, you usually just need to update the version and maybe adjust your dependencies. In the next section, we'll give you some advice that should make the process easier.

## Advice for starting new projects

It can be a tedious process to do all the preparation work for packaging in one go. It's also easy to make a mistake like forgetting to list a required dependency if you try to write all your setup config just before publishing your package for the first time. It's much easier to start with very simple pyproject.toml and setup.cfg files, containing only the essential configuration and metadata. You can then add to your metadata and configuration as you work on your project. For example, every time you start using a new third-party project in your code, you can immediately add it to your install_requires list. It's also a good idea to start writing your README file early on and expanding it as you progress. You may even find that writing a paragraph or two describing your project helps you to think more clearly about what it is you are trying to achieve.

To help you, we have created what we think is a good initial skeleton for a new project. You can find it in the skeleton-project folder in this chapter's source code:
```bash

$ tree skeleton-project/
skeleton-project/
├── example
│   └── __init__.py
├── tests
│   └── __init__.py
├── README.md
├── pyproject.toml
├── setup.cfg
└── setup.py
```

Feel free to copy this, modify it as you wish, and use it as a starting point for your own projects.

## Alternative tools

Before we finish the chapter, let's have a quick look at some alternative options that you have for packaging your projects. Before PEP 517 and PEP 518, it was very difficult to use anything other than setuptools to build packages. There was no way for projects to specify what libraries were required to build them or how they should be built, so pip and other tools just assumed that packages should be built using setuptools.

Thanks to the build-system information in the pyproject.toml file, it is now easy to use any packaging library you want. There aren't that many alternatives yet, but there are a few that are worth mentioning:

•  The Flit project (https://flit.readthedocs.io/en/latest/index.html) was instrumental in inspiring the development of the PEP 517 and PEP 518 standards (the creator of Flit was a co-author of PEP 517). Flit aims to make packaging simple, pure Python projects that don't require complex build steps (like compiling C code) as easy as possible. Flit also provides a command-line interface for building packages and uploading them to PyPI (so you don't need the build tool or twine).

•  Poetry (https://python-poetry.org/) also provides both a command-line interface for building and publishing packages, and a lightweight PEP 517 build backend. Where Poetry really shines, though, is in its advanced dependency management features. Poetry can even manage virtual environments for you.

•  Enscons (https://github.com/dholth/enscons) is rather different from the other tools we've seen, in that it is based on the SCons (https://scons.org/) general-purpose build system. This means that unlike Flit or Poetry, Enscons can be used to build distributions that include C language extensions.

The tools we've discussed in this chapter are all focused on distributing packages through PyPI. Depending on your target audience, this might not always be the best choice, though. PyPI exists mainly for distributing projects such as libraries and development tools for use by Python developers. Installing and using packages from PyPI also requires having a working Python installation and at least enough Python knowledge to know how to install packages with pip.

If your project is an application aimed at a less technically adept audience, you may want to consider other options. The Python Packaging User Guide has a useful overview of various options for distributing applications, at https://packaging.python.org/overview/#packaging-applications.

## Further reading

That brings us to the end of our packaging journey. We will finish this chapter with a few links to resources where you can read more about packaging:

•  The Python Packaging Authority's Packaging History page (https://www. pypa.io/en/latest/history/) is a useful resource for understanding the evolution of Python packaging.

•  The Python Packaging User Guide (https://packaging.python.org/) has useful tutorials and guides, as well as a packaging glossary, links to packaging specifications, and a summary of various interesting projects related to packaging.
•  The setuptools documentation (https://setuptools.readthedocs.io/) contains a lot of useful information.

As you read these (and other packaging resources), it is worth bearing in mind that PEP 517 and PEP 518 were only finalized a few years ago and much of the available documentation still refers to older ways of doing things.

## Summary

In this chapter, we looked at how to package and distribute Python projects through the Python Package Index. We started with some theory about packaging and introduced the concepts of projects, releases, and distributions on PyPI.

We talked about setuptools, the most widely used packaging library for Python, and worked through the process of preparing a project for packaging with setuptools. In the process, we saw various files that need to be added to a project to package it and what each of them is for. We discussed the metadata that you should provide to describe your project and help users find it on PyPI, as well as how to add our code and data files to our distribution, how to specify our dependencies, and how to define entry points so that pip will automatically generate scripts for us. We also looked at the tools that Python gives us to query the distribution metadata from our code and how to access packaged data resources in our code.

We moved on to talk about how to build distribution packages and how to use twine to upload those packages to PyPI. We also gave you some advice on starting new projects. We concluded our tour of packaging by briefly talking about some alternatives to setuptools and pointing you to some resources where you can learn more about packaging.

We really do encourage you to start distributing your code on PyPI. No matter how trivial you think it might be, someone else somewhere in the world will probably find it useful. It really does feel good to contribute and give back to the community, and besides, it looks good on your CV.
