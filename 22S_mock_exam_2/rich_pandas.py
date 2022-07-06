from rich.table import Table
import pandas as pd

# Example Data Frame
df = pd.DataFrame([{1: 200, 2:300, 3:400}, {1:1000, 2:20000, 3:30000}])

def rich_pandas(df):
    """ This function takes a pandas data frame and returns a pretty printable rich.table for any data frame """
    
    # Rich Table wants you to format rows and columns separately. See https://rich.readthedocs.io/en/stable/tables.html for more info on the API and the expected output.
    df.columns

    # Pandas Data Frame have an attribute columns, which carries all columns(!)
    df.to_records()
    
    # The rows of a Data Frame are returned from the to_records() method

    return rich_df
