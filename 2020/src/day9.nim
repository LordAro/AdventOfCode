import os
import strutils

var previousNums: seq[int64]
var firstInvalidNum:int64 = 0

for line in paramStr(1).lines:
  let num = parseBiggestInt(line)
  # preamble
  if previousNums.len() < 25:
    previousNums.add(num)
    continue

  var found = false
  block outer:
    for i, prevA in previousNums[^25 .. ^1]:
      for j, prevB in previousNums[^25 .. ^1]:
        if i == j:
          continue
        if prevA + prevB == num:
          found = true
          break outer
  if found:
    previousNums.add(num)
  else:
    firstInvalidNum = num
    break

echo "Found invalid number: ", firstInvalidNum

block outer:
  for i, c in previousNums:
    var sumNums = @[c]
    var sum = c
    for d in previousNums[i + 1 .. ^1]:
      sum += d
      sumNums.add(d)
      if sum == firstInvalidNum:
        echo "Encryption weakness pair: ", sumNums.min, ", ", sumNums.max, " = ", sumNums.min + sumNums.max
        break outer
      if sum > firstInvalidNum:
        break
