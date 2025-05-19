import random

class OrderedTreeSet:
    class BinarySearchTree:
        # This is a Node class that is internal to the BinarySearchTree class. 
        class Node:
            def __init__(self,val,left=None,right=None):
                self.val = val
                self.left = left
                self.right = right
                
            def getVal(self):
                return self.val
            
            def setVal(self,newval):
                self.val = newval
                
            def getLeft(self):
                return self.left
            
            def getRight(self):
                return self.right
            
            def setLeft(self,newleft):
                self.left = newleft
                
            def setRight(self,newright):
                self.right = newright
                
            # This method deserves a little explanation. It does an inorder traversal
            # of the nodes of the tree yielding all the values. In this way, we get
            # the values in ascending order.
            def __iter__(self):
                if self.left != None:
                    for elem in self.left:
                        yield elem
                        
                yield self.val
                
                if self.right != None:
                    for elem in self.right:
                        yield elem

            def __repr__(self):
                return "BinarySearchTree.Node(" + repr(self.val) + "," + repr(self.left) + "," + repr(self.right) + ")"            
                
        # Below are the methods of the BinarySearchTree class. 
        def __init__(self, root=None):
            self.root = root
            
        def insert(self,val):
            self.root = OrderedTreeSet.BinarySearchTree.__insert(self.root,val)
            
        def __insert(root,val):
            if root == None:
                return OrderedTreeSet.BinarySearchTree.Node(val)
            
            if val < root.getVal():
                root.setLeft(OrderedTreeSet.BinarySearchTree.__insert(root.getLeft(),val))
            else:
                root.setRight(OrderedTreeSet.BinarySearchTree.__insert(root.getRight(),val))
                
            return root
            
        def __iter__(self):
            if self.root != None:
                return iter(self.root)
            else:
                return iter([])

        def __str__(self):
            return "BinarySearchTree(" + repr(self.root) + ")"
            
    def __init__(self,contents=None):
        self.tree = OrderedTreeSet.BinarySearchTree()
        if contents != None:
            # randomize the list
            indices = list(range(len(contents)))
            random.shuffle(indices)
            
            for i in range(len(contents)):
                self.tree.insert(contents[indices[i]])
                
            self.numItems = len(contents)
        else:
            self.numItems = 0
            
    def __str__(self):
        pass
    
    def __iter__(self):
        return iter(self.tree)
    
    # Following are the mutator set methods 
    def add(self, item):
        pass
                
    def remove(self, item):
        pass
        
    def discard(self, item):
        pass
        
    def pop(self):
        pass
            
    def clear(self):
        pass
        
    def update(self, other):
        pass
            
    def intersection_update(self, other):
        pass
            
    def difference_update(self, other):
        pass
                
    def symmetric_difference_update(self, other):
        pass
                
    # Following are the accessor methods for the HashSet  
    def __len__(self):
        pass
    
    def __contains__(self, item):
        pass
    
    # One extra method for use with the HashMap class. This method is not needed in the 
    # HashSet implementation, but it is used by the HashMap implementation. 
    def __getitem__(self, item):
        pass      
        
    def not__contains__(self, item):
        pass
    
    def isdisjoint(self, other):
        pass
    
    
    def issubset(self, other):
        pass
            
    
    def issuperset(self, other):
        pass
    
    def union(self, other):
        pass
    
    def intersection(self, other):
        pass
    #done
    def difference(self, other):
        pass
    
    def symmetric_difference(self, other):
        pass
    
    def copy(self):
        pass
    
    # Operator Definitions
    def __or__(self, other):
        pass
    
    def __and__(self,other):
        pass
    
    def __sub__(self,other):
        pass
    
    def __xor__(self,other):
        pass
    
    def __ior__(self,other):
        pass
    
    def __iand__(self,other):
        pass
    
    def __ixor(self,other):
        pass    
    
    def __le__(self,other):
        pass
    
    def __lt__(self,other):
        pass
    
    def __ge__(self,other):
        pass
    
    def __gt__(self,other):
        pass
    
    def __eq__(self,other):
        pass      
                
            
    
 
def main():
    s = input("Enter a list of numbers: ")
    lst = s.split()
    
    tree = OrderedTreeSet()
    
    for x in lst:
        tree.add(float(x))
        
    for x in tree:
        print(x)

if __name__ == "__main__":
    main()