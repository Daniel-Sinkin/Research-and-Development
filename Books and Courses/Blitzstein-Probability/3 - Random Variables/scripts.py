import numpy as np
import matplotlib.pyplot as plt
import math

def plotBinomial(n = 10, p = 1/8, size = 100000, show = True, save = False):
    distr  = np.random.binomial(n = n, p = p, size = size)
    x, y = np.unique(distr, return_counts=True)
    y = y / size

    plt.scatter(x, y, marker = 'o', color = 'black')
    plt.ylim(bottom=0, top = 0.4)
    plt.vlines(x, ymin=0, ymax=y, color = 'black')

    if save:
        plt.savefig(f'bin_{n}_{math.floor(p * 100)}pct.png', dpi = 300)

    if show:
        plt.show()