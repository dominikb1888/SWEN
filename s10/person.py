'''
File: person.py
Author: Steve Hubbard
Date: 8/16/13
Description: The class Person will be used in our hash tables and
  Btrees.  All that is required is that the messages
  hash(self.id) and self.id == anInstance.id work.  Hence, the id
  instance variable could be numeric or a string.
'''

from copy import deepcopy

class Person:
    def __init__(self, name, id = -1):
        self.name = name
        self.id = id

    def __str__(self):  
        return 'Name: %s Id: %s '% (self.name, self.id)
    
    def __repr__(self):  #needed to display a Person object in a list or tuple
        return 'Name: %s Id: %s '% (self.name, self.id)
    
    def __le__(self, other):
        if type(other) == Person:
            return self.id <= other.id
        
    def __lt__(self, other):
        if type(other) == Person:
            return self.id < other.id
        
    def __eq__(self, other):
        if type(other) == Person:
            return self.id == other.id
  
    def __ge__(self, other):
        if type(other) == Person:
            return self.id >= other.id
        
    def __gt__(self, other):
        if type(other) == Person:
            return self.id > other.id

    def __hash__(self):
        # Used for hash(aPerson)
        return hash(self.id)
    
    def getId(self):
        return self.id

    def setId(self, newId): # Private method
        self.id = newId
        
    def getName(self):
        return self.name

    def setName(self, newName):
        self.name = newName
        
class Student( Person ):
    def __init__(self, name, id, gpa):
        Person.__init__(self, name, id)
        self.gpa = gpa
        
    def __str__(self):
        st = Person.__str__(self) + "gpa: " +str(self.gpa)
        return st
    

def main():
    
    fred = Person('Fred', 1000)
    joe = Person('Joe', 99)
    print(fred <= joe)
    print(fred >= joe)
    print(joe == fred)
    print(joe.getName(), joe.getId())
    print(hash(fred), hash(joe))
    clem = deepcopy(joe)
    joe.setName('Sue')
    
    print(joe)
    print (clem)
    print(fred <= 1000)
    print((fred, joe))
    print(fred != joe)
    print(fred == Person('Sue', 1000))
    print(fred)
    print([fred, clem])
    
    jim = Student("Jim", 89, 4.00)
    print(jim)
    
if __name__ == '__main__': main()


''' output:
>>> [evaluate person.py]
False
True
False
Joe 99
1000 99
Name: Sue Id: 99 
Name: Joe Id: 99 
None
(Name: Fred Id: 1000 , Name: Sue Id: 99 )
True
True
Name: Fred Id: 1000 
[Name: Fred Id: 1000 , Name: Joe Id: 99 ]
Name: Jim Id: 89 gpa: 4.0
>>> 
'''

