import os
import nre
import sequtils
import tables
import sets

let containingBagRE = re"[0-9] (\w+ \w+) bags?"
let primaryColourRE = re"([a-z]+ [a-z]+)"

type
  BagColour = string
  Bags = Table[BagColour, seq[BagColour]]

var containingBags: Bags

for line in paramStr(1).lines:
  let primaryColourMatch = find(line, primaryColourRE) # first 2 words are primary colour
  if primaryColourMatch.isSome(): # trailing newline
    let primaryColour = primaryColourMatch.get.captures[0]
    let colours = findAll(line, containingBagRE).mapIt(find(it, primaryColourRE).get.captures[0]) # get rid of number and 'bag'
    for c in colours:
      containingBags.mgetOrPut(c, @[]).add(primaryColour)

var outerBags: HashSet[BagColour]
var bagsToCheck = @["shiny gold".BagColour]

while bagsToCheck.len > 0:
  let col = bagsToCheck.pop
  for c in containingBags.getOrDefault(col):
    bagsToCheck.add(c)
    outerBags.incl(c)

echo "Number of outer bags: ", $(outerBags.len)
