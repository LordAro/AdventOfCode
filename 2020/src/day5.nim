import algorithm
import os
import sequtils

let seatIds = toSeq(paramStr(1).lines).map(proc(pass: string): int =
  var
    row_lb = 0
    row_ub = 128
    col_lb = 0
    col_ub = 8

  for c in pass:
    if c == 'F':
      row_ub = (row_lb + row_ub) div 2
    elif c == 'B':
      row_lb = (row_lb + row_ub) div 2
    elif c == 'L':
      col_ub = (col_lb + col_ub) div 2
    elif c == 'R':
      col_lb = (col_lb + col_ub) div 2

  row_lb * 8 + col_lb
).sorted()

echo "Max seat ID: ", $(seatIds[^1])

for i in 1 .. len(seatIds) - 1:
  if seatIds[i - 1] + 2 == seatIds[i]:
    echo "My seat ID: ", $(seatIds[i - 1] + 1)
    break
