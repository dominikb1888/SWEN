# Data Science in Brief

    "If we have data, let's look at data. If all we have are opinions, let's go with mine."

    — Jim Barksdale, former Netscape CEO

Data science is a very broad term and can assume several different meanings depending on context, understanding, tools, and so on. In order to do proper data science, you need to, at the very least, know mathematics and statistics. Then, you may want to dig into other subjects, such as pattern recognition and machine learning, and, of course, there is a plethora of languages and tools you can choose from.

We won't be able to talk about everything here. Therefore, in order to render this chapter meaningful, we're going to work on a project together instead.

Around 2012/2013, Fabrizio was working for a top-tier social media company in London. He stayed there for two years and was privileged to work with several very brilliant people. The company was the first in the world to have access to the Twitter Ads API, and they were partners with Facebook as well. That means a lot of data.

Their analysts were dealing with a huge number of campaigns and they were struggling with the amount of work they had to do, so the development team Fabrizio was a part of tried to help by introducing them to Python and to the tools Python gives us to deal with data. It was a very interesting journey that led him to mentor several people in the company, eventually taking him to Manila where he gave a two-week intensive training course in Python and data science to the analysts over there.

The project we're going to do in this chapter is a lightweight version of the final example Fabrizio presented to his students in Manila. We have rewritten it to a size that will fit this chapter and made a few adjustments here and there for teaching purposes, but all the main concepts are there, so it should be fun and instructional for you.

Specifically, we are going to explore the following:

- The Jupyter Notebook and JupyterLab
- pandas and numpy: the main libraries for data science in Python
- A few concepts around Pandas's DataFrame class
- Creating and manipulating a dataset

Let's start by talking about Roman gods.

## IPython and Jupyter Notebook

In 2001, Fernando Perez was a graduate student in physics at CU Boulder, and was trying to improve the Python shell so that he could have the niceties he was used to when working with tools such as Mathematica and Maple. The result of these efforts took the name IPython.

In a nutshell, that small script began as an enhanced version of the Python shell and, through the efforts of other coders and eventually with proper funding from several different companies, it became the wonderful and successful project it is today. Some 10 years after its birth, a Notebook environment was created, powered by technologies such as WebSockets, the Tornado web server, jQuery, CodeMirror, and MathJax. The ZeroMQ library was also used to handle the messages between the Notebook interface and the Python core that lies behind it.

The IPython Notebook became so popular and widely used that, over time, all sorts of goodies were added to it. It can handle widgets, parallel computing, all sorts of media formats, and much, much more. Moreover, at some point, it became possible to code in languages other than Python from within the Notebook.

This led to a huge project that ended up being split into two: IPython has been stripped down to focus more on the kernel part and the shell, while the Notebook has become a brand new project called Jupyter. Jupyter allows interactive scientific computations to be done in more than 40 languages. More recently, the Jupyter project has created JupyterLab, a web-based IDE incorporating Jupyter notebooks, interactive consoles, a code editor, and more.

This chapter's project will all be coded and run in a Jupyter Notebook, so let us briefly explain what a Notebook is. A Notebook environment is a web page that exposes a simple menu and cells in which you can run Python code. Even though the cells are separate entities that you can run individually, they all share the same Python kernel. This means that all the names that you define in one cell (the variables, functions, and so on) will be available in any other cell.

Simply put, a Python kernel is a process in which Python is running. The Notebook web page is, therefore, an interface exposed to the user for driving this kernel. The web page communicates with it using a very fast messaging system.

Apart from all the graphical advantages, the beauty of having such an environment lies in the ability to run a Python script in chunks, and this can be a tremendous advantage. Take a script that is connecting to a database to fetch data and then manipulate that data. If you do it in the conventional way, with a Python script, you have to fetch the data every time you want to experiment with it. Within a Notebook environment, you can fetch the data in one cell and then manipulate and experiment with it in other cells, so fetching it every time is not necessary.

The Notebook environment is also extremely helpful for data science because it allows for the step-by-step inspection of results. You do one chunk of work and then verify it. You then do another chunk and verify again, and so on.

It's also invaluable for prototyping because the results are there, right in front of your eyes, immediately available.

If you want to know more about these tools, please check out ipython.org and jupyter.org.

We have created a very simple example Notebook with a fibonacci() function that gives you a list of all the Fibonacci numbers smaller than a given N. It looks like this:

Figure 13.1: A Jupyter Notebook

Every cell has an In [] label. If there's nothing between the brackets, it means that the cell has never been executed. If there is a number, it means that the cell has been executed, and the number represents the order in which the cell was executed. An * means that the cell is currently being executed.

You can see in the picture that in the first cell we have defined the fibonacci() function and executed it. This has the effect of placing the fibonacci name in the global scope associated with the Notebook, and therefore the fibonacci() function is now available to the other cells as well. In fact, in the second cell, we can run list(fibonacci(100)) and see the results in Out [2]. In the third cell, we have shown you one of the several magic functions you can find in a Notebook: %timeit runs the code several times and provides you with a nice benchmark for it (this is implemented using the timeit module, which we briefly introduced in Chapter 11, Debugging and Profiling).

You can execute a cell as many times as you want, and change the order in which you run them. Cells are very versatile: you can also put in Markdown text or render them as headers.

Markdown is a lightweight markup language with plain text formatting syntax designed so that it can be converted to HTML and many other formats.

Another useful feature is that whatever you place in the last line of a cell will be automatically printed for you. This is very handy because you're not forced to write print(...) explicitly.

## Using Anaconda

