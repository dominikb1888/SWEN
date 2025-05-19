import orderedtreeset

def main():
    s = orderedtreeset.OrderedTreeSet(list(range(100)))
    
    t = orderedtreeset.OrderedTreeSet(list(range(10,20)))
    
    u = orderedtreeset.OrderedTreeSet(list(range(10,20)))
    
    if len(t) == len(u) and len(t) == 10:
        print("Test 1 Passed")
    else:
        print("Test 1 Failed")
        
    s.intersection_update(t)
    
    if len(s) == 10:
        print("Test 2 Passed")
    else:
        print("Test 2 Failed")
        
    s = orderedtreeset.OrderedTreeSet(list(range(100)))
    
    t.update(s)
    
    if len(s) == len(t):
        print("Test 3 Passed")
    else:
        print("Test 3 Failed")
        
    t.clear()
    t.update(u)
    
    if len(t) == len(u):
        print("Test 4 Passed")
    else:
        print("Test 4 Failed")
        
    s.difference_update(t)
    
    test5Passed = True
    test6Passed = True
    
    for x in range(1,10):
        if x in s:
            pass
        else:
            test5Passed = False
            print("Test 5 Failed on",x)
            
        if x not in s:
            test6Passed = False
            print("Test 6 Failed on",x)
            
    if test5Passed:
        print("Test 5 Passed")
    
    if test6Passed:
        print("Test 6 Passed")
        

    test7Passed = True
    test8Passed = True
    
    for x in range(20,100):
        if x in s:
            pass
        else:
            test7Passed = False
            print("Test 7 Failed on",x)
            
        if x not in s:
            test8Passed = False
            print("Test 8 Failed on",x)
            
    if test7Passed:
        print("Test 7 Passed")
    
    if test8Passed:
        print("Test 8 Passed")   
        
    x = orderedtreeset.OrderedTreeSet(["a","b","c","d","e","f","g","h","i","j","k"])
    
    y = orderedtreeset.OrderedTreeSet(["c","d","e","l","m","n"])
    
    z = x.difference(y)
    
    if len(z) == 8:
        print("Test 9 Passed")
    else:
        print("Test 9 Failed")
        
    test10Passed = True
    
    for item in z:
        if item not in ["a","b","f","g","h","i","j","k"]:
            test10Passed = False
            print("Test 10 Failed on", x)
            
    if test10Passed:
        print("Test 10 Passed")
        
    if z.issubset(x):
        print("Test 11 Passed")
    else:
        print("Test 11 Failed")
        
    if x.issuperset(z):
        print("Test 12 Passed")
    else:
        print("Test 12 Failed")
        
    if z == y:
        print("Test 13 Failed")
    else:
        print("Test 13 Passed")
        
    if z == z:
        print("Test 14 Passed")
    else:
        print("Test 14 Failed")
        
    r = z.copy()
    
    if r == z:
        print("Test 15 Passed")
    else:
        print("Test 15 Failed")
        
    z = orderedtreeset.OrderedTreeSet(list(range(50)))
        
    for item in range(50):
        z.discard(item)
        
    if len(z) == 0:
        print("Test 16 Passed")
    else:
        print("Test 16 Failed")    
        
    z = orderedtreeset.OrderedTreeSet(list(range(50)))
        
    lastItem = -99999999999999999999999999999
    test17Passed = True
    
    for item in z:
        if lastItem >= item:
            print("Test 17 Failed with ", lastItem, "and", item, "out of order.")
            test17Passed = False
            
        lastItem = item
            
    if test17Passed:
        print("Test 17 Passed")  
        
    for item in range(25):
        z.remove(item)  
    
    lastItem = -99999999999999999999999999999
    test18Passed = True
    
    for item in z:
        if lastItem >= item:
            print("Test 18 Failed with ", lastItem, "and", item, "out of order.")
            test18Passed = False
            
        lastItem = item
            
    if test18Passed:
        print("Test 18 Passed") 
        
    if len(z) == 25:
        print("Test 19 Passed")
    else:
        print("Test 19 Failed")       

    
if __name__ == "__main__":
    main()
    