import os
import strutils
import strscans

var inputData = open(paramStr(1)).readAll().strip().splitLines()

var correctPasswordsP1 = 0
var correctPasswordsP2 = 0

for line in inputData:
  var lower, upper : int
  var char_str : string
  var password : string
  if scanf(line, "$i-$i $+: $+", lower, upper, char_str, password):
    let c = char_str[0]
    let count = password.count(c)
    if count >= lower and count <= upper:
      correctPasswordsP1 += 1

    # xor
    if (password[lower - 1] == c) != (password[upper - 1] == c):
      correctPasswordsP2 += 1


echo "Number of correct passwords P1: ", $(correctPasswordsP1)
echo "Number of correct passwords P2: ", $(correctPasswordsP2)
