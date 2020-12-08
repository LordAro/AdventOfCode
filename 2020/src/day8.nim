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

proc RunProg(prog: Program): (bool, int) =
  var pc = 0
  result = (true, 0)
  var seenIns: set[uint16]

  while pc < prog.len():
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

proc fillReds(prog: Program): seq[bool] =
  result = newSeq[bool](prog.len())
  var i = 0
  while not result[i]:
    result[i] = true
    if prog[i][0] == jmp:
      i += prog[i][1]
    else:
      inc i

proc fillBlues(prog: Program): seq[bool] =
  let l = prog.len()
  result = newSeq[bool](l)
  var comesFrom = newSeq[seq[int]](l)
  for i, ins in prog:
    if ins[0] == jmp and i + ins[1] < l:
      comesFrom[i + ins[1]].add(i)
    elif i + 1 < l:
      comesFrom[i + 1].add(i)

  var Idxs = @[prog.high()]
  while Idxs.len != 0:
    let i = Idxs.pop
    if result[i]:
      break
    result[i] = true
    Idxs.add(comesFrom[i])

proc fixProgram(prog: Program): int =
  let
    reds = fillReds(prog)
    blues = fillBlues(prog)
  for i, ins in prog:
    if not reds[i]:
      continue
    case prog[i][0]:
      of acc:
        discard
      of jmp:
        if blues[i + 1]:
          return i
      of nop:
        if blues[i + prog[i][1]]:
          return i
  return -1

let swapIdx = fixProgram(initialProg)

var modifiedProg = initialProg
modifiedProg[swapIdx][0] = (if modifiedProg[swapIdx][0] == nop: jmp else: nop)
let res2 = RunProg(modifiedProg)
echo "Accumulator value after properly terminating: ", res2[1]
