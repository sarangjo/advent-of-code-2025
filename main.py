#!/usr/bin/env python3

import sys


sample = False


def day1():
    with open(f"{"sample" if sample else "day"}1.txt") as f:
        lines = f.readlines()

    def part1(val: int, direction: int, amount: int) -> tuple[int, int]:
        val = (val + direction * amount) % 100

        return val, 1 if val == 0 else 0

    def part2(val: int, direction: int, amount: int) -> tuple[int, int]:
        new_val = val + direction * amount
        cross_times = abs((new_val // 100) - (val // 100))
        new_val %= 100

        if new_val == 0 and direction == -1:
            # If we land at 0 and we were direction == -1, then we need to add 1 to cross_times
            cross_times += 1
        elif val == 0 and direction == -1:
            # If we started at 0 and we were direction == -1, then we need to subtract 1 to cross_times
            cross_times -= 1

        return new_val, cross_times

    def run(fn):
        count = 0

        val = 50
        for l in lines:
            direction = -1 if l[:1] == 'L' else 1
            amount = int(l.strip()[1:])

            val, count_change = fn(val, direction, amount)
            count += count_change

        print(count)

    run(part1)
    run(part2)


if __name__ == "__main__":
    if len(sys.argv) > 2 and sys.argv[2] == "--sample":
        sample = True

    globals()[f"day{sys.argv[1]}"]()
