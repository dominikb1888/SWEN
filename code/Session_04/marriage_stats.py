from dataclasses import dataclass
import datetime
from typing import Any

@dataclass
class MarriageRecord:
    stat_id: int
    center: str
    amount: int
    period: datetime.date

@dataclass
class MarriageStats:
    data: list[Any]

    def __post_init__(self):
        for i, item in enumerate(self.data):
            self.data[i] = MarriageRecord(*item)

    def average_licenses(self, key: str) -> int:
         licenses_issued = [row.amount for row in self.data if row.center == key]
         if licenses_issued == []:
             return 0
         else:
             return sum(licenses_issued) / len(licenses_issued)

    def get_averages(self):
        centers = {record.center for record in self.data}
        return {key: self.average_licenses(key) for key in centers}


if __name__ == '__main__':
    marriage_data = [
     [1657, 'ET', 80, datetime.date(2011, 1, 1)],
     [1658, 'NY', 136, datetime.date(2011, 1, 1)],
     [1659, 'SC', 159, datetime.date(2011, 1, 1)],
     [1660, 'TO', 367, datetime.date(2011, 1, 1)],
     [1661, 'ET', 109, datetime.date(2011, 2, 1)],
     [1662, 'NY', 150, datetime.date(2011, 2, 1)],
     [1663, 'SC', 154, datetime.date(2011, 2, 1)],
     [1664, 'TO', 383, datetime.date(2011, 2, 1)]
    ]

    stats = MarriageStats(marriage_data)


    print(stats.get_averages())

