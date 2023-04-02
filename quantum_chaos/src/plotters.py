import matplotlib.pyplot as plt
import numpy as np

plt.rc('text', usetex=True)
plt.rc('font', family='serif')
plt.rcParams['text.latex.preamble'] = [
    r'\usepackage{siunitx}',  # consistent units formatting
    r'\sisetup{detect-all}',  # current font settings for units
    r'\usepackage{amsmath}',  # additional math environments
    r'\usepackage{amssymb}',  # additional math symbols
    r'\usepackage{bm}',  # bold math symbols
    r'\usepackage{mathrsfs}',  # math script fonts
]


def plot_energies(filename: str, bins: int):
    data = np.loadtxt(filename + ".txt")

    plt.hist(data, bins=bins)
    plt.xlabel("Energy of the well")
    plt.ylabel("Frequency")
    plt.savefig(filename + '.pdf', dpi=300)
    plt.show()

def plot_energy_differences(filename: str, bins: int):
    data = np.sort(np.loadtxt(filename+".txt"))
    # differencec between neighbors
    diff = data[1:-1]-data[0:-2]

    plt.hist(diff, bins=bins)
    plt.xlabel("S")
    plt.ylabel("Frequency")
    plt.savefig(filename+'_dif.pdf', dpi=300)
    plt.show()