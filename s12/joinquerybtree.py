'''
  File: btree.py
  Author: Steve Hubbard and Kent D. Lee
  Date: 8/27/2014
  Description: This module provides the BTree class, based on support from
    the BTreeNode class.  The BTreeNode class is also implemented in this
    module. This module is meant to support the iterative implementation of
    insert and lookup within a BTree.

    The module requires the Stack and Queue classes from the corresponding
    two modules, stack.py and queue.py.

    This program has two main functions, the btreemain function and the main
    function. The btreemain function tests the BTree datatype. The expected
    output is provided in a comment after the function. Once the btreemain
    function runs and produces the proper output, the main function can be
    run to test the BTree with the join functionality.

    The main function either builds a new BTree or reads an existing BTree
    from the index files, Feed.idx and FeedAttribType.idx files. If the idx
    file does not exist, then a new BTree is built and written to the
    corresponding idx file.
'''

import datetime
import os
from copy import deepcopy
import sys
import stack
import queue


class BTreeNode:
    '''
      This module provides the BTreeNode class.  This class will
      be used by the BTree class.  Much of the functionality of
      B-trees is provided by this class.
    '''
    def __init__(self, degree = 1, numberOfKeys = 0, items = None, child = None, \
        index = None):
        ''' Create an empty node with the indicated degree'''
        self.numberOfKeys = numberOfKeys
        if items != None:
            self.items = items
        else:
            self.items = [None]*2*degree
        if child != None:
            self.child = child
        else:
            self.child = [None]*(2*degree+1)
        self.index = index

    def __repr__(self):
        return "BTreeNode("+str(len(self.items)//2)+","+str(self.numberOfKeys)+ \
            ","+repr(self.items)+","+repr(self.child)+","+str(self.index)+")\n"

    def __str__(self):
        st = 'The contents of the node with index '+ \
             str(self.index) + ':\n'
        for i in range(0, self.numberOfKeys):
            st += '   Index   ' + str(i) + '  >  child: '
            st += str(self.child[i])
            st += '   item: '
            st += str(self.items[i]) + '\n'
        st += '                 child: '
        st += str(self.child[self.numberOfKeys]) + '\n'
        return st

    def addItemAndSplit(self, anItem, left, right):
        '''
         If the receiver is not full, generate an error.
         If full, split the receiver into two nodes, the
         smallest degree + 1 keys staying in the original node.
         The largest degree keys go into a new node which is
         returned. Note that the last child of the receiver
         and the first child of the new node will be the same.
        '''
        pass

    def getChild(self,i):
        # Answer the index of the ith child
        if (0 <= i <= self.numberOfKeys):
            return self.child[i]
        else:
            print( 'Error in getChild().' )

    def setChild(self, i, childIndex):
        # Set the ith child of the node to childIndex
        self.child[i] = childIndex

    def childIndexOf(self, anIndex):
        # Answer the index of the child in the receiver
        # which contains anIndex.
        index = -1
        k = 0
        found = False
        while not found and k <= self.numberOfKeys :
            if self.child[k] == anIndex:
                index = k
                found = True
            else:
                k += 1
        if index < 0:
            print( 'Error in childIndexOf' )
            return -1
        else:
            return index

    def clear(self):
        self.numberOfKeys = 0
        self.items = [None]*len(self.items)
        self.child = [None]*len(self.child)

    def copyItemsAndChildren(self, fromNode, start,
                             finish, index):
        '''The receiver gets the contents of the fromNode from
          index start to finish, along with the next child.
          The copying within the receiver begins at position
          index. Answer the receiver.
        '''
        pass

    def copyWithRight(self, aNode, parentNode):
        '''Answer a node which contains all the items and children
          of the receiver, followed by the parent item followed by
          all the items and children of aNode.  The receiver and
          aNode are left and right siblings wrt the parentNode.
        '''
        pass

    def insertItem(self, anItem, left = 0, right = 0):
        ''' We assume that the receiver is not full. anItem is
          inserted into the BTreeNode with child indices left and
          right.  This is done while retaining the <= ordering on
          the key of the item.  If the insertion is successful,
          answer True.  If not, answer False.
        '''
        for i, item in enumerate(self.items):
            if anItem == item:
                return False

            if item == None:
                self.items[i] = anItem
                return True

            if anItem < item:
                self.items.insert(i, anItem)
                if self.isFull:
                    if self.items[-1] == None:
                        self.items.pop(-1)
                    else:
                        pass # implement child behavior later

                return True

            if anItem > item:
                continue

    def isFull(self):
        ''' Answer True if the receiver is full.  If not, return
          False.
        '''
        return (self.numberOfKeys == len(self.items))

    def removeChild(self, index):
        ''' If the index is valid, remove and answer the child at
          location index.  If not, answer None.  In any event,
          do NOT update the key count.  We copy all the rest of
          the child entries down one, as removeItem will
          decrement numberOfKeys.
        '''
        pass

    def removeItem(self, index):
        '''If index is valid, remove and answer the item at
          location index. Update the key count.  If not, answer
          None.
        '''
        pass

    def searchNode(self, anItem):
        '''Answer a dictionary satisfying: at 'found'
          either True or False depending upon whether the receiver
          has a matching item;  at 'nodeIndex', either the index of
          the matching item, or in the case of an unsuccessful
          search, the index of the smallest (first) item such that
          anItem < item, or self.numberOfKeys if all items
          are < anItem.  In other words, nodeIndex is the place in the node
          where the object is, or should go if there is room in the node.
        '''
        pass

    def setIndex(self, anInteger):
        self.index = anInteger

    def setNumberOfKeys(self, anInt ):
        self.numberOfKeys = anInt

class BTree:
    def __init__(self, degree, nodes = {}, rootIndex = 1, freeIndex = 2):
        # This method is complete
        self.degree = degree

        if len(nodes) == 0:
            self.rootNode = BTreeNode(degree)
            self.nodes = {}
            self.rootNode.setIndex(rootIndex)
            self.writeAt(1, self.rootNode)
        else:
            self.nodes = deepcopy(nodes)
            self.rootNode = self.nodes[rootIndex]

        self.stackOfNodes = stack.Stack()
        self.rootIndex = rootIndex
        self.freeIndex = freeIndex

    def __repr__(self):
        # This method is complete
        return "BTree("+str(self.degree)+",\n "+repr(self.nodes)+","+ \
            str(self.rootIndex)+","+str(self.freeIndex)+")"

    def __str__(self):
        # This method is complete
        st = '  The degree of the BTree is ' + str(self.degree)+\
             '.\n'
        st += '  The index of the root node is ' + \
              str(self.rootIndex) + '.\n'
        for x in range(1, self.freeIndex):
            node = self.readFrom(x)
            if node.getNumberOfKeys() > 0:
                st += str(node)
        return st


    def delete(self, anItem):
        ''' Answer None if a matching item is not found.  If found,
          answer the entire item.
        '''
        pass

    def getFreeIndex(self):
        # Answer a new index and update freeIndex.  Private
        self.freeIndex += 1
        return self.freeIndex - 1

    def getFreeNode(self):
        #Answer a new BTreeNode with its index set correctly.
        #Also, update freeIndex.  Private
        newNode = BTreeNode(self.degree)
        index = self.getFreeIndex()
        newNode.setIndex(index)
        self.writeAt(index,newNode)
        return newNode

    def inorderOn(self, aFile):
        '''
          Print the items of the BTree in inorder on the file
          aFile.  aFile is open for writing.
          This method is complete at this time.
        '''
        aFile.write("An inorder traversal of the BTree:\n")
        self.inorderOnFrom( aFile, self.rootIndex)

    def inorderOnFrom(self, aFile, index):
        ''' Print the items of the subtree of the BTree, which is
          rooted at index, in inorder on aFile.
        '''
        pass

    def insert(self, anItem):
        ''' Answer None if the BTree already contains a matching
          item. If not, insert a deep copy of anItem and answer
          anItem.
        '''
        return anItem if self.nodes[self.rootIndex].insertItem(anItem) else None

    def levelByLevel(self, aFile):
        ''' Print the nodes of the BTree level-by-level on aFile. )
        '''
        pass

    def readFrom(self, index):
        ''' Answer the node at entry index of the btree structure.
          Later adapt to files
        '''
        if self.nodes.__contains__(index):
            return self.nodes[index]
        else:
            return None

    def recycle(self, aNode):
        # For now, do nothing
        aNode.clear()

    def retrieve(self, anItem):
        ''' If found, answer a deep copy of the matching item.
          If not found, answer None
        '''
        pass

    def __searchTree(self, anItem):
        ''' Answer a dictionary.  If there is a matching item, at
          'found' is True, at 'fileIndex' is the index of the node
          in the BTree with the matching item, and at 'nodeIndex'
          is the index into the node of the matching item.  If not,
          at 'found' is False, but the entry for 'fileIndex' is the
          leaf node where the search terminated.  An important
          function of this method is that it pushes all of the
          nodes of the search path from the rootnode, down to,
          but not including the corresponding leaf node of a search
          (or the node containing a match).  Again, the rootnode
          is pushed if it is not a leaf node and has no match.
        '''
        pass

    def update(self, anItem):
        ''' If found, update the item with a matching key to be a
          deep copy of anItem and answer anItem.  If not, answer None.
        '''
        pass

    def writeAt(self, index, aNode):
        ''' Set the element in the btree with the given index
          to aNode.  This method must be invoked to make any
          permanent changes to the btree.  We may later change
          this method to work with files.
          This method is complete at this time.
        '''
        self.nodes[index] = aNode

def btreemain():
    print("My/Our name(s) is/are ")

    lst = [10,8,22,14,12,18,2,50,15]

    b = BTree(2)

    for x in lst:
        print(repr(b))
        print("***Inserting",x)
        b.insert(x)

    print(repr(b))

    lst = [14,50,8,12,18,2,10,22,15]

    for x in lst:
        print("***Deleting",x)
        b.delete(x)
        print(repr(b))

    #return
    lst = [54,76]

    for x in lst:
        print("***Deleting",x)
        b.delete(x)
        print(repr(b))

    print("***Inserting 14")
    b.insert(14)

    print(repr(b))

    print("***Deleting 2")
    b.delete(2)

    print(repr(b))

    print ("***Deleting 84")
    b.delete(84)

    print(repr(b))

    return

'''
Here is the expected output from running this program. Depending on the order
of redistributing or coalescing, your output may vary. However, the end result
in every case should be the insertion or deletion of the item from the BTree.

My/Our name(s) is/are
BTree(2,
 {1: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],1)
},1,2)
***Inserting 10
BTree(2,
 {1: BTreeNode(2,1,[10, None, None, None],[None, None, None, None, None],1)
},1,2)
***Inserting 8
BTree(2,
 {1: BTreeNode(2,2,[8, 10, None, None],[None, None, None, None, None],1)
},1,2)
***Inserting 22
BTree(2,
 {1: BTreeNode(2,3,[8, 10, 22, None],[None, None, None, None, None],1)
},1,2)
***Inserting 14
BTree(2,
 {1: BTreeNode(2,4,[8, 10, 14, 22],[None, None, None, None, None],1)
},1,2)
***Inserting 12
BTree(2,
 {1: BTreeNode(2,2,[8, 10, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,2,[14, 22, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
},3,4)
***Inserting 18
BTree(2,
 {1: BTreeNode(2,2,[8, 10, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,3,[14, 18, 22, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
},3,4)
***Inserting 2
BTree(2,
 {1: BTreeNode(2,3,[2, 8, 10, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,3,[14, 18, 22, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
},3,4)
***Inserting 50
BTree(2,
 {1: BTreeNode(2,3,[2, 8, 10, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,4,[14, 18, 22, 50],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
},3,4)
***Inserting 15
BTree(2,
 {1: BTreeNode(2,3,[2, 8, 10, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,2,[14, 15, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,2,[12, 18, None, None],[1, 2, 4, None, None],3)
, 4: BTreeNode(2,2,[22, 50, None, None],[None, None, None, None, None],4)
},3,5)
***Deleting 14
BTree(2,
 {1: BTreeNode(2,3,[2, 8, 10, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,4,[15, 18, 22, 50],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},3,5)
***Deleting 50
BTree(2,
 {1: BTreeNode(2,3,[2, 8, 10, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,3,[15, 18, 22, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},3,5)
***Deleting 8
BTree(2,
 {1: BTreeNode(2,2,[2, 10, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,3,[15, 18, 22, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[12, None, None, None],[1, 2, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},3,5)
***Deleting 12
BTree(2,
 {1: BTreeNode(2,2,[2, 10, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,2,[18, 22, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,1,[15, None, None, None],[1, 2, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},3,5)
***Deleting 18
BTree(2,
 {1: BTreeNode(2,4,[2, 10, 15, 22],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 2
BTree(2,
 {1: BTreeNode(2,3,[10, 15, 22, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 10
BTree(2,
 {1: BTreeNode(2,2,[15, 22, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 22
BTree(2,
 {1: BTreeNode(2,1,[15, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 15
BTree(2,
 {1: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 54
BTree(2,
 {1: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 76
BTree(2,
 {1: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Inserting 14
BTree(2,
 {1: BTreeNode(2,1,[14, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 2
BTree(2,
 {1: BTreeNode(2,1,[14, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
***Deleting 84
BTree(2,
 {1: BTreeNode(2,1,[14, None, None, None],[None, None, None, None, None],1)
, 2: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],2)
, 3: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],3)
, 4: BTreeNode(2,0,[None, None, None, None],[None, None, None, None, None],4)
},1,5)
'''

def readRecord(file,recNum,recSize):
    file.seek(recNum*recSize)
    record = file.read(recSize)
    return record

def readField(record, colTypes, fieldNum, val=None):
    # fieldNum is zero based
    # record is a string containing the record
    # colTypes is the types for each of the columns in the record

    offset = 0
    for i in range(fieldNum):
        colType = colTypes[i]

        if colType == "int":
            offset+=10
        elif colType[:4] == "char":
            size = int(colType[4:])
            offset += size
        elif colType == "float":
            offset+=20
        elif colType == "datetime":
            offset+=24

    colType = colTypes[fieldNum]

    if colType == "int":
        value = record[offset:offset+10].strip()
        if value == "null":
            val = None
        else:
            val = int(value)
    elif colType == "float":
        value = record[offset:offset+20].strip()
        if value == "null":
            val = None
        else:
            val = float(value)
    elif colType[:4] == "char":
        size = int(colType[4:])
        value = record[offset:offset+size].strip()
        if value == "null":
            val = None
        else:
            val = value[1:-1] # remove the ' and ' from each end of the string
            if type(val) == bytes:
                val = val.decode("utf-8")
    elif colType == "datetime":
        value = record[offset:offset+24].strip()
        if value == "null":
            val = None
        else:
            if type(val) == bytes:
                val = val.decode("utf-8")
            val = datetime.datetime.strptime(val,'%m/%d/%Y %I:%M:%S %p')
    else:
        print("Unrecognized Type")
        raise Exception("Unrecognized Type")

    return val

class Item:
    def __init__(self,key,value):
        self.key = key
        self.value = value

    def __repr__(self):
        return "Item("+repr(self.key)+","+repr(self.value)+")"

    def __eq__(self,other):
        if type(self) != type(other):
            return False

        return self.key == other.key

    def __lt__(self,other):
        return self.key < other.key

    def __gt__(self,other):
        return self.key > other.key

    def __ge__(self,other):
        return self.key >= other.key

    def getValue(self):
        return self.value

    def getKey(self):
        return self.key

def main():
    # Select Feed.FeedNum, Feed.Name, FeedAttribType.Name, FeedAttribute.Value where
    # Feed.FeedID = FeedAttribute.FeedID and FeedAttribute.FeedAtribTypeID = FeedAttribType.ID
    attribTypeCols = ["int","char20","char60","int","int","int","int"]
    feedCols = ["int","int","int","char50","datetime","float","float","int","char50","int"]
    feedAttributeCols = ["int","int","float"]

    feedAttributeTable = open("FeedAttribute.tbl","r")

    if os.path.isfile("Feed.idx"):
        indexFile = open("Feed.idx","r")
        feedTableRecLength = int(indexFile.readline())
        feedIndex = eval("".join(indexFile.readlines()))
    else:
        feedIndex = BTree(3)
        feedTable = open("Feed.tbl","r")
        offset = 0
        for record in feedTable:
            feedID = readField(record,feedCols,0)
            anItem = Item(feedID,offset)
            feedIndex.insert(anItem)
            offset+=1
            feedTableRecLength = len(record)

        print("Feed Table Index Created")
        indexFile = open("Feed.idx","w")
        indexFile.write(str(feedTableRecLength)+"\n")
        indexFile.write(repr(feedIndex)+"\n")
        indexFile.close()

    if os.path.isfile("FeedAttribType.idx"):
        indexFile = open("FeedAttribType.idx","r")
        attribTypeTableRecLength = int(indexFile.readline())
        attribTypeIndex = eval("".join(indexFile.readlines()))
    else:
        attribTypeIndex = BTree(3)
        attribTable = open("FeedAttribType.tbl","r")
        offset = 0
        for record in attribTable:
            feedAttribTypeID = readField(record,attribTypeCols,0)
            anItem = Item(feedAttribTypeID,offset)
            attribTypeIndex.insert(anItem)
            offset+=1
            attribTypeTableRecLength = len(record)

        print("Attrib Type Table Index Created")
        indexFile = open("FeedAttribType.idx","w")
        indexFile.write(str(attribTypeTableRecLength)+"\n")
        indexFile.write(repr(attribTypeIndex)+"\n")
        indexFile.close()

    feedTable = open("Feed.tbl","rb")
    feedAttribTypeTable = open("FeedAttribType.tbl", "rb")
    before = datetime.datetime.now()
    for record in feedAttributeTable:

        feedID = readField(record,feedAttributeCols,0)
        feedAttribTypeID = readField(record,feedAttributeCols,1)
        value = readField(record,feedAttributeCols,2)

        lookupItem = Item(feedID,None)
        item = feedIndex.retrieve(lookupItem)
        offset = item.getValue()
        feedRecord = readRecord(feedTable,offset,feedTableRecLength)
        feedNum = readField(feedRecord,feedCols,2)
        feedName = readField(feedRecord,feedCols,3)

        lookupItem = Item(feedAttribTypeID,None)
        item = attribTypeIndex.retrieve(lookupItem)
        offset = item.getValue()
        feedAttribTypeRecord = readRecord(feedAttribTypeTable,offset,attribTypeTableRecLength)
        feedAttribTypeName = readField(feedAttribTypeRecord,attribTypeCols,1)

        print(feedNum,feedName,feedAttribTypeName,value)
    after = datetime.datetime.now()
    deltaT = after - before
    milliseconds = deltaT.total_seconds() * 1000
    print("Done. The total time for the query without indexing was",milliseconds, \
      "milliseconds.")

if __name__ == "__main__":
    btreemain()
