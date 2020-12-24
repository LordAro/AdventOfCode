import lists
import os
import sequtils
import strutils
import times

#for c in [3,8,9,1,2,5,4,6,7]:
let inputData = open(paramStr(1)).readAll.strip.mapIt(ord(it) - ord('0'))

proc playCrabGame(startCups: seq[int], numCups: int, numRounds: int): DoublyLinkedRing[int] =
  var
    cups = initDoublyLinkedRing[int]()

  for c in startCups:
    cups.append(c)
  for c in startCups.len + 1 .. numCups:
    cups.append(c)

  var currentCup = cups.head
  var pickUp: seq[DoublyLinkedNode[int]] # store the node, to save reallocations

  for i in 1 .. numRounds:
    if i mod 1000 == 0:
      echo "Round ", i
    for i in 1 .. 3:
      pickUp.add(currentCup.next)
      cups.remove(currentCup.next)

    var destCupVal = currentCup.value - 1
    if destCupVal == 0:
      destCupVal = 9
    while destCupVal in pickUp.mapIt(it.value):
      dec destCupVal
      if destCupVal == 0:
        destCupVal = numCups # max val
    let destCup = cups.find(destCupVal)

    while pickUp.len > 0:
      let n = pickUp.pop
      n.next = destCup.next
      n.prev = destCup
      destCup.next.prev = n
      destCup.next = n
    #let now1 = now()
    #if i mod 1000 == 0:
    #  echo "Find time: ", now() - now1

    currentCup = currentCup.next

  return cups

let p1Ring = playCrabGame(inputData, inputData.len, 100)

var r: string
var val = p1Ring.find(1).next
while val.value != 1:
  r &= chr(ord('0') + val.value)
  val = val.next

echo "Cup labels after 100 rounds with 9 cups: ", r

let p2Ring = playCrabGame(inputData, 1_000_000, 10_000_000)
var oneVal = p2Ring.find(1)
echo oneVal.next.value, oneVal.next.next.value
