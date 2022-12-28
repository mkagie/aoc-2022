"""Part 1 and 2"""
from argparse import ArgumentParser
from dataclasses import dataclass
import numpy as np
import matplotlib.pyplot as plt

def parse_args():
    parser = ArgumentParser()
    parser.add_argument("-i", "--input", type=str, required=True, help="Input file")

    return parser.parse_args()

@dataclass
class Droplet:
    x: int
    y: int
    z: int

def droplet_from_line(input: str):
    numbers = input.split(",")
    return Droplet(int(numbers[0]), int(numbers[1]), int(numbers[2]))



def main(args):
    with open(args.input, "r") as fid:
        lines = fid.readlines()
    droplets = []
    for line in lines:
        droplets.append(droplet_from_line(line))

    x = []
    y = []
    z = []
    for droplet in droplets:
        x.append(droplet.x)
        y.append(droplet.y)
        z.append(droplet.z)

    fig = plt.figure()
    ax = fig.add_subplot(projection="3d")
    ax.scatter(x, y, z)
    plt.show()


if __name__ == '__main__':
    args = parse_args()
    main(args)
