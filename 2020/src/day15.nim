import os
import strutils
import sequtils

let inputData = open(paramStr(1)).readAll.strip.split(",").map(proc(x: string): int = parseInt(x))

const LIMIT = 30_000_000

var seenNums = newSeq[int](LIMIT)

for i, v in inputData:
  seenNums[v] = i + 1 # one-based turn num

seenNums[inputData[^1]] = 0  # reset last value as we don't want to insert it yet

var
  lastNum = inputData[^1]
  turnNum = inputData.len

while turnNum < LIMIT:
  let lastNumWasNew = seenNums[lastNum] == 0
  let newNum = (
    if lastNumWasNew:
      0
    else:
      turnNum - seenNums[lastNum]
  )
  seenNums[lastNum] = turnNum
  lastNum = newNum
  inc turnNum
  if turnNum == 2020:
    echo "2020th number: ", lastNum

echo "30_000_000th number: ", lastNum
