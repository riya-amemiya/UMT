import numpy as np


def softmax(x) -> np.ndarray:
    max = np.max(x)
    exp = np.exp(x - max)
    return exp / np.sum(exp)

print(softmax(np.array([1.0, 2.0, 4.0])))
