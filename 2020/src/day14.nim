import bitops
import os
import strscans
import strutils
import tables

var mask0: int64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111
var mask1: int64 = 0
var mem: Table[int64, int64]

for line in [
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
"mem[8] = 11",
"mem[7] = 101",
"mem[8] = 0",
]:
#for line in paramStr(1).lines:
  if line.startsWith("mask = "):
    let mask_str = line[7 .. ^1]
    for i, c in mask_str:
      let b = 36 - i - 1
      case c:
        of 'X':
          discard
        of '1':
          mask0.setBit(b)
          mask1.setBit(b)
        of '0':
          mask0.clearBit(b)
          mask1.clearBit(b)
        else:
          raise newException(ValueError, "Unknown bitmask char")
  else:
    var
      mem_addr, mem_value: int
      mem_value64: int64
    if scanf(line, "mem[$i] = $i", mem_addr, mem_value):
      mem_value_64 = mem_value
      mem[mem_addr] = mem_value64.bitor(mask1).bitand(mask0)
#      echo mem_value.toBin(36)
#      echo mem_value64.bitor(mask1).bitand(mask0).toBin(36)
#      echo ""
      echo (mem_addr, mem_value, mem_value64.bitor(mask1).bitand(mask0))
    else:
      raise newException(ValueError, "Could not match mem address")

var total_val = 0i64
for val in mem.values:
  total_val += val

echo "Total memory value: ", total_val