As usual, you can install the libraries required for this chapter using the requirements.txt file in the source code for the chapter. Sometimes, however, installing data science libraries can be extremely painful. If you are struggling to install the libraries for this chapter in your virtual environment, an alternative choice you have is to install Anaconda. Anaconda is a free and open-source distribution of the Python and R programming languages for data science and machine learning- related applications that aims to simplify package management and deployment. You can download it from the anaconda.org website. Once you have installed it, you can use the Anaconda interface to create a virtual environment and install the packages listed in the requirements.in file, which you can also find in the source code for the chapter.

### Starting a Notebook

Once you have all the required libraries installed, you can start a Notebook with the following command:

```bash

$ jupyter notebook

```

If you installed the requirements via Anaconda, you can also launch the Notebook from the Anaconda interface. In either case, you will have an open page in your web browser at this address (the port might be different): http://localhost:8888/.

You can also launch JupyterLab from Anaconda, or with the command:
```bash

$ jupyter lab

```

It will also open as a new page in your web browser.

Explore both interfaces. Create a new Notebook, or open the example.ipynb Notebook we showed you above. See which interface you prefer and get comfortable with it before proceeding with the rest of the chapter. We have included a saved JupyterLab workspace containing the Notebooks used in the rest of this chapter in the source code for the chapter (the file is called ch13.jupyterlab-workspace). You can use that to follow along in JupyterLab or stick to the classic Notebook interface if you prefer.

To help you follow along, we will tag each code example in this chapter with the Notebook cell number it belongs to.

If you familiarize yourself with the keyboard shortcuts (look in the classic Notebook Help menu or the Advanced Settings Editor in JupyterLab), you will be able to move between cells and handle their content without having to reach for the mouse. This will make you more proficient and much faster when you work in a Notebook.

Let's now move on and talk about the most interesting part of this chapter: data.

## Dealing with data

Typically, when you deal with data, this is the path you go through: you fetch it; you clean and manipulate it; and then you analyze it and present results as values, spreadsheets, graphs, and so on. We want you to be in charge of all three steps of the process without having any external dependency on a data provider, so we're going to do the following:

1.  We're going to create the data, simulating that it comes in a format that is not perfect or ready to be worked on.
2.  We're going to clean it and feed it to the main tool we'll use in the project, which is a DataFrame from the pandas library.
3.  We're going to manipulate the data in a DataFrame.
4.  We're going to save a DataFrame to a file in different formats.
5.  We're going to analyze the data and get some results out of it.

### Setting up the Notebook

First things first, let's produce the data. We start from the ch13-dataprep Notebook. Cell #1 takes care of the imports:
```python

#1
import json
import random
from datetime import date, timedelta
import faker

```

The only modules we haven't already encountered are random and faker. random is a standard library module for generating pseudo-random numbers. faker is a third-party module for generating fake data. It's very useful in tests, when you prepare your fixtures, to get all sorts of things such as names, email addresses, phone numbers, and credit card details.

### Preparing the data

