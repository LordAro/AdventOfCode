import math
import os
import strutils
import sequtils
import sets

let inputData = open(paramStr(1)).readAll.split("\n\n").mapIt(it.strip.split.mapIt(it.toHashSet))

let p1 = inputData.mapIt(it.foldl(a + b).len).sum
let p2 = inputData.mapIt(it.foldl(a * b).len).sum

echo "Set union: ", $(p1)
echo "Set intersection: ", $(p2)
