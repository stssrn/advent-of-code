#!/usr/bin/python

import os
import sys
import math

input_file = sys.argv[1]
mass_list = []

def fuelRequirements(mass):
    fuel_list = []
    fuel_list.append(mass)
    while fuel_list[-1] >= 0:
        calced = math.floor(int(fuel_list[-1]) / 3 - 2)
        fuel_list.append(calced)
    return sum(fuel_list[1:-1])

with open(input_file, 'r') as f:
    for mass in f:
        try:
            mass_list.append(int(mass))
        except:
            pass

calced_list = []
for mass in mass_list:
    calced_list.append(fuelRequirements(mass))
print(sum(calced_list))

