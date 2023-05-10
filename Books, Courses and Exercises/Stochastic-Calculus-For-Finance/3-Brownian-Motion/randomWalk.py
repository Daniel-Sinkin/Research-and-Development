import numpy as np
import matplotlib.pyplot as plt

rng = np.random.default_rng()
n = 1000

X = np.arange(n)
X = np.insert(X, 0, 0)
# 2(1) - 1 = 1, 2(0)-1 = -1
samples = 2 * rng.binomial(n = 1, p = 0.5, size = n) - 1
Y = np.cumsum(samples)
Y = np.insert(Y, 0, 0)

mean = np.mean(Y)
var = np.var(Y)
se = np.sqrt(var)

max = np.max(Y)
min = np.min(Y)

plt.plot(X, Y)

plt.axhline(y=mean, color='g')
plt.axhline(y=mean - se, color='r', linestyle = "dotted")
plt.axhline(y=mean + se, color='r', linestyle = "dotted")
plt.axhline(y=mean - 2 * se, color='r', linestyle = "dotted")
plt.axhline(y=mean + 2 * se, color='r', linestyle = "dotted")

plt.show()
