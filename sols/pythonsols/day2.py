from aoclib import get_input
import sys


def is_valid(test_dict):
    colors_dict = dict({"red": 12, "green": 13, "blue": 14})
    for key in colors_dict.keys():
        if test_dict[key] > colors_dict[key]:
            return False
    return True


def part1(input):
    sum = 0
    for i in range(len(input)):
        line = input[i]
        curr_dict = dict({"red": 0, "green": 0, "blue": 0})
        for segment in line.split(":")[-1].split(";"):
            for colors in segment.split(","):
                frag = colors.strip().split(" ")
                num = frag[0]
                color = frag[1]
                curr_dict[color] = max(int(num), curr_dict[color])
        if is_valid(curr_dict):
            sum += i + 1
    return sum


def part2(input):
    sum = 0
    for i in range(len(input)):
        line = input[i]
        curr_dict = dict({"red": 0, "green": 0, "blue": 0})
        for segment in line.split(":")[-1].split(";"):
            for colors in segment.split(","):
                frag = colors.strip().split(" ")
                num = frag[0]
                color = frag[1]
                curr_dict[color] = max(int(num), curr_dict[color])
        power = 1
        for val in curr_dict.values():
            power *= max(val, 1)
        sum += power
    return sum


def solve(day_num):
    input = get_input(day_num)
    res1 = part1(input)
    res2 = part2(input)
    return res1, res2


if __name__ == "__main__":
    daynum = sys.argv[1]
    ans1, ans2 = solve(daynum)
    print(f"Part a:{ans1}")
    print(f"Part b:{ans2}")
