import algorithm
import os
import sequtils
import strutils

var inputData = toSeq(paramStr(1).lines).map(parseInt)
inputData.sort()

var matchedI = 0
var matchedJ = 0

for i in 0 .. high(inputData):
  let foundIdx = binarySearch(inputData, 2020 - inputData[i])
  if foundIdx != -1:
    matchedI = i
    matchedJ = foundIdx

let pairProduct = inputData[matchedI] * inputData[matchedJ]
echo "Matched pair: ", $(pairProduct)

var matchedK = 0

block outer:
  for i in 0 .. high(inputData) - 1:
    for j in i + 1 .. high(inputData):
      let foundIdx = binarySearch(inputData, 2020 - (inputData[i] + inputData[j]))
      if foundIdx != -1:
        matchedI = i
        matchedJ = j
        matchedK = foundIdx
        break outer

let tripleProduct = inputData[matchedI] * inputData[matchedJ] * inputData[matchedK]
echo "Matched triple: ", $(tripleProduct)
