from rich.table import Table
import pandas as pd

# Example Data Frame
df = pd.DataFrame([{1: 200, 2:300, 3:400}, {1:1000, 2:20000, 3:30000}])

def rich_pandas(df):
    """ This function takes a pandas data frame and returns a pretty printable rich.table for any data frame """
    table = Table(title = "Tables")
    table.add_column(df.keys)
    table.add_row(df.values)

    return rich_df
    
    # Rich Table wants you to format rows and columns separately. See https://rich.readthedocs.io/en/stable/tables.html for more info on the API and the expected output.
    # Pandas Data Frame have an attribute columns, which carries all columns(!)
    # The rows of a Data Frame are returned from the to_records() functions


'''Pretty printing is a virtue. However, ever artist also needs basic efficiency. Can you pretty print a tabular data structure programmatically? You get two hints in the code already. As a data structure, please use a data frame from the Pandas Package. The output is handled via the exciting Rich Library. Both of them have been imported for you already. Unfortunately, rich does not offer direct pandas compatability with its table module. Can you connect the two and define a function, which pretty prints any Pandas Data Frame with Rich? '''
