import os
import strscans
import strutils

var correctPasswordsP1 = 0
var correctPasswordsP2 = 0

for line in paramStr(1).lines:
  var lower, upper: int
  var char_str, password: string
  if scanf(line, "$i-$i $w: $w", lower, upper, char_str, password):
    let c = char_str[0]
    if password.count(c) in lower..upper:
      inc correctPasswordsP1

    if (password[lower - 1] == c) xor (password[upper - 1] == c):
      inc correctPasswordsP2


echo "Number of correct passwords P1: ", $(correctPasswordsP1)
echo "Number of correct passwords P2: ", $(correctPasswordsP2)
