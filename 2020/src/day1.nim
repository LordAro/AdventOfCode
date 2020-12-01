import os
import strutils
import sequtils

let inputData = open(paramStr(1)).readAll().strip().splitLines().map(parseInt)

var matchedI = 0
var matchedJ = 0

block outer:
    for i in low(inputData) .. high(inputData) - 1:
        for j in i + 1 .. high(inputData):
            if inputData[i] + inputData[j] == 2020:
                matchedI = i
                matchedJ = j
                break outer

let pairProduct = inputData[matchedI] * inputData[matchedJ]
echo "Matched pair: ", $(pairProduct)

var matchedK = 0

block outer:
    for i in low(inputData) .. high(inputData) - 2:
        for j in i + 1 .. high(inputData) - 1:
            for k in j + 1 .. high(inputData):
                if inputData[i] + inputData[j] + inputData[k] == 2020:
                    matchedI = i
                    matchedJ = j
                    matchedK = k
                    break outer

let tripleProduct = inputData[matchedI] * inputData[matchedJ] * inputData[matchedK]
echo "Matched triple: ", $(tripleProduct)
