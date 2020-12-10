import algorithm
import os
import strutils

#var inputData = @[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
var inputData = @[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]
#var inputData: seq[int] = @[]
#for line in paramStr(1).lines:
#  inputData.add(parseInt(line))
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

var memo = newSeq[int](inputData.len())

proc CountCombinations(arr: seq[int], i: int): int =
  if memo[i] != 0:
    return memo[i]
  echo i, " ", arr[i]
  if i == arr.high():
    result = 1
  else:
    result = CountCombinations(arr, i + 1)
    if i < arr.len() - 2 and arr[i + 2] <= arr[i] + 3:
      result += CountCombinations(arr, i + 2)
    if i < arr.len() - 3 and arr[i + 3] <= arr[i] + 3:
      result += CountCombinations(arr, i + 3)
  memo[i] = result
  return result

echo "Total possible arrangements: ", CountCombinations(inputData, 0)
