from rich.table import Table
import pandas as pd

# Example Data Frame
df = pd.DataFrame([{1: 200, 2:300, 3:400}, {1:1000, 2:20000, 3:30000}])

def rich_pandas(df):
    """ This function takes a pandas data frame and returns a pretty printable rich.table for any data frame """
    rich_df = Table
    show_index = True # 
    for column in df.columns:
        rich_df.add_column(str(column))

    for index, value_list in enumerate(df.values.tolist()):
        row = [str(index)] if show_index else []
        row += [str(x) for x in value_list]
        rich_df.add_row(*row)

    # Rich Table wants you to format rows and columns separately. See https://rich.readthedocs.io/en/stable/tables.html for more info on the API and the expected output.

    # Pandas Data Frame have an attribute colums, which carries all columns(!)

    # The rows of a Data Frame are returned from the to_records() functions

    return rich_df
