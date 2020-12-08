import os
import strutils

type
  Opcode = enum
    nop, jmp, acc
  Instruction = (Opcode, int)
  Program = seq[(Instruction, bool)]

var initialProg: Program

for line in paramStr(1).lines:
  let linewords = line.strip.split
  let opcode = parseEnum[Opcode](linewords[0])
  let arg = parseInt(linewords[1])
  initialProg.add(((opcode, arg), false))

proc RunProg(inputProg: Program): (bool, int) =
  var prog = inputProg
  var pc = 0
  result = (true, 0)

  while pc < inputProg.len():
    if prog[pc][1]:
      result[0] = false  # have looped
      break
    let ins = prog[pc][0]
    prog[pc][1] = true
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
  if ins[0][0] == nop or ins[0][0] == jmp:
    let newOpcode = (if ins[0][0] == nop: jmp else: nop)
    var modifiedProg = initialProg
    modifiedProg[i] = ((newOpcode, ins[0][1]), false)
    let res = RunProg(modifiedProg)
    if res[0]:
      echo "Accumulator value after properly terminating: ", res[1]
      break

