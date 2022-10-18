# GUIs and Scripting

    "A user interface is like a joke. If you have to explain it, it's not that good."

    – Martin LeBlanc

In this chapter, we're going to work on a project together. We are going to write a simple scraper that finds and saves images from a web page. We'll focus on three parts:

•  A simple HTTP web server in Python
•  A script that scrapes a given URL
•  A GUI application that scrapes a given URL

A graphical user interface (GUI) is a type of interface that allows the user to interact with an electronic device through graphical icons, buttons, and widgets, as opposed to text-based or command- line interfaces, which require commands or text to be typed on the keyboard. In a nutshell, any browser, any office suite such as LibreOffice, and, in general, anything that pops up when you click on an icon, is a GUI application.

So, if you haven't already done so, this would be the perfect time to start a console and position yourself in a folder called ch12 in the root of your project for this book. Within that folder, we'll create two Python modules (scrape.py and guiscrape.py) and a folder (simple_server). Within simple_server, we'll write our HTML page: index.html. Images will be stored in simple_server/img.

The structure in ch12 should look like this:
```bash

$ tree -A
.
├── guiscrape.py
├── scrape.py
└── simple_server
    ├── img
    │ ├── owl-alcohol.png
    │ ├── owl-book.png
    │ ├── owl-books.png
    │ ├── owl-ebook.jpg
    │ └── owl-rose.jpeg
    ├── index.html
    └── serve.sh


```

If you're using either Linux or macOS, you can put the code to start the HTTP server in a serve.sh file. On Windows, you'll probably want to use a batch file. The purpose of this file is just to give you an example, but for such a straightforward scenario, you can simply type its content into a console and you'll be fine.

The HTML page we're going to scrape has the following structure:
```html

# simple_server/index.html
<!DOCTYPE html>
<html lang="en">
  <head><title>Cool Owls!</title></head>
  <body>
    <h1>Welcome to our owl gallery</h1>
    <div>
      <img src="img/owl-alcohol.png" height="128" />
      <img src="img/owl-book.png" height="128" />
      <img src="img/owl-books.png" height="128" />
      <img src="img/owl-ebook.jpg" height="128" />
      <img src="img/owl-rose.jpeg" height="128" />
    </div>
    <p>Do you like these owls?</p>
  </body>
</html>
```

It's a simple page, so let's just note that we have five images, three of which are PNGs and two of which are JPGs (note that even though they are both JPGs, one ends with .jpg and the other with .jpeg, which are both valid extensions for this format).

Python gives you a rudimentary HTTP server that can serve static pages. You can start it with the following command (make sure you run it from the simple_server folder):
```bash

$ python -m http.server 8000
Serving HTTP on :: port 8000 (http://[::]:8000/) ...
::1 - - [04/Sep/2021 10:40:07] "GET / HTTP/1.1" 200 -
...

```

The last line is the log you get when you access http://localhost:8000, where the owl page will be served. Alternatively, you can put that command in a file called serve.sh, and just run it (make sure it's executable):
```bash

$ ./serve.sh

```

It will have the same effect. If you have the code for this book, your page should look something like the one depicted in Figure 12.1:


Figure 12.1: The owl page

If you wish to compare the source HTML with that in the page, you can right-click on the page and your browser should offer you an option like View Page Source (or something similar).

Feel free to use any other set of images, as long as you use at least one PNG and one JPG, and in the src tag, you use relative paths, not absolute ones. We got these lovely owls from https://openclipart.org/.

## First approach: scripting

Now, let's start writing the script. We'll go through the source in three steps: imports, argument parsing, and business logic.


### The imports

Here's how the script starts:
```python

# scrape.py
import argparse
import base64
import json
from pathlib import Path
from bs4 import BeautifulSoup
import requests

```

Going through the imports from the top, you can see that we'll need to parse the arguments, which we'll feed to the script itself (using argparse). We will need the base64 library to save the images within a JSON file (so we will also need json), and we'll need to open files for writing (using pathlib). Finally, we'll need BeautifulSoup for scraping the web page easily, and requests to fetch its content. We assume you're familiar with requests as we used it in Chapter 8, Files and Data Persistence.

We will explore the HTTP protocol and the requests mechanism in Chapter 14, Introduction to API Development, so for now, let's just (simplistically) say that we perform an HTTP request to fetch the content of a web page. We can do it programmatically using a library, such as requests, and it's more or less the equivalent of typing a URL in your browser and pressing Enter (the browser then fetches the content of a web page and displays it to you).

Of all these imports, only the last two don't belong to the Python standard library, so make sure you have them installed. You can check if that is the case by running the following command:
```python

$ pip freeze | egrep -i "soup4|requests"
beautifulsoup4==4.9.3
requests==2.26.0

```

The above command won't work on Windows. If that is your operating system, you can use the findstr command, instead of egrep. Alternatively, simply type $ pip freeze, and skim through to get the desired versions.

The above dependencies are listed in the requirements.txt file for this chapter, of course. At this point, the only thing that might be a bit confusing is the base64/json couple we are using, so allow us to spend a few words on that.

