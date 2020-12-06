import algorithm
import os

var seatIds: seq[int] = @[]

for pass in paramStr(1).lines:
  var pass_no = 0
  for i, c in pass:
    pass_no = pass_no or ord(c == 'B' or c == 'R') shl (pass.len - i - 1)

  # row is first 7 bits
  # col is last 3 bits
  # seat id is row * 8 + col, ...which is just the whole 10bit number
  seatIds.add(pass_no)

seatIds.sort()

echo "Max seat ID: ", $(seatIds[^1])

for i in 1 .. high(seatIds):
  if seatIds[i - 1] + 1 != seatIds[i]:
    echo "My seat ID: ", $(seatIds[i - 1] + 1)
    break
