import std/strutils
from std/sequtils import map

let file = readFile("input.txt").strip()
var rows = file.splitLines()

proc solve_one(): int = 
  var valid: int = 0
  for row in rows:
    let list = row.split(' ')
    let minMax = list[0].split('-').map(parseInt)
    let character = list[1].strip(chars={':'})
    var occ: int = 0
    for c in list[2]:
      if c == character[0]:
        inc(occ)
    if occ >= minMax[0] and occ <= minMax[1]:
      inc(valid)
  return valid

proc solve_two(): int = 
  var valid: int = 0
  for row in rows:
    let list = row.split(' ')
    let minMax = list[0].split('-').map(parseInt)
    let character = list[1].strip(chars={':'})
    if list[2][minMax[0] - 1] == character[0] and list[2][minMax[1] - 1] != character[0] or 
      list[2][minMax[0] - 1] != character[0] and list[2][minMax[1] - 1] == character[0]:
      inc(valid)
  return valid

echo solve_one()
echo solve_two()

