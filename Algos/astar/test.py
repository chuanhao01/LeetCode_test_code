import queue

q = queue.PriorityQueue()
q.put((1, 'A', 'B'))

print(q.qsize())
print(q.get())
print(q.qsize())

