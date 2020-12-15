import os
import strutils
import sequtils
import tables

let inputData = open(paramStr(1)).readAll.strip.split(",").map(proc(x: string): int = parseInt(x))
#let inputData = @[0, 3, 6]

var
  seenNums: Table[int, seq[int]]

for i, v in inputData:
  seenNums[v] = @[i]

var
  lastNum = inputData[^1]
  turnNum = inputData.len

while turnNum < 30_000_000:
  let lastNumWasNew = seenNums[lastNum].len == 1
  let newNum = (
    if lastNumWasNew:
      0
    else:
      seenNums[lastNum][^1] - seenNums[lastNum][^2]
  )
  if seenNums.contains(newNum):
    if seenNums[newNum].len == 2:
      seenNums[newNum][0] = seenNums[newNum][1]
      seenNums[newNum][1] = turnNum
    else:
      seenNums[newNum].add(turnNum)
  else:
    seenNums[newNum] = @[turnNum]
  lastNum = newNum
  inc turnNum
  if turnNum == 2020:
    echo "2020th number: ", lastNum

echo "30_000_000th number: ", lastNum
