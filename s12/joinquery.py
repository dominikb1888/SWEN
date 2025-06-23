import datetime

def readField(record,colTypes,fieldNum):
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

def main():
    # SELECT Feed.FeedNum, Feed.Name, FeedAttribType.Name, FeedAttribute.Value WHERE
    # Feed.FeedID = FeedAttribute.FeedID AND 
    # FeedAttribute.FeedAttribTypeID = FeedAttribType.FeedAttribTypeID
    
    attribTypeCols = ["int","char20","char60","int","int","int","int"]
    feedCols = ["int","int","int","char50","datetime","float","float","int","char50","int"]
    feedAttributeCols = ["int","int","float"]
    
    before = datetime.datetime.now()
    feedAttributeTable = open("FeedAttribute.tbl","r")
    
    for record in feedAttributeTable:
        
        feedID = readField(record,feedAttributeCols,0)
        feedAttribTypeID = readField(record,feedAttributeCols,1)
        value = readField(record,feedAttributeCols,2)
        
        feedTable = open("Feed.tbl","r")
        
        feedFeedID = -1
        
        while feedFeedID != feedID:
            feedRecord = feedTable.readline()   
            feedFeedID = readField(feedRecord,feedCols,0)
            
            
        feedNum = readField(feedRecord,feedCols,2)
        feedName = readField(feedRecord,feedCols,3)
        
        feedAttribTypeTable = open("FeedAttribType.tbl", "r")
        
        feedAttribTypeIDID = -1
        
        while feedAttribTypeIDID != feedAttribTypeID:
            feedAttribTypeRecord = feedAttribTypeTable.readline()
            feedAttribTypeIDID = readField(feedAttribTypeRecord,attribTypeCols,0)
            
        feedAttribTypeName = readField(feedAttribTypeRecord,attribTypeCols,1)
        
        print(feedNum,feedName,feedAttribTypeName,value)
    after = datetime.datetime.now()
    deltaT = after - before
    milliseconds = deltaT.total_seconds() * 1000    
    print("Done. The total time for the query without indexing was",milliseconds, \
        "milliseconds.")
    
if __name__ == "__main__":
    main()