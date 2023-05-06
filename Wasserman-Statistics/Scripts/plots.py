# Plots always take up so much space in the pdf and don't contribute to the problem
# itself. As such it is better to just offload them to here.
import numpy as np
import matplotlib.pyplot as plt

def plot7_2_1(save = False):
    plt.title('Sample Mean Coverage Probability within 95% CI')
    plt.xlabel('True Proportion (q)')
    plt.ylabel('Coverage Probability')

    # Draws a dashed horizontal line at y = 0.95
    plt.axhline(0.95, linestyle='--', color='gray', alpha=0.6)

    # Custom y axis tick labels
    plt.yticks(np.array([0, 0.2, 0.4, 0.6, 0.8, 1]))

    # Set y-axis limits
    plt.ylim(0, 1)

    plt.legend()
    if save:
        plt.savefig("Ex7_2_1.png")
    plt.show()

def plot7_2_2(p, n, conf, save = False):
    plt.title(f'Sample Mean Coverage Probability within 95% CI\np = {p}, n = {n}')
    plt.xlabel('True Proportion (q)')
    plt.ylabel('Coverage Probability')

    # Draws a dashed horizontal line at y = 0.95
    plt.axhline(0.95, linestyle='--', color='gray', alpha=0.6)

    # Draws vertical lines at conf interval
    plt.axvline(conf[0], color='red', linestyle='--')
    plt.axvline(conf[1], color='red', linestyle='--')

    # Custom y axis tick labels
    plt.yticks(np.array([0, 0.2, 0.4, 0.6, 0.8, 0.95, 1]))
    # Custom x axis tick labels
    if conf[0] > 0.1:
        plt.xticks(np.array([0, conf[0], p, conf[1], 0.8, 1]))
    else:
        plt.xticks(np.array([conf[0], p, conf[1], 0.8, 1]))

    # Set y-axis limits
    plt.ylim(0, 1)

    plt.legend()
    if save:
        plt.savefig(f"Ex7_2_2-{n}.png")
    plt.show()