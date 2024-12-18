import numpy as np



input = None
with open('./input.txt', 'r') as f:
    input = f.readlines()

def process(input, values):
    if len(values) == 0:
        return input
    return [process(input + values[0], values[1:]), process(input * values[0], values[1:]), process(int(str(input) + str(values[0])), values[1:])]

count = 0
targets = []
for line in input:
    line = line.replace("\n", "")
    target, values = int(line.split(":")[0]), [int(x) for x in line.split(":")[1].split(' ') if x != '']
    print(target, values)
    array = np.array(process(values[0], values[1:])).flatten()
    print(array)
    if np.any(array == target):
        count += 1
        targets.append(target)
print(sum(targets))