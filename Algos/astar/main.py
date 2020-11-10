import queue
import math

class Node:
    '''
    The struct to represent a node in the graph
    '''
    def __init__(self, x, y, key: str):
        '''
        Init the Node struct

        Parameters
        ------------
        x: number
            A number to represent the x location of the node
        y: number
            A number to represent the y location of the node
        key: str
            A unique identifier of the node
        '''
        self.x = x
        self.y = y
        self.key = key
        self.edges = []
    
    def add_edge(self, target_node, weight):
        '''
        Helper function to add an edge from this node to target node

        Parameters
        ------------
        target_node: Node
            Target node to add an edge from this node to
        weight: number
            The weight value associated with the edge
        '''
        self.edges.append((target_node, weight))
    
    def get_dist(self, target_node):
        # Return dist between node and target node
        pass
    
class Graph:
    def __init__(self, nodes_repr=None, graph_repr=None):
        '''
        Init graph
        '''
        self.nodes = {}
        if nodes_repr is not None or graph_repr is not None:
            assert nodes_repr is not None and graph_repr is not None
            for key, (x, y) in nodes_repr.items():
                node = Node(x, y, key)
                self.nodes[key] = node
            for key, edges in graph_repr.items():
                for target_node_key, weight in edges:
                    self.nodes[key].add_edge(self.nodes[target_node_key], weight)
    
    def get_node(self, key):
        return self.nodes[key]

class AStar:
    def __init__(self, start_key, end_key):
        self.start_key = start_key
        self.end_key = end_key
        self.open_queue = queue.PriorityQueue()
        self.open_scores = {}
        self.closed_nodes = {}
    
    def run(self, graph):
        # Place current node into open queue
        self.open_queue.put((0, self.start_key))
        self.open_scores[self.start_key] = {
            'last': None
        }
        while True:
            # Checking if there are no more paths
            if self.open_queue.qsize() == 0:
                # Retuning that no path are available
                # Let upstream handle the exception`
                return None

            # Taking the weight and node from the queue
            # This will always be smallest, because of prio queue
            # Since we are checking if this value has been checked before, the larger values will be ignored
            cur_weight, cur_node_key = self.open_queue.get()

            if cur_node_key == self.end_key:
                # If we are taking out the end node key
                # We have found the shortest path
                found_path = []
                prev_node_key = cur_node_key
                while prev_node_key is not None:
                    found_path.append(prev_node_key)
                    prev_node_key = self.open_scores[prev_node_key]['last']
                return found_path

            if cur_node_key not in self.closed_nodes:
                # Node is not closed
                # Grab the node
                cur_node = graph.get_node(cur_node_key)
                # For each connection
                for edge in cur_node.edges:
                    target_node, target_weight = edge
                    target_node_key = target_node.key
                    g = cur_weight + target_weight
                    h = math.sqrt((cur_node.x - target_node.x)**2 + (cur_node.y - target_node.y)**2)
                    f = g + h
                    if target_node_key not in self.open_scores:
                        # Does not have a score
                        self.open_scores[target_node_key] = {
                            'f': f,
                            'g': g,
                            'h': h,
                            'last': cur_node_key
                        }
                        self.open_queue.put((f, target_node_key))
                    else:
                        # There is a prev cost
                        if f < self.open_scores[target_node_key]['f']:
                            # Current cost is smaller
                            self.open_scores[target_node_key] = {
                                'f': f,
                                'g': g,
                                'h': h,
                                'last': cur_node_key
                            }
                            self.open_queue.put((f, target_node_key))


if __name__ == "__main__":
    # graph_repr = {
    #     'start': [('A', 7), ('B', 2), ('G', 3)],
    #     'A': [('C', 4)],
    #     'B': [('A', 3), ('C', 4), ('D', 1)],
    #     'C': [('E', 5)],
    #     'D': [('E', 3), ('F', 2)],
    #     'F': [('end', 2)],
    #     'G': [('H', 2)],
    #     'H': [('I', 4), ('J', 4)],
    #     'I': [('J', 6), ('K', 4)],
    #     'J': [('K', 4)],
    #     'K': [('end', 5)]
    # }
    nodes_repr = {
        'start': (40, 40),
        'A': (10, 30),
        'B': (20, 30),
        'C': (70, 30),
        'D': (80, 20),
        'end': (25, 10)
    }
    graph_repr = {
        'start': [('A', 7), ('B', 2), ('C', 5)],
        'A': [('end', 2)],
        'B': [('A', 3), ('end', 10)],
        'C': [('D', 6)],
        'D': [('end', 2)],
    }
    graph = Graph(nodes_repr=nodes_repr, graph_repr=graph_repr)
    a = AStar('start', 'D')
    res = a.run(graph)
    print(res)

