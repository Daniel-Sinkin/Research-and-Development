import numpy as np
import matplotlib.pyplot as plt

rng = np.random.default_rng()

def SymmetricScaledRandomWalk(n = 400, scaling_factor = 1, draw = True, saveName = ""):
    t = np.arange(start = 0, stop = n / scaling_factor, step = 1 / scaling_factor)
    samples = 2 * rng.binomial(n = 1, p = 0.5, size = len(t)) - 1
    W = np.cumsum(samples) / np.sqrt(scaling_factor)

    plt.plot(t, W)

    if saveName != "":
        plt.savefig(f"{saveName}.png")
    if draw:
        plt.show()

SymmetricScaledRandomWalk(400, 1, draw = True)
SymmetricScaledRandomWalk(400, 100, draw = True, saveName="ScaledSymmetricRandomWalk")
