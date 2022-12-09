#!/usr/bin/python

import os
import sys
import math

inputfile = sys.argv[1]
module_masses = []

def massToFuel(self, mass):
    if type(mass) is list:
        module_fuel = []
        for module_mass in mass:
            result = math.floor(int(module_mass) / 3 - 2)
            module_fuel.append(result)
        return module_fuel
    if type(mass) is int:
        result = math.floor(int(mass) / 3 - 2)
        module_masses.append(result)
        return module_masses
    else:
        pass

with open(inputfile, "r") as f:
    for mass in f:
        try:
            massToFuel(int(mass))
        except:
            pass
        
print("Part 1: {} amount of fuel required.".format(sum(module_masses)))

def additionalFuel(mass):
    iterations = 0
    iterated_mass = massToFuel()
    while sum(mass) >= 0:
        massToFuel(mass)
        iterations += 1
    else:
        return mass, iterations

print("Part 2: Whoops! there is {} amount of fuel required ({} iterations)".format(additionalFuel(module_masses)))
