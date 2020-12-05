import algorithm
import os
import sequtils

var seatIds = toSeq(paramStr(1).lines).map(proc(pass: string): int =
  var pass_no = 0
  for i, c in pass:
    pass_no = pass_no or ord(c == 'B' or c == 'R') shl (pass.len - i - 1)

  let row = pass_no shr 3    # Take first 7 bits
  let col = pass_no and 0x7  # Take last 3 bits

  row * 8 + col
)
seatIds.sort()

echo "Max seat ID: ", $(seatIds[^1])

for i in 1 .. len(seatIds) - 1:
  if seatIds[i - 1] + 2 == seatIds[i]:
    echo "My seat ID: ", $(seatIds[i - 1] + 1)
    break
