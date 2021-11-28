import std/strutils
from std/sequtils import map

# Run with: nim c -r --verbosity:0 main.nim

# Read from file and strip last \n char (returns a string)
let file = readFile("input.txt").strip()
# Split the string and convert each line to an int with pasrseInt 
var rows = file.splitLines().map(parseInt)

proc solve_one(): int = 
  for row in rows:
    for cmp in rows[1 .. ^1]:
      let sum = row + cmp
      if (sum == 2020):
        return row * cmp


proc solve_two(): int =
  for row in rows:
    for cmp in rows[1 .. ^1]:
      for cmp2 in rows[2 .. ^1]:
        let sum = row + cmp + cmp2
        if (sum == 2020):
          return row * cmp * cmp2


echo solve_one()
echo solve_two()

