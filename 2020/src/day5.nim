import algorithm
import os
import sequtils

var seatIds = toSeq(paramStr(1).lines).map(proc(pass: string): int =
  let row = pass[0 .. ^4]
  let col = pass[^3 .. ^1]

  var lb = 0
  var ub = 128
  for c in row:
    if c == 'F':
      ub = (lb + ub) div 2
    elif c == 'B':
      lb = (lb + ub) div 2

  let row_num = lb

  lb = 0
  ub = 8
  for c in col:
    if c == 'L':
      ub = (lb + ub) div 2
    elif c == 'R':
      lb = (lb + ub) div 2

  let col_num = lb

  row_num * 8 + col_num
)
seatIds.sort()

echo "Max seat ID: ", $(seatIds[^1])

for i in 1 .. len(seatIds) - 1:
  if seatIds[i - 1] + 2 == seatIds[i]:
    echo "My seat ID: ", $(seatIds[i - 1] + 1)
    break
