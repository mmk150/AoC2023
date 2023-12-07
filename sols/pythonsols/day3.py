from aoclib import get_input, get_tests
import sys


def part1(input):
    arr = []
    for line in input:
        line = line.strip()
        second_arr = []
        for character in line:
            second_arr.append(character)
        arr.append(second_arr)
    return add_sym_arr(arr)


def part2(input):
    pass


def test(day_num):
    input = get_tests(day_num)
    res1 = part1(input)
    res2 = part2(input)
    return res1, res2


def solve(day_num):
    input = get_input(day_num)
    res1 = part1(input)
    res2 = part2(input)
    return res1, res2


if __name__ == "__main__":
    daynum = sys.argv[1]
    test1, test2 = test(daynum)
    ans1, ans2 = solve(daynum)
    print(f"Part a test:{test1}")
    print(f"Part a:{ans1}")
    print(f"Part b:{ans2}")
