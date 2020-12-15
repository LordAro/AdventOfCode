import bitops
import os
import strscans
import strutils
import tables
import sets

# Copied straight out of narimiran/itertools
iterator product*[T](s: openArray[T], repeat: Positive): seq[T] =
  ## Iterator yielding Cartesian products of ``s`` with itself, ``repeat`` number of times.
  var counters = newSeq[int](repeat)

  block outer:
    while true:
      var result = newSeq[T](repeat)
      for i, cnt in counters:
        result[i] = s[cnt]
      yield result

      var i = repeat - 1
      while true:
        inc counters[i]
        if counters[i] == s.len:
          counters[i] = 0
          dec i
        else: break
        if i < 0:
          break outer

var
  mask_str: string
  mask0, mask1: int64
  mem, mem2: Table[int64, int64]
  bitTwiddles: seq[int]
  memAddrMask: int64

#for line in [
#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
#"mem[8] = 11",
#"mem[7] = 101",
#"mem[8] = 0",
#]:
#for line in [
#"mask = 000000000000000000000000000000X1001X",
#"mem[42] = 100",
#"mask = 00000000000000000000000000000000X0XX",
#"mem[26] = 1",
#]:
for line in paramStr(1).lines:
  if line.startsWith("mask = "):
    mask0 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111
    mask1 = 0
    mask_str = line[7 .. ^1]
    bitTwiddles.setLen(0)
    memAddrMask = 0
    for i, c in mask_str:
      let b = 36 - i - 1
      case c:
        of 'X':
          bitTwiddles.add(b)
        of '1':
          mask1.setBit(b)
          memAddrMask.setBit(b)
        of '0':
          mask0.clearBit(b)
        else:
          raise newException(ValueError, "Unknown bitmask char")
  else:
    var
      mem_addr, mem_value: int
    if scanf(line, "mem[$i] = $i", mem_addr, mem_value):
      mem[mem_addr] = cast[int64](mem_value).bitor(mask1).bitand(mask0)
      var memAddrMasked = cast[int64](mem_addr).bitor(memAddrMask)

      # Get all combinations of bit twiddling
      for arr in product([0, 1], bit_twiddles.len):
        var mem_addr_modif2 = memAddrMasked
        for i, b in arr:
          if b == 0:
            mem_addr_modif2.clearBit(bit_twiddles[i])
          else:
            mem_addr_modif2.setBit(bit_twiddles[i])
        mem2[mem_addr_modif2] = mem_value

    else:
      raise newException(ValueError, "Could not match mem address")

var total_val = 0i64
for val in mem.values:
  total_val += val

echo "Total memory value: ", total_val

total_val = 0
for val in mem2.values:
  total_val += val
echo "Total memory value v2: ", total_val
