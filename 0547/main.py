from typing import List
from collections import deque 

class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        counter = 0
        for node in range(len(isConnected)):
            if isConnected[node][node]:
                counter += 1
                Solution.bfs(node, isConnected)
                
        return counter
        
    def bfs(start, matrix):
        q = deque()
        q.append(start)
        
        while len(q) > 0:
            curr = q.pop()
            matrix[curr][curr] = 0
            
            for node, connected in enumerate(matrix[curr]):
                if connected and matrix[node][node]:
                    q.append(node)
                    
if __name__ == "__main__":
    pass