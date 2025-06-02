class AVLTree:
    class AVLNode:
        def __init__(self, item, balance=0, left=None, right=None):
            self.item = item
            self.left = left
            self.right = right
            self.balance = balance # -1,0,1

        def __repr__(self):
            return "AVLTree.AVLNode("+repr(self.item)+",balance="+ repr(self.balance)+",left="+repr(self.left)+",right="+repr(self.right)+")"
