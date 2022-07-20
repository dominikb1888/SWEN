from rich.table import Table
import pandas as pd
from rich.columns import Columns

# Example Data Frame
df = pd.DataFrame([{1: 200, 2:300, 3:400}, {1:1000, 2:20000, 3:30000}])

def rich_pandas(df):

    """ This function takes a pandas data frame and returns a pretty printable rich.table for any data frame """
    
    # Rich Table wants you to format rows and columns separately. See https://rich.readthedocs.io/en/stable/tables.html for more info on the API and the expected output.

    # Pandas Data Frame have an attribute colums, which carries all columns(!)

    # The rows of a Data Frame are returned from the to_records() functions

def re_recors():

    return rich_df
