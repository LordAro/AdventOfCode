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
    var
      sum = c
      sumNumMax = c
      sumNumMin = c
    for d in previousNums[i + 1 .. ^1]:
      sum += d
      sumNumMin = min(sumNumMin, d)
      sumNumMax = max(sumNumMax, d)
      if sum == firstInvalidNum:
        echo "Encryption weakness pair sum: ", sumNumMin + sumNumMax
        break outer
      if sum > firstInvalidNum:
        break
