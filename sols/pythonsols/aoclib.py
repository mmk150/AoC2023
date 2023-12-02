inputs_path = "../../inputs"


def get_input(day_num):
    path_actual = inputs_path + f"/day{day_num}.txt"
    file = open(path_actual, "r", encoding="utf-8")
    lines = file.readlines()
    return lines
