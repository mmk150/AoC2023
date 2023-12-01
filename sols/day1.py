from aoclib import get_input
import sys


def part1(input):
    sum = 0
    for line in input:
        line = line.strip()
        first = -99
        last = -99
        for c in line:
            if c.isdigit() and first == -99:
                first = c
            if c.isdigit():
                last = c
        sum += int(first + last)
    return sum


def part2(input):
    sum = 0
    lst = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ]

    for line in input:
        line = line.strip()
        first = -99
        last = -99
        nums = []
        for i in range(len(line)):
            for j in range(i + 1, len(line) + 1):
                word = line[i:j]
                if word in lst or word.isdigit():
                    nums.append(word)
                    i = j
        if nums == []:
            continue
        first = nums[0]
        last = nums[-1]
        if first.isdigit():
            pass
        else:
            first = str(lst.index(first))
        if last.isdigit():
            pass
        else:
            last = str(lst.index(last))
        print(line)
        print(nums)
        print(first, last)
        print(sum)
        sum += int(first + last)
        print(sum)
        print("\n")

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
