
board = [
    [{5},{3},0,0,{7},0,0,0,0],
    [{6},0,0,{1},{9},{5},0,0,0],
    [0,{9},{8},0,0,0,0,{6},0],
    [{8},0,0,0,{6},0,0,0,{3}],
    [{4},0,0,{8},0,{3},0,0,{1}],
    [{7},0,0,0,{2},0,0,0,{6}],
    [0,{6},0,0,0,0,{2},{8},0],
    [0,0,0,{4},{1},{9},0,0,{5}],
    [0,0,0,0,{8},0,0,{7},{9}]
]

def create_start_sets(board):
    for x,row in enumerate(board):
        for y,field in enumerate(row):
            if field == 0:
                board[x][y] = set(range(10))

    return board

def reduce_sets(board, x, y):
    start_x = (x // 3) * 3
    start_y = (y // 3) * 3

    union_result = set()

    for s in board[x]:
        if len(s) == 1:
            union_result = union_result.union(s)

    for s in board[y]:
        if len == 1:
            union_result = union_result.union(s)

    matrix = [i for s in board[start_x:start_x+3][start_y:start_y+3] for i in s]

    for s in matrix:
        if len == 1:
            union_result = union_result.union(s)

    # compute the union result on the current field
    board[x][y] = board[x][y] - union_result

def find_open_sets(board):
    # implement a function that finds all sets with a length > 1
    pass

def valid(board, num, x, y):
    if num in board[x, :]:
        return False
    if num in board[:, y]:
        return False
    start_x = (x // 3) * 3
    start_y = (y // 3) * 3
    if num in board[start_x:start_x+3, start_y:start_y+3]:
        return False
    return True

def sudoku(board):
    # This needs to adapt to the new code
    x, y = find_empty_cell(board)
    if (x, y) == (-1, -1):
        return True  # Board solved

    # Add a new way to range based on find_open_sets()
    # for num in range(1, 10):
        if valid(board, num, x, y):
            board[x, y] = num
            if sudoku(board):
                return True
            board[x, y] = 0  # Undo move (backtracking)

    return False  # No valid number found, need to backtrack

# --- Main program ---

import time
start_time = time.time()

print("Original board:")
print(board)

if sudoku(board):
    print("Solved board:")
    print(board)
else:
    print("No solution exists.")

print("--- %s seconds ---" % (time.time() - start_time))
