
#[ Print hello world to stdout ]#
echo "\nHello World"

proc getSequence(t: string): string = 
  var accm = ""
  if t == "alpha":
    for letter in 'a' .. 'z':
      accm.add(letter)
  elif t == "nums":
    for number in '0' .. '9':
      accm.add(number)
  return accm

# Computed at compile time
const alphabet = getSequence("alpha")
const numbabet = getSequence("nums")
echo alphabet
echo numbabet

# var is mutable
var a = "foo"
a.add("bar")
echo a

# let is immutable
let b = "foo"
# b.add("bar") # error
echo b


#[ Using special "result" keyword (automatically returns value) ]#
proc getResult(): string = 
  for letter in 'a' .. 'z':
    result.add(letter)

echo getResult()


# Type casting
var x = 5.0 # float
var y = "foo" # string
# x = y compile time error

var p = int(x / 5)
echo p, " ", y

# if, elif, else
# continue starts next itteration of loop
# break exits loop

# case
# supports strings
# must go through every possible case (no break) :(
case p:
  of 1:
    echo "p = 1"
  else:
    echo "Lets hope not"