As we saw in the previous chapter, JSON is one of the most popular formats for data exchange between applications. It's widely used for other purposes too, for example, to save data in a file. In our script, we're going to offer the user the ability to save images as image files, or as a single JSON file. Within the JSON, we'll put a dictionary with keys as the image names and values as their content. The only issue is that saving images in the binary format is tricky, and this is where the base64 library comes to the rescue.

The base64 library is quite useful. For example, every time you send an email with an image attached to it, the image gets Base64-encoded before the email is sent. On the recipient end, images are automatically decoded into their original binary format so that the email client can display them.

We used Base64 in Chapter 9, Cryptography and Tokens. If you skipped it, this would be a good time to check it out.

### Parsing arguments

Now that the technicalities are out of the way, let's see the next section of our script, argument parsing, which should be at the end of the scrape.py module:
```python

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description='Scrape a webpage.')
    parser.add_argument(
        '-t',
        '--type',
        choices=['all', 'png', 'jpg'],
        default='all',
        help='The image type we want to scrape.')
    parser.add_argument(
        '-f',
        '--format',
        choices=['img', 'json'],
        default='img',
        help='The format images are saved to.')
    parser.add_argument(
        'url',
        help='The URL we want to scrape for images.')
    args = parser.parse_args()
    scrape(args.url, args.format, args.type)

```

Look at that first line; it is a very common idiom when it comes to scripting. According to the official Python documentation, the '__main__' string is the name of the scope in which top-level code executes. A module's __name__ attribute is set to '__main__' when read from standard input, a script, or from an interactive prompt.

Therefore, if you put the execution logic under that if, it will be run only when you run the script directly, as its __name__ will be '__main__' in that case. On the other hand, should you import objects from this module, then its name will be set to something else, so the logic under the if won't run.

