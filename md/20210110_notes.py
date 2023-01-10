# OOP Solution
class Person:
    def __init__(self, name, surname, species='Human'):
        self.name = name
        self.surname = surname

        self.species = species

    def namelength(self):
        return len(self.name) + len(self.surname)

 class Employee(Person):
    def __init__(self, name, surname, wage, employer=''):
        Person.__init__(self, name, surname)
        self.wage = wage
        self.employer = employer

    def get_wage(self):
        return f"The wage of {self.name} is {self.wage} Euros"


Steven = Employee("Steven", "Smith", 1000)
Steven.get_wage()

## Functional Solution
employee_list = [
    {
        'name': 'Steven',
        'surname': 'Smith',
        'species': 'Human',
        'wage': 1000,
        'employer': ''
    },
    {
        'name': 'Nothing',
        'surname': 'Void',
        'species': 'Human',
        'wage': 2000,
        'employer': ''
    }
]

def get_wage(employee):
     return f"The wage of Steven is {employee['wage']} Euros"

def namelength(employee):
     return len(employee['name']) + len(employee['surname'])

Steven_fp = employee_list[0]
Nothing_fp = employee_list[1]

Something = dict(name="Something", surname="Wow", wage=5000)

get_wage(Steven_fp)

