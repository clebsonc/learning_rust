import numpy as np


if __name__ == "__main__":
    # linkedlist = []
    # for i in range(0, 1_000_000):
    #     linkedlist.insert(0, i);
    
    array = np.empty(1_000_000, dtype=int);
    for i in np.arange(0, 1_000_000, 1):
        array[-(i+1)] = i

    print(list(array))
