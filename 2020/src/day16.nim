import bitops
import os
import sequtils
import strscans
import strutils
import tables

type
  Ticket = seq[int]
  Range = (int, int)
  FieldMap = OrderedTable[string, (Range, Range)]

var
  validValues = newSeq[bool](1000)
  fieldRanges: FieldMap
  myTicket: Ticket
  nearbyTickets: seq[Ticket]
  myTicketParse = false
  nearbyTicketParse = false

  # scanf vars
  field_name: string
  range1a, range1b, range2a, range2b: int

for line in paramStr(1).lines:
  if nearbyTicketParse:
    nearbyTickets.add(line.split(",").mapIt(parseInt(it)))
  elif myTicketParse:
    myTicket = line.split(",").mapIt(parseInt(it))
    myTicketParse = false  # just the one line
  elif scanf(line, "$+: $i-$i or $i-$i", field_name, range1a, range1b, range2a, range2b):
    fieldRanges[field_name] = ((range1a, range1b), (range2a, range2b))
    for i in range1a .. range1b:
      validValues[i] = true
    for i in range2a .. range2b:
      validValues[i] = true
  elif line == "your ticket:":
    myTicketParse = true
  elif line == "nearby tickets:":
    nearbyTicketParse = true

var
  invalidTicketSum = 0
  validTickets: seq[Ticket] = @[]

for ticket in nearbyTickets:
  var valid = true
  for val in ticket:
    if not validValues[val]:
      invalidTicketSum += val
      valid = false
      break # only 1 invalid field
  if valid:
    validTickets.add(ticket)

echo "Invalid ticket sum: ", invalidTicketSum

let bitMask = toMask[int32](0 .. myTicket.high)
var possibleKeys: Table[string, int32]
for fieldName in fieldRanges.keys:
  possibleKeys[fieldName] = bitMask

for ticket in validTickets:
  for i, v in ticket:
    for field, fieldRange in fieldRanges:
      if v notin fieldRange[0][0] .. fieldRange[0][1] and v notin fieldRange[1][0] .. fieldRange[1][1]:
        possibleKeys[field].clearBit(i)

var actualKeys: Table[string, int]

while possibleKeys.len != 0:
  var
    oneElementName: string
    oneElementIdx: int
    found = false
  for key, value in possibleKeys:
    if value.countSetBits == 1:
      oneElementName = key
      oneElementIdx = value.firstSetBit - 1
      found = true
      break
  if found:
    possibleKeys.del(oneElementName)
    for value in possibleKeys.mvalues:
      value.clearBit(oneElementIdx)
    actualKeys[oneElementName] = oneElementIdx
  else:
    raise newException(ValueError, "Could not find candidate key")

var departureMult = 1
for key, idx in actualKeys:
  if key.startsWith("departure"):
    departureMult *= myTicket[idx]

echo "Departure multiplication: ", departureMult
