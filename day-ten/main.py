"""Day Ten -- In Python"""
from argparse import ArgumentParser
import numpy as np


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("-i", "--input", type=str, default="input.txt")
    parser.add_argument("-p", "--part", type=int, required=True)
    return parser.parse_args()


def part_one(args):
    # Read in the input
    with open(args.input, "r") as fid:
        lines = fid.readlines()

    stripped_lines = [line.strip() for line in lines]

    current_cmd = None
    addx_cycle_count = 0
    x = 1
    sum = 0
    for cycle_num in range(1, 221):
        if current_cmd is None:
            current_cmd = stripped_lines.pop(0)
            addx_cycle_count = 0

        # Increment cycle number
        if current_cmd.split()[0] == "noop":
            pass
        else:
            addx_cycle_count += 1

        # Do the middle part
        if cycle_num in [20, 60, 100, 140, 180, 220]:
            sum += x * cycle_num

        # Do the AFTER part
        if current_cmd.split()[0] == "noop":
            current_cmd = None
        else:
            if addx_cycle_count == 2:
                x += int(current_cmd.split()[1])
                current_cmd = None

    return sum


def part_two(args):
    # Read in the input
    with open(args.input, "r") as fid:
        lines = fid.readlines()

    stripped_lines = [line.strip() for line in lines]

    current_cmd = None
    addx_cycle_count = 0
    x = 1

    output = []  # Will eventually be an array
    for cycle_num in range(1, 241):
        if current_cmd is None:
            current_cmd = stripped_lines.pop(0)
            addx_cycle_count = 0

        # Increment cycle number
        if current_cmd.split()[0] == "noop":
            pass
        else:
            addx_cycle_count += 1

        # Do the middle part
        if abs(x - (cycle_num - 1) % 40) < 2:
            output.append("#")
        else:
            output.append(".")

        # Do the AFTER part
        if current_cmd.split()[0] == "noop":
            current_cmd = None
        else:
            if addx_cycle_count == 2:
                x += int(current_cmd.split()[1])
                current_cmd = None

    output = np.asarray(output).reshape((6, 40))
    np.set_printoptions(edgeitems=30, linewidth=100000)

    return output


if __name__ == "__main__":
    args = parse_args()
    if args.part == 1:
        print(part_one(args))
    elif args.part == 2:
        print(part_two(args))
    else:
        print("Bad part")
