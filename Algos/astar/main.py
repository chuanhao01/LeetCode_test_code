import queue

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
        done = False
        self.open_queue.put((0, self.start_key, None))
        while not done:
            # Pops the top node in the open nodes
            cur_weight, cur_key, _ = self.open_queue.get()
            cur_node = graph.get_node()
            # For each edge from the node
            for edge in cur_node.edges:
                target_node, weight = edge
                if target_node.key not in self.open_scores:
                    # Check if node is in the open list
                        self.open_scores[target_node.key] = [weight]
                    # F G T F for AStar
                    # F T F for normal algo
                    self.open_queue.put((weight, target_node.key, cur_node.key))
                else:
                    # Node is already in the list
                    # Check if this path is shorter









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

