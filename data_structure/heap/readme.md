# heap

A heap is a tree-based data structure that satisfies the **heap property**. For a min-heap, the heap property is such that, for any parent node **p** of a node **u**, node **p** have a key that is lower than the key of **u**. In a max-heap, **p** key must be greater than that of **u**. 

Heaps can be used to sort an array in O(n log n) time. This is achieved by getting the array's values into a heap, and then by popping the root node value successively, while ensuring the heap property between each pop. 

Here you can find a binary min-heap implementation.