import os
import sys

inputfile = sys.argv[1]

with open(inputfile, "r") as f:
    loaded = f.read().rstrip().split(",")
    loaded = list(map(int, loaded))

def preprocessing():
    loaded[1] = 12
    loaded[2] = 2

def decoder(data):
    step = 0
    decoded = data
    allowed_steps = range(0, len(data), 4)
    for x in data:
        #print(str(step) + " " + str(x))
        if x == 1 and step in allowed_steps:
            decoded[data[step + 3]] = data[data[step + 1]] + data[data[step + 2]]
        elif x == 2 and step in allowed_steps:
            decoded[data[step + 3]] = data[step + 1] * data[step + 2]
        elif x == 99 and step in allowed_steps:
            break
        else:
            pass
        step += 1
    return decoded



def main():
    preprocessing()
    print(decoder(loaded))

main()
