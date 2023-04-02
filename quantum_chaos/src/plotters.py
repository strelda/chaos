import matplotlib.pyplot as plt
import numpy as np



def plot_energies(filename: str, bins: int):
    data = np.loadtxt(filename + ".txt")

    plt.hist(data, bins=bins)
    plt.xlabel("Energy of the well")
    plt.ylabel("Frequency")
    plt.savefig(filename + '.png', dpi=300)
    plt.show()


def plot_energy_differences(filename: str, bins: int):
    data = np.loadtxt(filename+".txt")

    plt.hist(data, bins=bins)
    plt.xlabel("S")
    plt.ylabel("Frequency")
    plt.savefig(filename+'.png', dpi=300)
    plt.show()