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

proc RunProg(inputProg: Program): (bool, int) =
  var seenIns: set[uint16] # upper limit for program size
  var prog = inputProg
  var pc = 0
  result = (true, 0)

  while pc < inputProg.len():
    if cast[uint16](pc) in seenIns:
      result[0] = false  # have looped
      break
    let ins = prog[pc]
    seenIns.incl(cast[uint16](pc))
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

for i, ins in initialProg:
  if ins[0] == nop or ins[0] == jmp:
    let newOpcode = (if ins[0] == nop: jmp else: nop)
    var modifiedProg = initialProg
    modifiedProg[i] = (newOpcode, ins[1])
    let res = RunProg(modifiedProg)
    if res[0]:
      echo "Accumulator value after properly terminating: ", res[1]
      break

