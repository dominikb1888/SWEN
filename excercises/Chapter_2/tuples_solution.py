import sys
import csv
from collections import namedtuple


def get_csv_data():
    with open("tuples_data.csv", "rt") as csvfile:
        csv_data = csv.reader(csvfile, delimiter=",", quotechar='"')
        # Movie = namedtuple("Movie", next(csv_data))
        # data = []
        # for row in csv_data:
        #     data.append(Movie(*row))
        header = next(csv_data)
        data = [dict(zip(row, header)) for row in csv_data]
    # return print(data)
    return print(sys.getsizeof(data))


def select_field(data, field):
    result = []
    for row in data:
        result.append(getattr(row, field))

    return result


if __name__ == "__main__":
    get_csv_data()
    # print(select_field(get_csv_data(), "year"))
