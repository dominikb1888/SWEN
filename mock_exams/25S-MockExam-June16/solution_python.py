class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None

    def insert_at_end(self, value):
        new_node = Node(value)
        if self.head is None:
            self.head = new_node  # New node becomes the head
            return

        current = self.head
        while current.next is not None:
            current = current.next
        current.next = new_node
        return self.head # Head remains the same if list was not empty
