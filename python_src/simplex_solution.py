import numpy as np
from pulp import *

# 定数
C: np.ndarray = np.array([2, 1]).T
E: np.ndarray[int] = np.array([2, 3]).T
B: np.ndarray[int] = np.array([10, 5]).T
PI_0: np.ndarray[float] = np.array([
    [0., .5],
    [.5, 0.],
])
PI_1: np.ndarray[float] = np.array([
    [0., .5],
    [.5, 0.],
])
M = 5
N = len(E)

def main():
    p1: np.ndarray = np.array([LpVariable(f"p1_{i}", lowBound=0) for i in range(N)]).T
    x: np.ndarray = np.array([LpVariable(f"x_{i}", lowBound=0) for i in range(N)]).T

    prob = LpProblem("Problem", LpMaximize)

    expr_vec = p1 - PI_1 @ p1 - x
    for i in range(N):
        prob += expr_vec[i] <= E[i]
    
    for i in range(N):
        prob += p1[i] <= B[i]
    
    prob += lpSum(x) <= M

    prob += lpSum(C * p1)

    status = prob.solve()

    print({
        "result": C @ np.array([value(p1[i]) for i in range(N)]).T,
        "x": [value(x[i]) for i in range(N)],
        "p1": [value(p1[i]) for i in range(N)],
    })

if __name__ == "__main__":
    main()