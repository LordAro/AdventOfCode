import os
import strutils
import sequtils
import sets

let inputData = open(paramStr(1)).readAll.split("\n\n").map(proc(answerList: string): seq[HashSet[char]] =
  answerList.strip.split.mapIt(it.toHashSet)
)

let p1 = inputData.mapIt(it.foldl(a + b)).mapIt(it.len).foldl(a + b)
let p2 = inputData.mapIt(it.foldl(a * b)).mapIt(it.len).foldl(a + b)

echo "Set union: ", $(p1)
echo "Set intersection: ", $(p2)
