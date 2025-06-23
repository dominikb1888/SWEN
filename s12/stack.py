class Stack:
    def __init__(self):
        self.items = []
        
    def pop(self):
        if self.isEmpty():
            raise RuntimeError("Attempt to pop an empty stack")
        
        topIdx = len(self.items)-1
        item = self.items[topIdx]
        del self.items[topIdx]
        return item
    
    def push(self,item):
        self.items.append(item)
        
    def top(self):
        if self.isEmpty():
            raise RuntimeError("Attempt to get top of empty stack")
        
        topIdx = len(self.items)-1
        return self.items[topIdx]
    
    def isEmpty(self):
        return len(self.items) == 0

    def clear(self):
        self.items = []
    
def main():
    s = Stack()
    items = list(range(10))
    items2 = []
    
    for k in items:
        s.push(k)
        
    if s.top() == 9:
        print("Test 1 Passed")
    else:
        print("Test 1 Failed")
        
    while not s.isEmpty():
        items2.append(s.pop())

    items2.reverse()
    
    if items2 != items:
        print("Test 2 Failed")
    else:
        print("Test 2 Passed")
        
    try:
        s.pop()
        print("Test 3 Failed")
        
    except RuntimeError:
        print("Test 3 Passed")
    except:
        print("Test 3 Failed")

    try:
        s.top()
        print("Test 4 Failed")
        
    except RuntimeError:
        print("Test 4 Passed")
    except:
        print("Test 4 Failed")   
        
if __name__=="__main__":
    main()