import os
import sequtils
import strutils
import tables
#import nimprof

let inputData = open(paramStr(1)).readAll.strip.mapIt(ord(it) - ord('0'))

proc playCrabGame(startCups: seq[int], numCups: int, numRounds: int): Table[int, int] =
  # Set up ring as a table
  # Use a table so we've got O(1) searching
  var cups: Table[int, int] # n pointing to first
  for i in 1 .. startCups.high:
    cups[startCups[i - 1]] = startCups[i]
  var prev = startCups[startCups.high]
  for i in startCups.len + 1 .. numCups:
    cups[prev] = i
    prev = i
  cups[prev] = startCups[0] # create the ring

  var currentCup = startCups[0]

  for i in 1 .. numRounds:
    let pickUp1 = cups[currentCup]
    let pickUp2 = cups[pickUp1]
    let pickUp3 = cups[pickUp2]
    let linkVal = cups[pickUp3]
    cups[currentCup] = linkVal # link past the pickup

    var destCup = currentCup - 1
    if destCup == 0:
      destCup = numCups
    while destCup == pickUp1 or destCup == pickUp2 or destCup == pickUp3:
      dec destCup
      if destCup == 0:
        destCup = numCups

    # Re-insert the pickup into the ring
    let oldLinkVal = cups[destCup]
    cups[destCup] = pickUp1
    cups[pickUp3] = oldLinkVal

    currentCup = cups[currentCup]

  return cups

let p1Ring = playCrabGame(inputData, inputData.len, 100)

var r: string
var val = p1Ring[1]
while val != 1:
  r &= chr(ord('0') + val)
  val = p1Ring[val]

echo "Cup labels after 100 rounds with 9 cups: ", r

let p2Ring = playCrabGame(inputData, 1_000_000, 10_000_000)
var oneVal = p2Ring[1]
echo "Cup product after lots of rounds with many cups: ", oneVal * p2Ring[oneVal]
