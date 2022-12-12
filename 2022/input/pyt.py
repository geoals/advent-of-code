command_list = open('sample_10.txt', 'r').readlines()


def solution1(commands):
    cycle_values = [1]

    for command in commands:
        if command.startswith("a"):
            for i in range(2):
                if i == 0:
                    cycle_values.append(cycle_values[-1])
                if i == 1:
                    cycle_values.append(cycle_values[-1] + int(command.split()[1]))
        elif command.startswith("n"):
            cycle_values.append(cycle_values[-1])

    score = sum([i * cycle_values[i - 1] for i in range(20, 240, 40)])
    return cycle_values, score


sprite, ans1 = solution1(command_list)
print(ans1)
