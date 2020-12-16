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

#for line in [
#"class: 1-3 or 5-7",
#"row: 6-11 or 33-44",
#"seat: 13-40 or 45-50",
#"",
#"your ticket:",
#"7,1,14",
#"",
#"nearby tickets:",
#"7,3,47",
#"40,4,50",
#"55,2,20",
#"38,6,12",
#]:
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
  validTickets = @[myTicket]

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

iterator iota(n: int): int =
  for i in 0 ..< n:
    yield i

var possibleKeys: Table[string, seq[int]]
for fieldName in fieldRanges.keys:
  possibleKeys[fieldName] = toSeq(iota(myTicket.len))

for ticket in validTickets:
  for i, v in ticket:
    for field, fieldRange in fieldRanges:
      if v notin fieldRange[0][0] .. fieldRange[0][1] and v notin fieldRange[1][0] .. fieldRange[1][1]:
        possibleKeys[field].del(possibleKeys[field].find(i))

var actualKeys: Table[string, int]

while possibleKeys.len != 0:
  var
    oneElementName: string
    oneElementIdx: int
  for key, value in possibleKeys:
    if value.len == 1:
      oneElementName = key
      oneElementIdx = value[0]
      break
  possibleKeys.del(oneElementName)
  for value in possibleKeys.mvalues:
    value.del(value.find(oneElementIdx))
  actualKeys[oneElementName] = oneElementIdx

var departureMult = 1
for key, idx in actualKeys:
  if key.startsWith("departure"):
    departureMult *= myTicket[idx]

echo "Departure multiplication: ", departureMult
