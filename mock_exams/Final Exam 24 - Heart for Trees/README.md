## Heart for Trees
You have a good friend who is a cardiologist. He is all about analyzing heart rates as efficiently as possible. He asked you to consult him on a project he is currently involved in that is at risk. Somehow the previous software developers found out they have a love for real trees now and abandoned their project.

1. The first thing your friend wants to know: Why did they choose a tree and not a lake or a mountain to store the data? He is by no means an expert. However, perhaps you can help him by discussing the benefits of using a tree structure for storing time-series data like heart rates compared to other data structures such as arrays or linked lists. (18 Points)

Answer in the open text box directly here online.

2. The second thing you friend want you to do, is to fix the code. Can you help him and implement the tree structure and the two insert methods for storing the heart rate data? (24 Points)

3. He usually also needs the average heart rate of the last minute. Please implement the methods for the average heart rate of the last minute, which includes the sum_and_count method. Do not use any pre-built function to calculate the average. (24 Points)

4. Your friend looks at the result and is really happy! However, the printed output  in debug mode with timestamps is not really nice. Can you implement the Display trait for your data structure so that a call to println!() will automatically format the data with dates in ISO-8601 format:

2021-07-01T12:26:40+00:00 - 72
2021-07-01T12:28:20+00:00 - 75
2021-07-01T12:30:00+00:00 - 78
(Hint: You can use the to_rfc3339() function) (24 Points)



Please submit ONE lib.rs file only. Do NOT submit more files or archives please.
