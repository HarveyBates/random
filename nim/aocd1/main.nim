import std/strutils

let file = readFile("input.txt").strip()
var rows = file.splitLines()

proc solve_one(): int = 
  for row in rows:
    for cmp in rows[1 .. ^1]:
      let irow = parseInt(row) 
      let icmp = parseInt(cmp)
      let sum = irow + icmp
      if (sum == 2020):
        return irow * icmp

proc solve_two(): int =
  for row in rows:
    for cmp in rows[1 .. ^1]:
      for cmp2 in rows[2 .. ^1]:
        let irow = parseInt(row) 
        let icmp = parseInt(cmp)
        let icmp2 = parseInt(cmp2)
        let sum = irow + icmp + icmp2
        if (sum == 2020):
          return irow * icmp * icmp2

echo solve_one()
echo solve_two()
