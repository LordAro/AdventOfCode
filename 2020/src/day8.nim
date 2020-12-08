import os
import strutils

type
  Opcode = enum
    nop, jmp, acc
  Instruction = (Opcode, int)
  Program = seq[Instruction]

var initialProg: Program

for line in paramStr(1).lines:
  let opcode = parseEnum[Opcode](line[0..2])
  let arg = parseInt(line[4..^1])
  initialProg.add((opcode, arg))

proc RunProg(prog: Program): (bool, int, set[uint16]) =
  var pc = 0
  result = (true, 0, {})

  while pc < prog.len():
    if cast[uint16](pc) in result[2]:
      result[0] = false  # have looped
      break
    let ins = prog[pc]
    result[2].incl(cast[uint16](pc))
    case ins[0]:
      of nop:
        discard
      of jmp:
        pc += ins[1] - 1
      of acc:
        result[1] += ins[1]
    inc pc

let res = RunProg(initialProg)
echo "Accumulator value: ", res[1]

let trace = res[2]

var potential_landing_spots: set[uint16]

var i = initialProg.len()
while true:
  potential_landing_spots.incl(cast[uint16](i))
  dec i

  if initialProg[i][0] == jmp and initialProg[i][1] < 0:
    break

let start = i
var swapIdx = 0

if cast[uint16](i) in trace:
  swapIdx = i
else:
  while true:
    dec i
    if cast[uint16](i) in potential_landing_spots:
      continue
    elif initialProg[i][0] == nop:
      if cast[uint16](i) in trace and cast[uint16](i + initialProg[i][1]) in potential_landing_spots:
        swapIdx = i
        break
    elif initialProg[i][0] == jmp:
      if cast[uint16](i) notin trace and
        cast[uint16](i) notin potential_landing_spots and
        cast[uint16](i + initialProg[i][1]) in potential_landing_spots:
        var j = i - 1
        while true:
          if initialProg[j][0] == jmp:
            break
          dec j

        if cast[uint16](j) in trace:
          swapIdx = j
          break
        else:
          for k in j + 1..i:
            potential_landing_spots.incl(cast[uint16](k))
          i = start

var modifiedProg = initialProg
modifiedProg[swapIdx][0] = (if modifiedProg[swapIdx][0] == nop: jmp else: nop)
let res2 = RunProg(modifiedProg)
echo "Accumulator value after properly terminating: ", (res2[0], res2[1])
