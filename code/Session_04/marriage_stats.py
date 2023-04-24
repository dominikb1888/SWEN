from dataclasses import dataclass
import datetime
import csv
from typing import Any

@dataclass
class MarriageRecord:
    stat_id: int
    center: str
    amount: int
    period: datetime.date

@dataclass
class MarriageStats:
    data: list[MarriageRecord] = None

    def __post_init__(self):
        if self.data:
            for i, item in enumerate(self.data):
                if not isinstance(item, MarriageRecord):
                    self.data[i] = MarriageRecord(*item)

    # The classmethod decorator allows alternative init functions, e.g.
    # with a csv file instead of a list as an input. It will take the file
    # pre-process it and create the object
    @classmethod
    def from_csv(cls, filepath):
        with open(filepath) as f:
            csv_data = csv.reader(f, delimiter=',')
            data = []
            next(csv_data)
            for row in csv_data:
                row[0] = int(row[0])
                row[1] = str(row[1])
                row[2] = int(row[2])
                year, month = row[3].split('-')
                row[3] = datetime.date(year=int(year), month=int(month), day=1)
                data.append(MarriageRecord(*row))

        return cls(data)

    def average_licenses(self, key: str) -> int:
         licenses_issued = [row.amount for row in self.data if row.center == key]
         if licenses_issued == []:
             return 0
         else:
             return sum(licenses_issued) / len(licenses_issued)

    def get_averages(self) -> dict:
        centers = {record.center for record in self.data}
        return {key: self.average_licenses(key) for key in centers}


if __name__ == '__main__':
    # marriage_data = [
    #  [1657, 'ET', 80, datetime.date(2011, 1, 1)],
    #  [1658, 'NY', 136, datetime.date(2011, 1, 1)],
    #  [1659, 'SC', 159, datetime.date(2011, 1, 1)],
    #  [1660, 'TO', 367, datetime.date(2011, 1, 1)],
    #  [1661, 'ET', 109, datetime.date(2011, 2, 1)],
    #  [1662, 'NY', 150, datetime.date(2011, 2, 1)],
    #  [1663, 'SC', 154, datetime.date(2011, 2, 1)],
    #  [1664, 'TO', 383, datetime.date(2011, 2, 1)]
    # ]

    stats = MarriageStats().from_csv("Marriage Licence Statistics Data.csv")
    print(stats.get_averages())

