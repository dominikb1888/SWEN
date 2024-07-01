## Session 12

1. Recap Rectangles Exercises

2. Graphs what are they, why are they useful?
    - Trees are just directed, acyclic graphs
    - N-dimensional Data structures possible
    - Levels of Abstraction and Lack of Words: Data Structure in Memory != Data Structure in Program
    - How do we represent tabular data? What if there is more than two dimensions? (1 Dim = list, 2 Dim = Table, 3 Dim = Graph)
    - In memory: List can be a graph, too?!
    - Accessing Data efficiently for machine access vs. storing data for systems

3. Implementing a GraphData Structure
    - Nodes, Edges, Weights
    - shortest Path Algorithms

4. Representing Graphs
    - How to represent a graph?! Adjacency List, Adjacency Matrix
    - Let's build the Internet of Things ;-)

5. Implementing Ord Trait for Tentative Weight
    - Representing number or infinite? Which type?
    - Can any int store infinite?
    - Enum as algebraic Data Type, Number or Infinite
    - Compare a number with infinite we must. Meeeh. Don't Work
    - Let's implement Ord trait for TentativeWeight

6. Neighborhood Search: Implement connected function
    - Direct Neighborhood
    - Let's provide weights
    - Analyze Big-O: Recursion vs. iteration?
    - Tail-call optimization in Rust


7. Shortest Path: Review Dijkstra's Algorithm
