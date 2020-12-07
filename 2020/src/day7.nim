import os
import strutils
import sequtils
import tables
import math
import sets

type
  BagColour = string
  Bags = Table[BagColour, seq[BagColour]]
  BagMap = Table[BagColour, seq[(int, BagColour)]]

var
  containingBags: Bags
  bagMap: BagMap

for line in paramStr(1).lines:
  let lineWords = line.split
  if lineWords.len < 2:
    continue  # trailing newline

  let primaryColour = lineWords[0] & lineWords[1]
  bagMap[primaryColour] = @[]
  for i, w in lineWords[.. ^3]:
    if w[0] in '0' .. '9':
      let num = int(w[0]) - int('0')
      let containedColour = lineWords[i + 1] & lineWords[i + 2]
      containingBags.mgetOrPut(containedColour, @[]).add(primaryColour)
      bagMap[primaryColour].add((num, containedColour))

var outerBags: HashSet[BagColour]
var bagsToCheck = @["shinygold".BagColour]

while bagsToCheck.len > 0:
  let col = bagsToCheck.pop
  for c in containingBags.getOrDefault(col):
    bagsToCheck.add(c)
    outerBags.incl(c)

echo "Number of outer bags: ", $(outerBags.len)

proc GetBagCount(bagMap : BagMap, col: BagColour): int =
  if col notin bagMap:
    return 0
  let containingBags = bagMap[col]
  containingBags.mapIt(it[0] * (GetBagCount(bagMap, it[1]) + 1)).sum

echo "Bag count: ", $(GetBagCount(bagMap, "shinygold".BagColour))
