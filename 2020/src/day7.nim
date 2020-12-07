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
  let idx = line.find(" bags contain ")
  if idx == -1:
    continue
  let primaryColour = line[0 .. idx - 1]

  let slicePoint = idx + len(" bags contain ")
  bagMap[primaryColour] = @[]
  var inBag = 0
  var num = 0
  var containedColour = "";
  for w in line[slicePoint .. ^3].split:
    if inBag == 2:
      containedColour &= " " & w

      containingBags.mgetOrPut(containedColour, @[]).add(primaryColour)
      bagMap[primaryColour].add((num, containedColour))

      inBag = 0
      num = 0
    elif inBag == 1:
      containedColour = w
      inBag = 2
    elif w[0] in '0' .. '9':
      num = int(w[0]) - int('0')
      inBag = 1

var outerBags: HashSet[BagColour]
var bagsToCheck = @["shiny gold".BagColour]

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

echo "Bag count: ", $(GetBagCount(bagMap, "shiny gold".BagColour))
