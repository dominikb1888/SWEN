import pandas as pd
from rich.table import Table

def rich_df(df):
    table = Table()
    for col in df.columns:
        table.add_column(str(col))

    for row in df.to_records(index=False):
        table.add_row(*[str(item) for item in row])

    return table