The first thing we do is define our parser. There are several useful libraries out there nowadays to write parsers for arguments, like Docopt (https://github.com/docopt/ docopt) and Click (https://click.palletsprojects.com/), but in this case, we prefer to use the standard library module, argparse, which is simple enough and quite powerful.

We want to feed our script three different pieces of data: the types of images we want to save, the format in which we want to save them, and the URL of the page to be scraped.

The types can be PNGs, JPGs, or both (the default option), while the format can be either image or JSON, image being the default. URL is the only mandatory argument.

So, we add the -t option, allowing also the long version, --type. The choices are 'all', 'png', and 'jpg'. We set the default to 'all' and we add a help message.

We do a similar procedure for the format argument, allowing both the short and long syntax (-f and --format), and finally, we add the url argument, which is the only one that is specified differently so that it won't be treated as an option, but rather as a positional argument.

In order to parse all the arguments, all we need is parser.parse_args(). Simple, isn't it?

The last line is where we trigger the actual logic by calling the scrape() function, passing all the arguments we just parsed. We will see its definition shortly. The nice thing about argparse is that if you call the script by passing -h, it will print some nice usage text for you automatically. Let's try it out:

```bash

$ python scrape.py -h
usage: scrape.py [-h] [-t {all,png,jpg}] [-f {img,json}] url

Scrape a webpage.

positional arguments:
  url                   The URL we want to scrape for images.

optional arguments:
  -h, --help            show this help message and exit
  -t {all,png,jpg}, --type {all,png,jpg}
                        The image type we want to scrape.
  -f {img,json}, --format {img,json}
                        The format images are saved to.

```

If you think about it, the one true advantage of this is that we just need to specify the arguments and we don't have to worry about the usage text, which means we won't have to keep it in sync with the arguments' definitions every time we change something. This translates to time saved.

Here are a few different ways to call the scrape.py script, which demonstrate that type and format are optional, and how you can use the short and long syntaxes to employ them:
```bash

$ python scrape.py http://localhost:8000
$ python scrape.py -t png http://localhost:8000
$ python scrape.py --type=jpg -f json http://localhost:8000

```

The first one uses default values for type and format. The second one will save only PNG images, and the third one will save only JPGs, but in JSON format.

### The business logic

Now that we've seen the scaffolding, let's dive deep into the actual logic (if it looks intimidating, don't worry; we'll go through it together). 

Within the script, this logic lies after the imports and before the parsing (before the if __name__ clause):
```python

def scrape(url, format_, type_):
    try:
        page = requests.get(url)
    except requests.RequestException as err:
        print(str(err))
    else:
        soup = BeautifulSoup(page.content, 'html.parser')
        images = fetch_images(soup, url)
        images = filter_images(images, type_)
        save(images, format_)

```

Let's start with the scrape() function. The first thing it does is fetch the page at the given url argument. Whatever error may happen while doing this, we trap it in RequestException (err) and print it. RequestException is the base exception class for all the exceptions in the requests library. However, if things go well, and we have a page back from the GET request, then we can proceed (else branch) and feed its content to the BeautifulSoup parser. The BeautifulSoup library allows us to parse a web page in no time, without having to write all the logic that would be needed to find all the images in a page, which we really don't want to do. It's not as easy as it seems, and reinventing the wheel is never good. To fetch images, we use the fetch_images() function and we filter them with filter_images(). Finally, we call save with the result.

Splitting the code into different functions with meaningful names allows us to read it more easily. Even if you haven't seen the logic of the fetch_images(), filter_images(), and save() functions, it's not hard to predict what they do, right? 

Check out the following:
```python

def fetch_images(soup, base_url):
    images = []
    for img in soup.findAll('img'):
        src = img.get('src')
        img_url = f'{base_url}/{src}'
        name = img_url.split('/')[-1]
        images.append(dict(name=name, url=img_url))
    return images

```

The Fix spacing fetch_images() function takes a BeautifulSoup object and a base URL. All it does is loop through all of the images found on the page and fill in the name and url information about them in a dictionary (one per image). All dictionaries are added to the images list, which is returned at the end.

There is some trickery going on when we get the name of an image. We split the img_url (for example, http://localhost:8000/img/my_image_name.png) string using '/' as a separator, and we take the last item as the image name. There is a more robust way of doing this, but for this example it would be overkill. If you want to see the details of each step, try to break this logic down into smaller steps, and print the result of each of them to help yourself understand. You can read about more efficient ways of doing this in Chapter 11, Debugging and Profiling. For example, if we add print(images) at the end of the fetch_images() function, we 
get this:
```python

[{'url': 'http://localhost:8000/img/owl-alcohol.png', 'name': 'owl-
alcohol.png'}, {'url': 'http://localhost:8000/img/owl-book.png', 
'name': 'owl-book.png'}, ...]

```

We truncated the result for brevity. You can see each dictionary has a url and name key/value pair, which we can use to fetch, identify, and save our images as we like. At this point, something interesting to consider is: what would happen if the images on the page were specified with an absolute path instead of a relative one? The answer is that the script would fail to download them because this logic expects relative paths.

We hope you find the body of the filter_images() function below interesting. We wanted to show you how to check through multiple extensions using a mapping technique:
````python

def filter_images(images, type_):
    if type_ == 'all':
        return images
    ext_map = {
        'png': ['.png'],
        'jpg': ['.jpg', '.jpeg'],
    }
    return [
        img for img in images
        if matches_extension(img['name'], ext_map[type_])
    ]

def matches_extension(filename, extension_list):
    extension = Path(filename.lower()).suffix
    return extension in extension_list

```

In this function, if type_ is all, then no filtering is required, so we just return all the images. On the other hand, when type_ is not all, we get the allowed extensions from the ext_map dictionary, and use it to filter the images in the list comprehension that ends the function body. You can see that by using another helper function, matches_extension(), we have made the list comprehension simpler and more readable.

All that matches_extension() does is get the extension from the image name and check whether it is within the list of allowed ones.

In case you're wondering why we have collected all the images in the images list and then filtered out some of them, instead of checking whether we wanted to save them before adding them to the list, there are three reasons for that. The first reason is that we will need fetch_images() in the GUI application as it is now. The second reason is that combining fetching and filtering would produce a longer and more complicated function, and we are trying to keep the complexity level down. The third reason is that this could be a nice exercise for you to do.

Let's keep going through the code and inspect the save() function:
```python

def save(images, format_):
    if images:
        if format_ == 'img':
            save_images(images)
        else:
            save_json(images)
        print('Done')
    else:
        print('No images to save.')

def save_images(images):
    for img in images:
        img_data = requests.get(img['url']).content
        with open(img['name'], 'wb') as f:
            f.write(img_data)

def save_json(images):
    data = {}
    for img in images:
        img_data = requests.get(img['url']).content
        b64_img_data = base64.b64encode(img_data)
        str_img_data = b64_img_data.decode('utf-8')
        data[img['name']] = str_img_data
    with open('images.json', 'w') as ijson:
        ijson.write(json.dumps(data))

```

You can see that, when images isn't empty, this acts as a dispatcher. We either call save_images() or save_json(), depending on what information is stored in the format_ variable.

We are almost done. Let's jump to save_images(). We loop over the images list and for each dictionary we find in it, we perform a GET request on the image URL and save its content in a file, which we name as the image itself.

Finally, let's now step into the save_json() function. It's very similar to the previous one. We basically fill in the data dictionary. The image name is the key, and the Base64 representation of its binary content is the value. When we're done populating our dictionary, we use the json library to dump it in the images.json file. Here's a small preview of that:
```python

# images.json (truncated)
{
  "owl-alcohol.png": "iVBORw0KGgoAAAANSUhEUgAAASwAAAEICA...
  "owl-book.png": "iVBORw0KGgoAAAANSUhEUgAAASwAAAEbCAYAA...
  "owl-books.png": "iVBORw0KGgoAAAANSUhEUgAAASwAAAElCAYA...
  "owl-ebook.jpg": "/9j/4AAQSkZJRgABAQEAMQAxAAD/2wBDAAEB...
  "owl-rose.jpeg": "/9j/4AAQSkZJRgABAQEANAA0AAD/2wBDAAEB...
}


```

And that's it! Now, before proceeding to the next section, make sure you play with this script and understand how it works. Try to modify something, print out intermediate results, add a new argument or functionality, or scramble the logic. We're going to migrate it into a GUI application now, which will add a layer of complexity simply because we'll have to build the GUI interface, so it's important that you're well acquainted with the business logic—it will allow you to concentrate on the rest of the code.

## Second approach: a GUI application

There are several libraries for writing GUI applications in Python. Some of the most famous ones are Tkinter, wxPython, Kivy, and PyQt. They all offer a wide range of tools and widgets that you can use to compose a GUI application. The one we are going to use in this chapter is Tkinter. Tkinter stands for Tk interface and it is the standard Python interface to the Tk GUI toolkit. Both Tk and Tkinter are available on most Unix platforms, macOS X, as well as on Windows systems.

Let's make sure that tkinter is installed properly on your system by running this command:
```bash

$ python -m tkinter

```

It should open a dialog window, demonstrating a simple Tk interface. If you can see that, we're good to go. However, if it doesn't work, please search for tkinter in the Python official documentation (https://docs.python.org/3.9/library/tkinter.html). You will find several links to resources that will help you get up and running with it.

We're going to make a very simple GUI application that basically mimics the behavior of the script we saw in the first part of this chapter. We won't add the ability to filter by image type, but after you've gone through this chapter, you should be able to play with the code and put that feature back in by yourself.

So, this is what we're aiming for:

Figure 12.2: Main window of the Scrape app

Gorgeous, isn't it? As you can see, it's a very simple interface (Figure 12.2 shows how it should look on a Mac). There is a frame (that is, a container) for the URL field and the Fetch info button, another frame for the Listbox (Content) to hold the image names and the radio buttons to control the way we save them, and there is a Scrape! button at the bottom right. There is also a status bar at the bottom, which shows us some information.

In order to get this layout, we could just place all the widgets on a root window, but that would make the layout logic quite messy and unnecessarily complicated. So, instead, we will divide the space up using frames and place the widgets in those frames. This way we will achieve a much nicer result. So, this is the draft for the layout:

Figure 12.3: Scrape app diagram

We have a Root Window, which is the main window of the application. We divide it into two rows, the first one in which we place the Main Frame, and the second one in which we place the Status Frame (which will hold the status bar text). The Main Frame is subsequently divided into three rows. In the first one, we place the URL Frame, which holds the URL widgets. In the second one, we place the Img Frame, which will hold the Listbox and the Radio Frame, which will host a label and the radio button widgets. And finally, we have the third one, which will just hold the Scrape button. In order to lay out frames and widgets, we will use a layout manager called grid, 
which simply divides up the space into rows and columns, as in a matrix.

Now, all the code we are going to write comes from the guiscrape.py module, so we won't repeat its name for each snippet, to save space. The module is logically divided into three sections, not unlike the script version: imports, layout logic, and business logic. We're going to analyze them line by line, in three chunks.

### The imports

Imports are like in the script version, except we've lost argparse, which is no longer needed, and we have added two lines:
```python

# guiscrape.py
from tkinter import * 
from tkinter import ttk, filedialog, messagebox 
...

```

The first line is quite common practice when using tkinter, although in general it is bad practice to import using the * syntax. Doing so may result in name collisions, which result in shadowing issues and, if the imported module is very big, importing everything can be expensive.

On the second line, we import ttk, filedialog, and messagebox explicitly, following the conventional approach used with this library. ttk is Tkinter's new set of styled widgets. They behave basically like the old ones but are capable of drawing themselves correctly according to the style your OS is set on, which is great.

The rest of the imports (omitted) are what we need in order to carry out the task you know well by now. Note that there is nothing extra we need to install with pip in this second part; if you have installed the requirements for the chapter, you already have everything you need.

### The layout logic

We are going to present the layout logic chunk by chunk so that we can explain it easily to you. You'll see how all those pieces we talked about in the layout draft are arranged and glued together. We will first show you, as in the script before, the final part of the guiscrape.py module. We'll leave the middle part, the business logic, for last:
```python

if __name__ == "__main__":
    _root = Tk()
    _root.title('Scrape app')


```

As you know by now, we only want to execute the logic when the module is run directly, so that first line shouldn't surprise you.

In the last two lines, we set up the main window, which is an instance of the Tk class. We instantiate it and give it a title. 

Note that we use the prepending underscore technique for all the names of the tkinter objects, in order to avoid potential collisions with names in the business logic. It's possibly not the most eye-pleasing solution, but it gets the job done.
```python

    _mainframe = ttk.Frame(_root, padding='5 5 5 5')
    _mainframe.grid(row=0, column=0, sticky=(E, W, N, S))

```

Here, we set up the Main Frame. It's a ttk.Frame instance. We set _root as its parent, and give it some padding. The padding is a measure in pixels of how much space should be inserted between the inner content and the borders in order to let the layout breathe a little. Little or no padding at all produces a sardine effect, where it feels like widgets are packed too tightly.

The second line is more interesting. We place this _mainframe on the first row (0) and first column (0) of the parent object (_root). We also say that this frame needs to extend itself in each direction by using the sticky argument with all four cardinal directions. If you're wondering where they came from, it's the from tkinter import * magic that brought them to us.
```python

    _url_frame = ttk.LabelFrame(
        _mainframe, text='URL', padding='5 5 5 5')
    _url_frame.grid(row=0, column=0, sticky=(E, W))
    _url_frame.columnconfigure(0, weight=1)
    _url_frame.rowconfigure(0, weight=1)


```

Next, we place the URL Frame. This time, the parent object is _mainframe, as you will recall from the draft in Figure 12.3. This is not just a simple Frame; it's actually a LabelFrame, which means we can set the text argument and expect a rectangle to be drawn around it, with the content of the text argument written in the top-left part of it (check out Figure 12.2 if it helps). We position this frame at (0, 0), and say that it should expand to the left (W) and to the right (E). We don't need it to expand in either of the other two directions. Finally, we use rowconfigure() and columnconfigure() to make sure it behaves correctly when resizing. This is just a formality in our present layout.

    _url = StringVar()
    _url.set('http://localhost:8000')
    _url_entry = ttk.Entry(
        _url_frame, width=40, textvariable=_url)
    _url_entry.grid(row=0, column=0, sticky=(E, W, S, N), padx=5)
    _fetch_btn = ttk.Button(
        _url_frame, text='Fetch info', command=fetch_url)
    _fetch_btn.grid(row=0, column=1, sticky=W, padx=5)

Here, we have the code to lay out the URL textbox and the _fetch button. A textbox in the Tkinter environment is called Entry. We instantiate it as usual, setting _url_frame as its parent and giving it a width in pixels. Also, and this is the most interesting part, we set the textvariable argument to be _url. _url is a StringVar, which is an object that is now connected to Entry and will be used to manipulate its content. This means we will not modify the text in the _url_entry instance directly, but rather by accessing _url. In this case, we call the set() method on it to set the initial value to the URL of our local web page.

We position _url_entry at (0, 0), setting all four cardinal directions for it to stick to, and we also set a bit of extra padding on the left and right edges using padx, which adds padding on the x-axis (horizontal). On the other hand, pady takes care of the vertical direction.

By now, it should be clear that every time we call the .grid() method on an object, we're basically telling the grid layout manager to place that object somewhere, according to rules that we specify as arguments to the grid() call.

Similarly, we set up and place the _fetch button. The only interesting parameter is command=fetch_url. This means that when we click this button, we call the  fetch_url() function. This technique is called callback.
```python

    _img_frame = ttk.LabelFrame(
        _mainframe, text='Content', padding='9 0 0 0')
    _img_frame.grid(row=1, column=0, sticky=(N, S, E, W))

```

This is what we called Img Frame in the layout draft. It is placed on the second row of its parent _mainframe. It will hold the Listbox and the Radio Frame:
```python

    _images = StringVar()
    _img_listbox = Listbox(
        _img_frame, listvariable=_images, height=6, width=25)
    _img_listbox.grid(row=0, column=0, sticky=(E, W), pady=5)
    _scrollbar = ttk.Scrollbar(
        _img_frame, orient=VERTICAL, command=_img_listbox.yview)
    _scrollbar.grid(row=0, column=1, sticky=(S, N), pady=6)
    _img_listbox.configure(yscrollcommand=_scrollbar.set)

```

This is probably the most interesting bit of the whole layout logic. As we did with _url_entry, we need to drive the contents of Listbox by tying it to an _images variable. We set up Listbox so that _img_frame is its parent, and _images is the variable it is tied to. We also pass some dimensions.

The interesting bit comes from the _scrollbar instance. We pass orient=VERTICAL to set its orientation. In order to tie its position to the vertical scroll of Listbox, when we instantiate it, we set its command to _img_listbox.yview. This is the first half of the contract we are creating between Listbox and Scrollbar. The other half is provided by the _img_ listbox.configure() method, which sets yscrollcommand=_scrollbar.set.

By setting up this reciprocal bond, when we scroll on Listbox, Scrollbar will move accordingly and, vice versa, when we operate Scrollbar, Listbox will scroll accordingly.
```python

    _radio_frame = ttk.Frame(_img_frame)
    _radio_frame.grid(row=0, column=2, sticky=(N, S, W, E))

```

We place the Radio Frame, ready to be populated. Note that Listbox is occupying (0, 0) on _img_frame, Scrollbar takes up (0, 1), and therefore _radio_frame will go to (0, 2). Let's populate it:
```python

    _choice_lbl = ttk.Label(
        _radio_frame, text="Choose how to save images")
    _choice_lbl.grid(row=0, column=0, padx=5, pady=5)
    _save_method = StringVar()
    _save_method.set('img')
    _img_only_radio = ttk.Radiobutton(
        _radio_frame, text='As Images', variable=_save_method,
        value='img')
    _img_only_radio.grid(
        row=1, column=0, padx=5, pady=2, sticky=W)
    _img_only_radio.configure(state='normal')
    _json_radio = ttk.Radiobutton(
        _radio_frame, text='As JSON', variable=_save_method,
        value='json')
    _json_radio.grid(row=2, column=0, padx=5, pady=2, sticky=W)

```

Firstly, we place the label, and we give it some padding. Note that the label and radio buttons are children of _radio_frame.

As for the Entry and Listbox objects, Radiobutton is also driven by a bond to an external variable, which we called _save_method. Each Radiobutton instance sets up a value argument, and by checking the value on _save_method, we know which button is selected.
```python

    _scrape_btn = ttk.Button(
        _mainframe, text='Scrape!', command=save)
    _scrape_btn.grid(row=2, column=0, sticky=E, pady=5)

```

On the third row of _mainframe, we place the Scrape! button. Its command is save, which saves the images to be listed in Listbox, after we have successfully parsed a web page.

```python
    _status_frame = ttk.Frame(
        _root, relief='sunken', padding='2 2 2 2')
    _status_frame.grid(row=1, column=0, sticky=(E, W, S))
    _status_msg = StringVar()
    _status_msg.set('Type a URL to start scraping...')
    _status = ttk.Label(
        _status_frame, textvariable=_status_msg, anchor=W)
    _status.grid(row=0, column=0, sticky=(E, W))

```

We end the layout section by placing the status frame, which is a simple ttk.Frame. To give it a little status bar effect, we set its relief property to 'sunken' and give it a uniform padding of two pixels. It needs to stick to the left, right, and bottom parts of the _root window, so we set its sticky attribute to (E, W, S).

We then place a label in it and, this time, we tie it to a StringVar object, because we will have to modify it every time we want to update the status bar text. You should be acquainted with this technique by now.

Finally, on the last line, we run the application by calling the mainloop() method on the Tk instance:
```python

    _root.mainloop()

```

Please remember that all these instructions are placed under the if __name__ == "__main__": clause in the original script.

As you can see, the code to design our GUI application is not hard. Granted, at the beginning you have to play around a little bit. Not everything will work out perfectly on the first attempt, but it's easy to find helpful tutorials on the web. Let's now get to the interesting bit, the business logic.

### The business logic

We'll analyze the business logic of the GUI application in three chunks. There is the fetching logic, the saving logic, and the alerting logic.

#### Fetching the web page

Let's start with the code to fetch the page and images:
```python

config = {}

def fetch_url():
    url = _url.get()
    config['images'] = []
    _images.set(())  # initialized as an empty tuple
    try:
        page = requests.get(url)
    except requests.RequestException as err:
        sb(str(err))
    else:
        soup = BeautifulSoup(page.content, 'html.parser')
        images = fetch_images(soup, url)
        if images:
            _images.set(tuple(img['name'] for img in images))
            sb('Images found: {}'.format(len(images)))
        else:
            sb('No images found')
        config['images'] = images

def fetch_images(soup, base_url):
    images = []
    for img in soup.findAll('img'):
        src = img.get('src')
        img_url = f'{base_url}/{src}'
        name = img_url.split('/')[-1]
        images.append(dict(name=name, url=img_url))
    return images

```

First of all, let's discuss that config dictionary. We need some way of passing data between the GUI application and the business logic. Now, instead of polluting the global namespace with many different variables, one simple technique is to have a single dictionary that holds all the objects we need to pass back and forth.

In this simple example, we'll just populate the config dictionary with the images we fetch from the page, but we wanted to show you the technique so that you have at least one example. 

This technique comes from our experience with JavaScript. When you code a web page, you often import several different libraries. If each of these cluttered the global namespace with all sorts of variables, there might be issues in making everything work, because of name clashes and variable overriding.

In our case, we find that using one config variable is a good solution to this issue.

The fetch_url() function is quite similar to what we did in the script. First, we get the url value by calling _url.get(). Remember that the _url object is a StringVar instance that is tied to the _url_entry object, which is an instance of Entry. The text field you see on the GUI is the Entry object, but the text behind the scenes is the value of the StringVar object.

By calling get() on _url, we get the value of the text that is displayed in _url_entry.

The next step is to prepare config['images'] to be an empty list, and to empty the _images variable, which is tied to _img_listbox. This, of course, has the effect of cleaning up all the items in _img_listbox.

After this preparation step, we can attempt to fetch the page, using the same try/except logic we adopted in the script at the beginning of the chapter. The one difference is the action we take if things go wrong. Within the GUI application, we call sb(str(err)). We will see the code for the sb() helper function shortly. Basically, it sets the text in the status bar for us. Once you know that sb stands for "status bar," it makes sense, right? However, we would argue this is not a good name. We had to explain its behavior to you, which means it isn't self-explanatory. We left this as an example of how easy it can be to write poor-quality code that only makes total sense once our head is wrapped around it, hence making it difficult to spot.

If we can fetch the page, then we create the soup instance and fetch the images from it. The logic of fetch_images() is exactly the same as the one explained before, so we won't repeat it here.

If we have images, using a quick tuple comprehension (which is actually a generator expression fed to a tuple constructor) we feed the _images as StringVar and this has the effect of populating our _img_listbox with all the image names. Finally, we update the status bar.

If there were no images, we would still update the status bar. At the end of the function, regardless of how many images were found, we update config['images'] to hold the images list. In this way, we'll be able to access the images from other functions by inspecting config['images'] without having to pass that list around.

#### Saving the images

The logic to save the images is pretty straightforward. Here it is:

```python
def save():
    if not config.get('images'):
        alert('No images to save')
        return
    if _save_method.get() == 'img':
        dirname = filedialog.askdirectory(mustexist=True)
        save_images(dirname)
    else:
        filename = filedialog.asksaveasfilename(
            initialfile='images.json',
            filetypes=[('JSON', '.json')])
        save_json(filename)

def save_images(dirname):
    if dirname and config.get('images'):
        for img in config['images']:
            img_data = requests.get(img['url']).content
            filename = Path(dirname).joinpath(img['name'])
            with open(filename, 'wb') as f:
                f.write(img_data)
        alert('Done')

def save_json(filename):
    if filename and config.get('images'):
        data = {}
        for img in config['images']:
            img_data = requests.get(img['url']).content
            b64_img_data = base64.b64encode(img_data)
            str_img_data = b64_img_data.decode('utf-8')
            data[img['name']] = str_img_data
        with open(filename, 'w') as ijson:
            ijson.write(json.dumps(data))
        alert('Done')

```

When the user clicks the Scrape! button, the save function is called using the callback mechanism.

The first thing that this function does is check whether there are actually any images to be saved. If not, it alerts the user about it, using another helper function, alert(), whose code we'll see shortly. No further action is performed if there are no images.

On the other hand, if the config['images'] list is not empty, save() acts as a dispatcher, and it calls either save_images() or save_json(), according to which value is held by _save_method. Remember, this variable is tied to the radio buttons, therefore we expect its value to be either 'img' or 'json'.

This dispatcher is a bit different from the one in the script; there are some additional steps that must be taken before we dispatch to either save_images() or save_json().

If we want to save the images to image format, we need to ask the user to choose a directory. We do this by calling filedialog.askdirectory and assigning the result of the call to the dirname variable. This opens up a dialog window that asks us to choose a directory. The directory we choose must exist, as specified by the way we call the method. This is done so that we don't have to write code to deal with a potentially missing directory when saving the files.

Here's how this dialog should look on a Mac:

If we cancel the operation, dirname is set to None.

Figure 12.4: Save dialog for image format

Before we finish analyzing the logic in save, let's quickly go through save_images().

It's very similar to the version we had in the script so just note that, at the beginning, in order to be sure that we actually have something to do, we check on both dirname and the presence of at least one image in config['images'].

If that's the case, it means we have at least one image to save and the path for it, so we can proceed. The logic to save the images has already been explained. The one thing we do differently this time is joining the directory (which means the complete path) to the image name, by means of the Path.joinpath() method.

At the end of save_images(), if we saved at least one image, we alert the user that we're done.

Let's now go back to the other logic branch in save. This branch is executed when the user selects the As JSON radio button before clicking the Scrape! button. In this case, we want to save the data to a JSON file and want to give the user the ability to choose a filename as well. Hence, we fire up a different dialog: filedialog.asksaveasfilename.

We pass an initial filename as a suggestion to the user, who can choose to change it if they don't like it. Moreover, because we're saving a JSON file, we're forcing the user to use the correct extension by passing the filetypes argument. It is a list, with any number of two-tuples (description, extension), that runs the logic of the dialog.

Here's how this dialog should look on macOS:

Once we have chosen a place and a filename, we can proceed with the saving logic, which is the same as it was in the script version of the app. We create a JSON object from a Python dictionary (data) that we have populated with key/value pairs made by the image's name and Base64-encoded content.

In save_json() as well, we have a little check at the beginning that makes sure that we don't proceed unless we have a filename and at least one image to save. This ensures that if the user presses the Cancel button, nothing happens.

#### Alerting the user

Finally, let's see the alerting logic. It's extremely simple:
```python

def sb(msg):
    _status_msg.set(msg)

def alert(msg):
    messagebox.showinfo(message=msg)

```

That's it! To change the status bar message, all we need to do is to access the  _status_msg StringVar, as it's tied to the _status label.

On the other hand, if we want to show the user a more visible message, we can fire up a message box. Here's how it should look on a Mac:


Figure 12.6: Messagebox alert example

The messagebox object can also be used to warn the user (messagebox.showwarning) or to signal an error (messagebox.showerror). It can also be used to provide dialogs that ask the user whether they are sure they want to proceed or if they really want to delete a certain file, and so on.

If we inspect messagebox by simply printing out what dir(messagebox) returns, we find methods such as askokcancel(), askquestion(), askretrycancel(), askyesno(), and askyesnocancel(), as well as a set of constants to verify the response of the user, such as CANCEL, NO, OK, OKCANCEL, YES, and YESNOCANCEL. You can use these constants by comparing them to the user's choice so that you know the desired action to execute when the dialog closes.

Now that we have explored the code for this application, we can give it a spin with the following command:
```python

$ python guiscrape.py

```

###  How can we improve the application?

Now that you're accustomed to the fundamentals of designing a GUI application, we would like to offer some suggestions on how to make ours better.

Code Quality: We can start with the code quality. Do you think this code is good enough, or would you improve it? If so, how? We would test it, and make sure it's robust and caters to all the various scenarios that a user might create by clicking around on the application. We would also make sure the behavior is what we would expect when the website we're scraping is down or unreachable for any reason.

Naming: Another thing that we could improve is the names we chose. We have prudently named all the components with a leading underscore, both to highlight their somewhat private nature, and to avoid having name clashes with the underlying objects they are linked to. But in retrospect, many of those components could use a better name, so it's really up to you to refactor until you find the form that suits you best. You could start by giving a better name to the sb() function!

UI: For what concerns the user interface, you could try to resize the main application. See what happens? The whole content stays exactly where it is. Empty space is added if you expand, or the whole widget set disappears gradually if you shrink. This behavior isn't acceptable, therefore one quick solution could be to make the root window fixed (that is, unable to be resized).

Functionality: Another thing that you could do to improve the application is to add the same functionality we had in the script, to save only PNGs or JPGs. In order to do this, you could place a combo box somewhere, with three values: All, PNGs, JPGs, or something similar. 

The user should be able to select one of those options before saving the files.

Even better, you could change the setup of Listbox so that it's possible to select multiple images at the same time, and only the selected ones will be saved. If you manage to do this (it's not as hard as it seems), then you should consider presenting the Listbox a bit better, maybe providing alternating background colors for the rows.

Another nice thing you could add is a button that opens up a dialog to select a file. The file must be one of the JSON files the application can produce. Once selected, you could run some logic to reconstruct the images from their Base64-encoded version. The logic to do this is very simple, so here's an example:
```python

with open('images.json', 'r') as f:
    data = json.loads(f.read())

for (name, b64val) in data.items():
    with open(name, 'wb') as f:
        f.write(base64.b64decode(b64val))

```

As you can see, we need to open images.json in read mode and grab the data dictionary. Once we have it, we can loop through its items, and save each image with the Base64-decoded content. We leave it up to you to tie this logic to a button in the application.

Another cool feature that you could add is the ability to open up a preview pane that shows any image you select from Listbox, so that the user can take a peek at the images before deciding to save them.

Another suggestion would be to add a menu. Maybe even a simple menu with File and ? to provide the usual Help or About sections. Adding menus is not that complicated; you can add text, keyboard shortcuts, images, and so on.

In terms of business logic, it would be worth experimenting with different ways to store the data that currently is stored in the config dict. One alternative would be to use a dedicated object. You will find that being familiar with different ways to do this enables you to choose the best one for the situation at hand.

### Where do we go from here?

If you are interested in digging deeper into the world of GUIs, then we would like to offer the following suggestions.

#### The turtle module

The turtle module is an extended reimplementation of the eponymous module from the Python standard distribution up to version Python 2.5. It's a very popular way to introduce children to programming.

It's based on the idea of an imaginary turtle starting at the center of a Cartesian plane. You can programmatically command the turtle to move forward and backward, rotate, and so on; by combining all the possible moves, all sorts of intricate shapes and images can be drawn.

It's definitely worth checking out, if only to see something different.

#### wxPython, Kivy, and PyQt

After you have explored the vastness of the tkinter realm, we would suggest you explore other GUI libraries: wxPython (https://www.wxpython.org/), PyQt (https:// www.riverbankcomputing.com/software/pyqt/), and Kivy (https://kivy.org/). You may find that one of these works better for you, or makes it easier for you to code the application you need.

We believe that programmers can reify their ideas much better when they are conscious of what tools they have available. If your toolset is too narrow, your ideas may seem impossible or extremely hard to transform into reality, and they risk remaining exactly what they are, just ideas.

Of course, the technological spectrum today is humongous, so knowing everything is not possible; therefore, when you are about to learn a new technology or a new subject, our suggestion is to grow your knowledge by exploring breadth first.

Investigate several things, and then go deeper with the one or the few that looked the most promising. This way you'll be able to be productive with at least one tool, and when the tool no longer fits your needs, you'll know where to look, thanks to your previous exploration.

#### The principle of least astonishment

When designing an interface, there are many different things to bear in mind. One of them, which to us feels important, is the law or principle of least astonishment. It states that if in your design a necessary feature has a high astonishment factor, it may be necessary to redesign your application. 

To give you one example, when you're used to working with Windows, where the buttons to minimize, maximize, and close an application window are in the top- right corner, it's quite hard to work on a Mac, where those buttons are in the top-left corner. You'll find yourself constantly going to the top-right corner only to discover once more that the buttons are on the other side.

If a certain button has become so important in applications that it's now placed in a precise location by designers, please don't innovate. Just follow the convention. Users will only become frustrated when they have to waste time looking for a button that is not where it's supposed to be.

#### Threading considerations

This topic is outside the scope of this book, but we do want to mention it.

If you are coding a GUI application that needs to perform a long-running operation when a button is clicked, you will see that your application will probably freeze until the operation has been carried out. In order to avoid this, and maintain the application's responsiveness, you may need to run that time-expensive operation in a different thread (or even a different process) so that the OS will be able to dedicate a little bit of time to the GUI every now and then, to keep it responsive.

Gain a good grasp of the fundamentals first, and then have fun exploring them!

## Summary

In this chapter, we worked on a project together. We have written a script that scrapes a very simple web page and accepts optional commands that alter its behavior in doing so. We also coded a GUI application to do the same thing by clicking buttons instead of typing on a console. We hope you enjoyed reading it and following along as much as we enjoyed writing it.

We saw many different concepts, such as working with files and performing HTTP requests, and we talked about guidelines for usability and design.

We have only been able to scratch the surface, but hopefully you have a good starting point from which to expand your exploration. Throughout the chapter, we have pointed out several different ways in which you could improve the application, and we have challenged you with a few exercises and questions. We hope you have taken the time to play with those ideas. You can learn a lot just by playing around with fun applications like the one we've coded together.

In the next chapter, we're going to talk about data science, or at least about the tools that a Python programmer has when it comes to facing this subject.
