import std/strutils
from std/sequtils import map
import std/math

let file = readFile("input.txt").strip()
var rows = file.splitLines().map(parseInt)


proc solve_one(): int = 
  var sum = 0
  for row in rows:
    var mass = int(floor(row / 3) - 2)
    sum = sum + mass
  return sum


proc solve_two(): int = 
  var sum = 0
  for mass in rows:
    var fuel = int(floor(mass / 3) - 2)
    var fuel_mass = int(floor(fuel / 3)  - 2)
    var total_fuel: int = fuel
    while (fuel_mass > 0):
      total_fuel = total_fuel + fuel_mass
      fuel_mass = int(floor(fuel_mass / 3)  - 2)
    sum = sum + total_fuel
  return sum


echo solve_one()
echo solve_two()