We want to achieve the following data structure: we're going to have a list of user objects. Each user object will be linked to a number of campaign objects. In Python, everything is an object, so we're using this term in a generic way. The user object may be a string, a dictionary, or something else. A campaign in the social media world is a promotional campaign that a media agency runs on social media networks on behalf of a client. Remember that we're going to prepare this data so that it's not in perfect shape (but it won't be that bad either...). Firstly, we instantiate the Faker that we'll use to create the data:
```python

#2
fake = faker.Faker() 

Then we need usernames. We want 1,000 unique usernames, so we loop until the 
usernames set has 1,000 elements. A set doesn't allow duplicate elements; therefore, 
uniqueness is guaranteed:

#3
usernames = set()
usernames_no = 1000

# populate the set with 1000 unique usernames
while len(usernames) < usernames_no:
    usernames.add(fake.user_name())

```

Next, we create a list of users. Each username will be augmented to a full-blown user dictionary, with other details such as name, gender, and email. Each user dictionary is then dumped to JSON and added to the list. This data structure is not optimal, of course, but we're simulating a scenario where users come to us like that.

```python
#4
def get_random_name_and_gender():
    skew = .6  # 60% of users will be female
    male = random.random() > skew
    if male:
        return fake.name_male(), 'M'
    else:
        return fake.name_female(), 'F'

def get_users(usernames):
    users = []
    for username in usernames:
        name, gender = get_random_name_and_gender()
        user = {
            'username': username,
            'name': name,
            'gender': gender,
            'email': fake.email(),
            'age': fake.random_int(min=18, max=90),
            'address': fake.address(),
        }
        users.append(json.dumps(user))
    return users

users = get_users(usernames)
users[:3]

```

The get_random_name_and_gender() function is quite interesting. We use the random. random() function to get a uniformly distributed random number between 0 and 1. We compare the random number to 0.6, to decide whether to generate a male or a female name. This has the effect of making 60% of our users female.

Note also the last line in the cell. Each cell automatically prints what's on the last line; therefore, the output of #4 is a list with the first three users:

```python
 ['{"username": "susan42", "name": "Emily Smith", "gender": ...}',
'{"username": "sarahcarpenter", "name": "Michael Kane", ...}',
'{"username": "kevin37", "name": "Nathaniel Miller", ...}']
```

We hope you're following along with your own Notebook. If you are, please note that all data is generated using random functions and values; therefore, you will see different results. They will change every time you execute the Notebook. Also note that we've had to trim most of the output in this chapter to fit onto the page, so you will see a lot more output in your Notebook than we've reproduced here.

Analysts use spreadsheets all the time, and they come up with all sorts of coding techniques to compress as much information as possible into the campaign names. The format we have chosen is a simple example of that technique—there is a code that tells us the campaign type, then the start and end dates, then the target age and gender, and finally the currency. All values are separated by an underscore. The code to generate these campaign names can be found in cell #5:

```python
#5
# campaign name format:
# InternalType_StartDate_EndDate_TargetAge_TargetGender_Currency
def get_type():
    # just some gibberish internal codes
    types = ['AKX', 'BYU', 'GRZ', 'KTR']
    return random.choice(types)

def get_start_end_dates():
    duration = random.randint(1, 2 * 365)
    offset = random.randint(-365, 365)
    start = date.today() - timedelta(days=offset)
    end = start + timedelta(days=duration)
    
    def _format_date(date_):
        return date_.strftime("%Y%m%d")
    return _format_date(start), _format_date(end)

def get_age():
    age = random.randrange(20, 46, 5)
    diff = random.randrange(5, 26, 5)
    return '{}-{}'.format(age, age + diff)

def get_gender():
    return random.choice(('M', 'F', 'B'))

def get_currency():
    return random.choice(('GBP', 'EUR', 'USD'))

def get_campaign_name():
    separator = '_'
    type_ = get_type()
    start, end = get_start_end_dates()
    age = get_age()
    gender = get_gender()
    currency = get_currency()
    return separator.join(
        (type_, start, end, age, gender, currency))

```

In the get_type() function, we use random.choice() to get one value randomly out of a collection. get_start_end_dates() is a bit more interesting. We compute two random integers: the duration of the campaign in days (between one day and two years) and an offset (a number of days between –365 and 365). We subtract offset (as a timedelta) from today's date to get the start date and add the duration to get the end date. Finally, we return string representations of both dates.

The get_age() function generates a random target age range, where both endpoints are multiples of five. We use the random.randrange() function, which returns a random number from a range defined by start, stop, and step parameters (these parameters have the same meaning as for the range object that we first encountered in Chapter 3, Conditionals and Iteration). We generate random numbers age (a multiple of 5 between 20 and 46) and diff (a multiple of 5 between 5 and 26). We add diff to age to get the upper limit of our age range and return a string representation of the age range.

The rest of the functions are just some applications of random.choice() and the last one, get_campaign_name(), is nothing more than a collector for all these puzzle pieces that returns the final campaign name.

In #6, we write a function that creates a complete campaign object:
```python

#6
# campaign data:
# name, budget, spent, clicks, impressions
def get_campaign_data():
    name = get_campaign_name()
    budget = random.randint(10**3, 10**6)
    spent = random.randint(10**2, budget) 
    clicks = int(random.triangular(10**2, 10**5, 0.2 * 10**5)) 
    impressions = int(random.gauss(0.5 * 10**6, 2))
    return {
        'cmp_name': name,
        'cmp_bgt': budget,
        'cmp_spent': spent,
        'cmp_clicks': clicks,
        'cmp_impr': impressions
    }


```

We used a few different functions from the random module. random.randint() gives you an integer between two extremes. The problem with it is that it follows a uniform probability distribution, which means that any number in the interval has the same probability of coming up. To avoid having all our data look similar, we chose to use triangular and gauss, for clicks and impressions. They use different probability distributions so that we'll have something more interesting to see in the end.

Just to make sure we're on the same page with the terminology: clicks represents the number of clicks on a campaign advertisement, budget is the total amount of money allocated for the campaign, spent is how much of that money has already been spent, and impressions is the number of times the campaign has been fetched, as a resource, from its source, regardless of the number of clicks that were performed on the campaign. Normally, the number of impressions is greater than the number of clicks, because an advertisement is often viewed without being clicked on.

Now that we have the data, it's time to put it all together:
```python

#7
def get_data(users):
    data = []
    for user in users:
        campaigns = [get_campaign_data()
                     for _ in range(random.randint(2, 8))]
        data.append({'user': user, 'campaigns': campaigns})
    return data

```
As you can see, each item in data is a dictionary with a user and a list of campaigns that are associated with that user.

### Cleaning the data

Let's start cleaning the data:

```python
#8
rough_data = get_data(users)
rough_data[:2]  # let's take a peek

```

We simulate fetching the data from a source and then inspect it. The Notebook is the perfect tool for inspecting your steps. You can vary the granularity to suit your needs. The first item in rough_data looks like this:
```python

 {'user': '{"username": "susan42", "name": "Emily Smith", ...}',
  'campaigns': [{'cmp_name': 'GRZ_20210131_20210411_30-40_F_GBP',
    'cmp_bgt': 253951,
    'cmp_spent': 17953,
    'cmp_clicks': 52573,
    'cmp_impr': 500001},
   ...
   {'cmp_name': 'BYU_20220216_20220407_20-25_F_EUR',
    'cmp_bgt': 393134,
    'cmp_spent': 158930,
    'cmp_clicks': 46631,
    'cmp_impr': 500000}]}

```

Now we can start working on the data. The first thing we need to do in order to be able to work with this data is to denormalize it. Denormalization is a process of restructuring data into a single table. This generally involves joining together data from multiple tables or flattening out nested data structures. It usually introduces some duplication of data; however, it simplifies data analysis by eliminating the need to deal with nested structures or to look related data up across multiple tables. In our case, this means transforming data into a list whose items are campaign dictionaries, augmented with their relative user dictionary. Users will be duplicated in each campaign they are associated with:
```python

#9
data = []
for datum in rough_data:
    for campaign in datum['campaigns']:
        campaign.update({'user': datum['user']})
        data.append(campaign)
data[:2]  # let's take another peek

```
The first item in data now looks like this:
```python

 {'cmp_name': 'GRZ_20210131_20210411_30-40_F_GBP',
  'cmp_bgt': 253951,
  'cmp_spent': 17953,
  'cmp_clicks': 52573,
  'cmp_impr': 500001,
  'user': '{"username": "susan42", "name": "Emily Smith", ...}'}

```

Now, we would like to help you and offer a deterministic second part of the chapter, so we're going to save the data we generated here so that we (and you, too) will be able to load it from the next Notebook, and we should then have the same results:
```python

#10
with open('data.json', 'w') as stream:
    stream.write(json.dumps(data))

```

You should find the data.json file in the source code for the book. Now we are done with ch13-dataprep, so we can close it and open up the ch13 notebook.

### Creating the DataFrame

Now that we have prepared our data, we can start analyzing it. First, we have another round of imports:
```python

#1
import json
import arrow
import numpy as np
import pandas as pd
from pandas import DataFrame


```

We've already seen the json module in Chapter 8, Files and Data Persistence. We also briefly introduced arrow in Chapter 2, Built-In Data Types. It is a very useful third- party library that makes working with dates and times a lot easier. numpy is the NumPy library, the fundamental package for scientific computing with Python. NumPy stands for Numeric Python, and it is one of the most widely used libraries in the data science environment. We'll say a bit more about it later on in this chapter. pandas is the very core upon which the whole project is based. pandas stands for Python Data Analysis Library. Among many other things, it provides the DataFrame, a matrix-like data structure with advanced processing capabilities. It's customary to import pandas as pd and also import DataFrame separately.

After the imports, we load our data into a DataFrame using the pandas.read_json() function:
```python

#2
df = pd.read_json("data.json")
df.head()

```

We inspect the first five rows using the head() method of DataFrame. You should see something like this:

Figure 13.2: The first few rows of the DataFrame

Jupyter automatically renders the output of the df.head() call as HTML. To get a plain text representation, you can wrap df.head() in a print call.

The DataFrame structure is very powerful. It allows us to perform various operations on its contents. You can filter by rows or columns, aggregate data, and so on. You can operate on entire rows or columns without suffering the time penalty you would have to pay if you were working on data with pure Python. This is possible because, under the hood, pandas is harnessing the power of the NumPy library, which itself draws its incredible speed from the low-level implementation of its core.

Using DataFrame allows us to couple the power of NumPy with spreadsheet-like capabilities so that we'll be able to work on our data in a fashion that is similar to what an analyst could do. Only, we do it with code.

Let's see two ways to quickly get a bird's eye view of the data:
```python

#3
df.count()

```

The count() method returns a count of all the non-empty cells in each column. This is useful to help you understand how sparse your data is. In our case, we have no missing values, so the output is:
```bash

cmp_name      5140
cmp_bgt       5140
cmp_spent     5140
cmp_clicks    5140
cmp_impr      5140
user          5140
dtype: int64

```

We have 5,140 rows. Given that we have 1,000 users and the number of campaigns per user is a random number between 2 and 8, that is exactly in line with what we would expect.

The dtype: int64 line at the end of the output indicates that the values returned by df.count() are NumPy int64 objects. Here, dtype stands for "data type" and int64 means 64-bit integers. NumPy is largely implemented in C and, instead of using Python's built-in numeric types, it uses its own types, which are closely related to C language data types. This allows it to perform numerical operations much faster than pure Python.

The describe method is useful to quickly obtain a statistical summary of our data:
```python

#4
df.describe() 

```

As you can see in the output below, it gives us several measures, such as count, mean, std (standard deviation), min, and max, and shows how data is distributed in the various quartiles. Thanks to this method, we already have a rough idea of how our data is structured:
```bash

           cmp_bgt     cmp_spent   cmp_clicks      cmp_impr
count  5140.000000   5140.000000  5140.000000   5140.000000
mean 496331.855058 249542.778210 40414.236576 499999.523346
std  289001.241891 219168.636408  1704.136480      2.010877
min    1017.000000    117.000000   355.000000 499991.000000
25%  250725.500000  70162.000000 22865.250000 499998.000000
50%  495957.000000 188704.000000 37103.000000 500000.000000
75%  741076.500000 381478.750000 55836.000000 500001.000000
max  999860.000000 984005.000000 98912.000000 500007.000000

```
Let's see the three campaigns with the highest budgets:
```python

#5
df.sort_values(by=['cmp_bgt'], ascending=False).head(3) 

```

This gives the following output:
```bash

                              cmp_name cmp_bgt cmp_clicks cmp_impr
5047 GRZ_20210217_20220406_35-45_B_GBP  999860      78932   499999
922  AKX_20211111_20230908_40-50_M_GBP  999859      73078   499996
2113 BYU_20220330_20220401_35-45_B_USD  999696      42961   499998


```
A call to tail() shows us the campaigns with the lowest budgets:
```bash

#6
df.sort_values(by=['cmp_bgt'], ascending=False).tail(3)


```

### Unpacking the campaign name

Now it's time to increase the complexity. First of all, we want to get rid of that horrible campaign name (cmp_name). We need to explode it into parts and put each part in its own dedicated column. In order to do this, we'll use the apply() method of the Series object.

The pandas.core.series.Series class is a powerful wrapper around an array (think of it as a list with augmented capabilities). We can extract a Series object from DataFrame by accessing it in the same way we do with a key in a dictionary, and we can call apply() on that Series object, which will call a given function on each item in the Series and return a new Series with the results. We compose the result into a new DataFrame, and then join that DataFrame with df:

```python

#7
def unpack_campaign_name(name):
    # very optimistic method, assumes data in campaign name
    # is always in good state
    type_, start, end, age, gender, currency = name.split('_')
    start = arrow.get(start, 'YYYYMMDD').date()
    end = arrow.get(end, 'YYYYMMDD').date()
    return type_, start, end, age, gender, currency

campaign_data = df['cmp_name'].apply(unpack_campaign_name)
campaign_cols = [
    'Type', 'Start', 'End', 'Target Age', 'Target Gender',
    'Currency']
campaign_df = DataFrame(
    campaign_data.tolist(), columns=campaign_cols, index=df.index)
campaign_df.head(3)

```

Within unpack_campaign_name(), we split the campaign name into parts. We use arrow.get() to get a proper date object out of those strings, and then we return the objects. A quick peek at the first three rows reveals:
```bash

  Type       Start         End Target Age Target Gender  Currency
0  GRZ  2021-01-31  2021-04-11      30-40             F       GBP
1  BYU  2021-01-09  2022-12-04      30-35             M       GBP
2  GRZ  2021-11-24  2022-09-21      20-35             B       EUR


```

That looks better! One important thing to remember: even if the dates are printed as strings, they are just the representation of the real date objects that are stored in DataFrame.

Another very important thing: when joining two DataFrame instances, it's imperative that they have the same index, otherwise pandas won't be able to know which rows go with which. Therefore, when we create campaign_df, we set its index to the one from df. This enables us to join them:
```python

#8
df = df.join(campaign_df)

```
And after join(), we take a peek, hoping to see matching data:
```python

#9
df[['cmp_name'] + campaign_cols].head(3)

```
The output is as follows:
```bash

                           cmp_name Type      Start        End
0 GRZ_20210131_20210411_30-40_F_GBP  GRZ 2021-01-31 2021-04-11
1 BYU_20210109_20221204_30-35_M_GBP  BYU 2021-01-09 2022-12-04
2 GRZ_20211124_20220921_20-35_B_EUR  GRZ 2021-11-24 2022-09-21

```

As you can see, join() was successful; the campaign name and the separate columns expose the same data. Did you see what we did there? We're accessing the DataFrame using the square brackets syntax, and we pass a list of column names. This will produce a brand new DataFrame, with those columns (in the same order), on which we then call the head() method.

### Unpacking the user data

We now do the same thing for each piece of user JSON data. We call apply() on the user series, running the unpack_user_json() function, which takes a JSON user object and transforms it into a list of its fields. We create a brand new DataFrame, user_df, with this data:
```python

#10
def unpack_user_json(user):
    # very optimistic as well, expects user objects
    # to have all attributes
    user = json.loads(user.strip())
    return [
        user['username'],
        user['email'],
        user['name'],
        user['gender'],
        user['age'],
        user['address'],
    ]

user_data = df['user'].apply(unpack_user_json)
user_cols = [
    'username', 'email', 'name', 'gender', 'age', 'address']
user_df = DataFrame(
    user_data.tolist(), columns=user_cols, index=df.index)


```

Very similar to the previous operation, isn't it? Next, we join user_df back with df (like we did with campaign_df), and take a peek at the result:
```python

#11
df = df.join(user_df)

#12
df[['user'] + user_cols].head(2)


```

The output shows us that everything went well. We're not done yet though. If you call df.columns in a cell, you'll see that we still have ugly names for our columns. Let's change that:
```python

#13
new_column_names = {
    'cmp_bgt': 'Budget',
    'cmp_spent': 'Spent',
    'cmp_clicks': 'Clicks',
    'cmp_impr': 'Impressions',
}
df.rename(columns=new_column_names, inplace=True)

```

The rename() method can be used to change the column (or row) labels. We've given it a dictionary mapping old column names to our preferred names. Any column that's not mentioned in the dictionary will remain unchanged. Now, with the exception of 'cmp_name' and 'user', we only have nice names.

Our next step will be to add some extra columns. For each campaign, we have the number of clicks and impressions, and we have the amounts spent. This allows us to introduce three measurement ratios: CTR, CPC, and CPI. They stand for Click Through Rate, Cost Per Click, and Cost Per Impression, respectively.

The last two are straightforward, but CTR is not. Suffice it to say that it is the ratio between clicks and impressions. It gives you a measure of how many clicks were performed on a campaign advertisement per impression—the higher this number, the more successful the advertisement is in attracting users to click on it. Let's write a function that calculates all three ratios and adds them to the DataFrame:
```python

#14
def calculate_extra_columns(df):
    # Click Through Rate
    df['CTR'] = df['Clicks'] / df['Impressions']
    # Cost Per Click
    df['CPC'] = df['Spent'] / df['Clicks']
    # Cost Per Impression
    df['CPI'] = df['Spent'] / df['Impressions']
calculate_extra_columns(df)


```

Notice that we're adding those three columns with one line of code each, but the DataFrame applies the operation automatically (the division, in this case) to each pair of cells from the appropriate columns. So, even though it looks like we're only doing three divisions, there are actually 5140 * 3 divisions, because they are performed for each row. pandas does a lot of work for us, and also does a very good job of hiding the complexity of it.

The function calculate_extra_columns() takes a DataFrame (df), and works directly on it. This mode of operation is called in-place. This is similar to how the list.sort() method sorts a list. You could also say that this function is not pure, which means it has side effects, as it modifies the mutable object it is passed as an argument.

We can take a look at the results by filtering on the relevant columns and calling head():
```python

#15
df[['Spent', 'Clicks', 'Impressions',
    'CTR', 'CPC', 'CPI']].head(3)


```
This shows us that the calculations were performed correctly on each row:
```python
 
    Spent  Clicks  Impressions       CTR       CPC       CPI
0   17953   52573       500001  0.105146  0.341487  0.035906
1  125884   24575       499999  0.049150  5.122442  0.251769
2  480963   39668       499999  0.079336 12.124710  0.961928


```
Now, we want to verify the accuracy of the results manually for the first row:
```python

#16
clicks = df['Clicks'][0]
impressions = df['Impressions'][0]
spent = df['Spent'][0]
CTR = df['CTR'][0]
CPC = df['CPC'][0]
CPI = df['CPI'][0]
print('CTR:', CTR, clicks / impressions)
print('CPC:', CPC, spent / clicks)
print('CPI:', CPI, spent / impressions)


```
This yields the following output:
```python

CTR: 0.10514578970842059 0.10514578970842059
CPC: 0.3414870751146026 0.3414870751146026
CPI: 0.03590592818814362 0.03590592818814362


```

The values match, confirming that our computations are correct. Of course, we wouldn't normally need to do this, but we wanted to show you how can you perform calculations this way. You can access a Series (a column) by passing its name to the DataFrame in square brackets (this is similar to looking up a key in a dictionary). You can then access each row in the column by its position, exactly as you would with a regular list or tuple.

We're almost done with our DataFrame. All we are missing now is a column that tells us the duration of the campaign and a column that tells us which day of the week corresponds to the start date of each campaign. The duration is important to have, since it allows us to relate data such as the amount spent or number of impressions to the duration of the campaign (we may expect longer running campaigns to cost more and have more impressions). The day of the week can also be useful; for example, some campaigns may be tied to events that happen on particular days of the week (such as sports events that take place on Saturdays or Sundays).
```python

#17
def get_day_of_the_week(day):
    return day.strftime("%A")    
def get_duration(row):
    return (row['End'] - row['Start']).days

df['Day of Week'] = df['Start'].apply(get_day_of_the_week)
df['Duration'] = df.apply(get_duration, axis=1)

```

get_day_of_the_week() is very simple. It takes a date object and formats it as a string, which only contains the name of the corresponding day of the week. get_duration() is more interesting. First, notice it takes an entire row, not just a single value. In its body, we perform a subtraction between a campaign's end and start dates. When you subtract date objects, the result is a timedelta object, which represents a given amount of time. We take the value of its .days property to get the duration in days.

Now, we can introduce the fun part, the application of those two functions. First, we apply get_day_of_the_week() to the Start column (as a Series object); this is similar to what we did with 'user' and 'cmp_name'. Next, we apply get_duration() to the whole DataFrame and, in order to instruct pandas to perform that operation on the rows, we pass axis=1.

We can verify the results very easily, as shown here:
```python

#18
df[['Start', 'End', 'Duration', 'Day of Week']].head(3)


```
This gives the following output:
```python

        Start         End  Duration Day of Week
0  2021-01-31  2021-04-11        70      Sunday
1  2021-01-09  2022-12-04       694    Saturday
2  2021-11-24  2022-09-21       301   Wednesday

```

So, we now know that between the 9th of January, 2021, and the 4th of December, 2022, there are 694 days, and that the 31st of January, 2021, is a Sunday.

### Cleaning everything up

Now that we have everything we want, it's time to do the final cleaning; remember we still have the 'cmp_name' and 'user' columns. Those are useless now, so they have to go. We also want to reorder the columns in our DataFrame so that they are more relevant to the data it now contains. In order to do this, we just need to filter df on the column list we want. We'll get back a brand new DataFrame that we can reassign to df itself:
```python

#19
final_columns = [
    'Type', 'Start', 'End', 'Duration', 'Day of Week', 'Budget',
    'Currency', 'Clicks', 'Impressions', 'Spent', 'CTR', 'CPC',
    'CPI', 'Target Age', 'Target Gender', 'Username', 'Email',
    'Name', 'Gender', 'Age'
]
df = df[final_columns]


```
We have grouped the campaign information at the beginning, then the measurements, and finally the user data at the end. Now our DataFrame is clean and ready for us to inspect.

Before we start going crazy with graphs, how about taking a snapshot of DataFrame so that we can easily reconstruct it from a file, rather than having to redo all the steps we did to get here? Some analysts may want to have it in spreadsheet form, to do a different kind of analysis than the one we want to do, so let's see how to save a DataFrame to a file. It's easier done than said.

### Saving the DataFrame to a file

We can save a DataFrame in many different ways. You can type df.to_ and then press Tab to make autocompletion pop up, so you can see all the possible options. We're going to save our DataFrame in three different formats, just for fun. First, CSV:
```python

#20
df.to_csv('df.csv')

Then JSON:

#21
df.to_json('df.json')

#And finally, in an Excel spreadsheet:

#22
df.to_excel('df.xlsx')

```

The to_excel() method requires the openpyxl package to be installed. It is included in the requirements.txt file for this chapter, so if you used that to install the requirements, you should have it in your virtual environment.

So, it's extremely easy to save a DataFrame in many different formats, and the good news is that the reverse is also true: it's very easy to load a spreadsheet into a DataFrame (just use the pandas read_csv() or read_excel() functions). The programmers behind pandas went a long way to make our tasks easier, which is something to be grateful for.

## Visualizing the results

Finally, the juicy bits. In this section, we're going to visualize some results. From a data science perspective, we're not going to go deep into analyzing our data, especially because the data is completely random. However, this example should still be enough to get you started with graphs and other features.

Something we have learned in our lives, and this may come as a surprise to you, is that looks also count, so it's very important that when you present your results, you do your best to make them pretty.

pandas uses the Matplotlib plotting library to draw graphs. We won't be using it directly, except to configure the plot style. You can learn more about this versatile plotting library at https://matplotlib.org/.

First, we'll tell the Notebook to render Matplotlib graphs in the cell output frame, rather than in a separate window. We do it with the following:
```python

#23
%matplotlib inline

```

Then, we proceed to set up some styling for our plots:
```python

#24
import matplotlib.pyplot as plt
plt.style.use(['classic', 'ggplot'])
plt.rc('font', family='serif'})


```

We use the matplotlib.pyplot interface to set the plot style. We've chosen to use a combination of the classic and ggplot style sheets. Style sheets are applied from left to right, so here ggplot will override the classic style for any style items that are defined in both. We also set the font family used in the plots to serif.

Now that our DataFrame is complete, let's run df.describe() (#25) again. The results should look like this:

Figure 13.3: Some statistics for our cleaned-up data

This kind of quick result is perfect for satisfying those managers who have 20 seconds to dedicate to you and just want rough numbers.

Once again, please keep in mind that our campaigns have different currencies, so these numbers are actually meaningless. The point here is to demonstrate the DataFrame capabilities, not to get to a correct or detailed analysis of real data.

Alternatively, a graph is usually much better than a table with numbers because it's much easier to read it and it gives you immediate feedback. So, let's plot the four pieces of information we have on each campaign: 'Budget', 'Spent', 'Clicks', and 'Impressions':
```python

#26
df[['Budget', 'Spent', 'Clicks', 'Impressions']].hist(
    bins=16, figsize=(16, 6));


```

We extract those four columns (this will give us another DataFrame made with only those columns) and call the hist() method to get a histogram plot. We give some arguments to specify the number of bins and figure sizes, and everything else is done automatically.

Since the histogram plot instruction is the last statement in the cell, the Notebook will print its result before drawing the graph. To suppress this behavior and have only the graph drawn with no printing, we add a 
semicolon at the end. Here are the graphs:

Figure 13.4: Histogram plots of the campaign data

They are beautiful, aren't they? Did you notice the serif font? How about the meaning of those figures? If you go back and take a look at the way we generate the data, you will see that all these graphs make perfect sense:

- Budget is selected randomly from an interval, so we expect a uniform distribution. Looking at the graph, that is exactly what we see: it's practically a constant line.
- Spent is also uniformly distributed, but its upper limit is the budget, which is not constant. This means we should expect an approximately logarithmic curve that decreases from left to right. Again, that is exactly what the graph shows.
- Clicks was generated with a triangular distribution with a mean roughly 20% of the interval size, and you can see that the peak is right there, at about 20% to the left. Impressions was a Gaussian distribution, which assumes the famous bell shape. The mean was exactly in the middle and we had a standard deviation of 2. You can see that the graph matches those parameters.

Good! Let's plot out the measures we calculated:

```python
#27
df[['CTR', 'CPC', 'CPI']].hist(
    bins=20, figsize=(16, 6))

```
Here is the plot representation:

Figure 13.5: Histogram plots of computed measures

We can see that the CPC is highly skewed to the left, meaning that most of the CPC values are very low. The CPI has a similar shape, but is less extreme. Suppose you want to analyze only a particular segment of the data; how would you do it? We can apply a mask to the DataFrame so that we get a new DataFrame with only the rows that satisfy the mask condition. It's like applying a global, row-wise if clause:
```python

#28
selector = (df.Spent > 0.75 * df.Budget)
df[selector][['Budget', 'Spent', 'Clicks', 'Impressions']].hist(
    bins=15, figsize=(16, 6), color='green');

```

In this case, we prepared selector to filter out all the rows for which the amount spent is less than or equal to 75% of the budget. In other words, we'll include only those campaigns for which we have spent at least three-quarters of the budget. Notice that in selector, we are showing you an alternative way of asking for a DataFrame column, by using direct property access (object.property_name), instead of dictionary-like access (object['property_name']). If property_name is a valid Python name, you can use both ways interchangeably.

selector is applied in the same way that we access a dictionary with a key. When we apply selector to df, we get back a new DataFrame and we select only the relevant columns from this and call hist() again. This time, just for fun, we want the results to be green:

Figure 13.6: Histogram plots of campaign data where at least 75% of the budget was spent

Note that the shapes of the graphs haven't changed much, apart from the Spent graph, which is quite different. The reason for this is that we've asked only for the rows where the amount spent is at least 75% of the budget. This means that we're including only the rows where the amount spent is close to the budget. The budget numbers come from a uniform distribution. Therefore, it is quite obvious that the Spent graph is now assuming that kind of shape. If you make the boundary even tighter and ask for 85% or more, you'll see the Spent graph become more and more like the Budget one. Let's now ask for something different. How about the measure of 'Spent', 'Clicks', and 'Impressions' grouped by day of the week?
```python

#29
df_weekday = df.groupby(['Day of Week']).sum()
df_weekday[['Impressions', 'Spent', 'Clicks']].plot(
    figsize=(16, 6), subplots=True);


```

The first line creates a new DataFrame, df_weekday, by asking for a grouping by 'Day of Week' on df. The function used to aggregate the data is an addition.

The second line gets a slice of df_weekday using a list of column names, something we're accustomed to by now. On the result, we call plot(), which is a bit different to hist(). The subplots=True option makes plot draw three separate graphs:

Figure 13.7: Plots of campaign data aggregated by day of the week

Interestingly enough, we can see that most of the action happens on Thursdays and Wednesdays. If this were meaningful data, this would potentially be important information to give to our clients, which is why we're showing you this example.

Note that the days are sorted alphabetically, which scrambles them up a bit. Can you think of a quick solution that would fix the issue? We'll leave it to you as an exercise to come up with something.

Let's finish this presentation section with a couple more things. First, a simple aggregation. We want to aggregate on 'Target Gender' and 'Target Age', and show 'Impressions' and 'Spent'. For both, we want to see 'mean' and the standard deviation ('std'):
```python

#30
agg_config = {
    'Impressions': ['mean', 'std'],
    'Spent': ['mean', 'std'],
}
df.groupby(['Target Gender', 'Target Age']).agg(agg_config)


```

It's very easy to do. We prepare a dictionary that we'll use as a configuration. Then, we perform a grouping on the 'Target Gender' and 'Target Age' columns, and we pass our configuration dictionary to the agg() method. The output looks like this:
```python
                            
                            Impressions                    Spent
                                   mean       std           mean
Target Gender Target Age                                        
B             20-25       499999.513514  2.068970  225522.364865
              20-30       499999.233766  2.372519  248960.376623
              20-35       499999.678161  1.742053  280387.114943
...                                 ...       ...            ...
M             45-50       499999.000000  1.490712  323302.200000
              45-55       499999.142857  1.956674  344844.142857
              45-60       499999.750000  1.332785  204209.750000

```

Let's do one more thing before we wrap this chapter up. We want to show you something called a pivot table. A pivot table is a way of grouping data, computing an aggregate value for each group, and displaying the result in table form. The pivot table is an essential tool for data analysis, so let's see a simple example:
```python

#31
df.pivot_table(
    values=['Impressions', 'Clicks', 'Spent'],
    index=['Target Age'],
    columns=['Target Gender'],
    aggfunc=np.sum
)

```

We create a pivot table that shows us the correlation between 'Target Age' and 'Impressions', 'Clicks', and 'Spent'. These last three will be subdivided according to 'Target Gender'. The aggregation function (aggfunc) used to calculate the results is the numpy.sum() function (numpy.mean would be the default, had we not specified anything). The result is a new DataFrame containing the pivot table:

Figure 13.8: A pivot table

Figure 13.8 shows a crop of the output. It's pretty clear and provides very useful information when the data is meaningful.

That's it! We'll leave you to discover more about the wonderful world of IPython, Jupyter, and data science. We strongly encourage you to get comfortable with the Notebook environment. It's much better than a console, it's extremely practical and fun to use, and you can even create slides and documents with it.

## Where do we go from here?

Data science is indeed a fascinating subject. As we said in the introduction, those who want to delve into its meanders need to be well trained in mathematics and statistics. Working with data that has been interpolated incorrectly renders any result about it useless. The same goes for data that has been extrapolated incorrectly or sampled with the wrong frequency. To give you an example, imagine a population of individuals that are aligned in a queue. If for some reason, the gender of that population alternated between male and female, the queue would look something like this: F-M-F-M-F-M-F-M-F...

If you sampled it taking only the even elements, you would draw the conclusion that the population was made up only of males, while sampling the odd ones would tell you exactly the opposite.

Of course, this was just a silly example, but it's very easy to make mistakes in this field, especially when dealing with big datasets where sampling is mandatory and, therefore, the quality of your analysis depends, first and foremost, on the quality of the sampling itself.

When it comes to data science and Python, these are the main tools you want to look at:

- NumPy (https://www.numpy.org/): This is the main package for scientific computing with Python. It contains a powerful N-dimensional array object, sophisticated (broadcasting) functions, tools for integrating C/C++ and Fortran code, useful linear algebra, the Fourier transform, random number capabilities, and much more.

- Scikit-learn (https://scikit-learn.org/): This is probably the most popular machine learning library in Python. It has simple and efficient tools for data mining and data analysis, is accessible to everybody, and is reusable in various contexts. It's built on NumPy, SciPy, and Matplotlib.

- pandas (https://pandas.pydata.org/): This is an open-source, BSD-licensed library providing high-performance, easy-to-use data structures and data analysis tools. We've used it throughout this chapter. IPython (https://ipython.org/)/Jupyter (https://jupyter.org/): These provide a rich architecture for interactive computing.


- Matplotlib (https://matplotlib.org/): This is a Python 2-D plotting library that produces publication-quality figures in a variety of hard-copy formats and interactive environments across platforms. Matplotlib can be used in Python scripts, the Python and IPython shell, a Jupyter Notebook, web application servers, and several graphical user interface toolkits.

- Numba (https://numba.pydata.org/): This gives you the power to speed up your applications with high-performance functions written directly in Python. With a few annotations, array-oriented and math-heavy Python code can be just-in-time compiled to native machine instructions, similar in performance to C, C++, and Fortran, without having to switch languages or Python interpreters.

- Bokeh (https://bokeh.pydata.org/): This is a Python-interactive visualization library that targets modern web browsers for presentation. Its goal is to provide elegant, concise construction of novel graphics in the style of D3.js, but also deliver this capability with high-performance interactivity over very large or streaming datasets.

Other than these single libraries, you can also find ecosystems, such as SciPy (https://scipy.org/) and the aforementioned Anaconda (https://anaconda.org/), that bundle several different packages in order to give you something that just works in an "out-of-the-box" fashion.

Installing all these tools and their several dependencies is hard on some systems, so we suggest that you try out ecosystems as well to see whether you are comfortable with them. It may be worth it.

## Summary

In this chapter, we talked about data science. Rather than attempting to explain anything about this extremely wide subject, we delved into a project. We familiarized ourselves with the Jupyter Notebook, and with different libraries, such as pandas, Matplotlib, and numPy.

Of course, having to compress all this information into one single chapter means we could only touch briefly on the subjects we presented. We hope the project we've gone through together has been comprehensive enough to give you an idea of what could potentially be the workflow you follow when working in this field.

The next chapter is dedicated to API development.
