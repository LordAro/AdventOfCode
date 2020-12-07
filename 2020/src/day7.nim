import math
import os
import parseutils
import sequtils
import sets
import strutils
import tables

type
  Colour = string
  Bags = Table[Colour, seq[Colour]]
  BagMap = Table[Colour, seq[(int, Colour)]]

var
  containingBags: Bags
  bagMap: BagMap

for line in paramStr(1).lines:
  var primaryColour, contents: string
  let i = parseUntil(line, primaryColour, " bags")
  let j = " bags contain ".len
  discard parseUntil(line, contents, ".", start=i+j)

  bagMap[primaryColour] = @[]

  for content in contents.split(", "):
    if content == "no other bags":
      break
    var num: int
    var colour: Colour
    let i = parseInt(content, num)
    discard parseUntil(content, colour, " bag", start=i+1)

    containingBags.mgetOrPut(colour, @[]).add(primaryColour)
    bagMap[primaryColour].add((num, colour))

var outerBags: HashSet[Colour]
var bagsToCheck = @["shiny gold"]

while bagsToCheck.len > 0:
  let col = bagsToCheck.pop
  for c in containingBags.getOrDefault(col):
    bagsToCheck.add(c)
    outerBags.incl(c)

echo "Number of outer bags: ", $(outerBags.len)

proc GetBagCount(bagMap : BagMap, col: Colour): int =
  if col notin bagMap:
    return 0
  let containingBags = bagMap[col]
  containingBags.mapIt(it[0] * (GetBagCount(bagMap, it[1]) + 1)).sum

echo "Bag count: ", $(GetBagCount(bagMap, "shiny gold"))
