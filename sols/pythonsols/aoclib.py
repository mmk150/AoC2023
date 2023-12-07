inputs_path = "../../inputs"
tests_path = "../../tests"


def get_input(day_num):
    path_actual = inputs_path + f"/day{day_num}.txt"
    file = open(path_actual, "r", encoding="utf-8")
    lines = file.readlines()
    return lines


def get_tests(day_num):
    path_actual = tests_path + f"/day{day_num}.txt"
    file = open(path_actual, "r", encoding="utf-8")
    lines = file.readlines()
    return lines
