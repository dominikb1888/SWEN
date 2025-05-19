colors = {1: None, 2: None, 3: None, 4: None, 5: None}
color_list = ["White", "Brown", "Yellow", "Blue", "Green"]

graph = {
1: [None, {2, 4}],
2: [None, {1, 4, 3}],
3: [None, {2, 4, 5}],
4: [None, {1, 2, 3, 5}],
5: [None, {3, 4}]
}


def find_uncolored(graph):
    for i, node in graph.items():
        if node[0] == None:
            return i

def is_valid(node, color):
    for connection in node[1]:
        if graph[connection][1] == color:
            return False

    return True

def solve(graph):
    i = find_uncolored(graph)
    # This needs to adapt to the new code
    #if list(graph.items())[-1][0] != None:
    #   return graph§

    print(i, graph)
    # Add a new way to range based on find_open_sets()
    for color in colors:
        if is_valid(graph[i], color):
            graph[i][0] = color
            if solve(graph):
                return True
            graph[i][0] = None  # Undo move (backtracking)

    return False  # No valid number found, need to backtrack

print(solve(graph))
