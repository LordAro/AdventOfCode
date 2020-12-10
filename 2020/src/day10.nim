import algorithm
import os
import strutils

var inputData: seq[int] = @[]
for line in paramStr(1).lines:
  inputData.add(parseInt(line))
inputData.sort()

var
  prev = 0
  jolt1Diff = 0
  jolt3Diff = 1 # final difference is always 3
for i, v in inputData:
  if v == prev + 1:
    inc jolt1Diff
  elif v == prev + 3:
    inc jolt3Diff
  prev = v

echo "Sum of jolt differences: ", jolt1Diff, " * ", jolt3Diff, " = ", jolt1Diff * jolt3Diff

# Uses -1 to represent the 0th element (i.e. starting point)
var memo = newSeq[int](inputData.len() + 1)

proc CountCombinations(arr: seq[int], i: int): int =
  if memo[i + 1] != 0:
    return memo[i + 1]
  if i == arr.high():
    result = 1
  else:
    let val = (if i == -1: 0 else: arr[i])
    result = CountCombinations(arr, i + 1)
    if i < arr.len() - 2 and arr[i + 2] <= val + 3:
      result += CountCombinations(arr, i + 2)
    if i < arr.len() - 3 and arr[i + 3] <= val + 3:
      result += CountCombinations(arr, i + 3)
  memo[i + 1] = result
  return result

echo "Total possible arrangements: ", CountCombinations(inputData, -1)
