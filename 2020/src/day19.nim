import nre
import os
import sequtils
import strutils
import tables

var
  isRegex = true
  regexRawStrings: Table[int, seq[seq[int]]]
  inputStrings: seq[string]
  aN, bN: int

for line in paramStr(1).lines:
  if line.len == 0:
    isRegex = false
  elif isRegex:
    let spl = line.split(": ")
    let num = parseInt(spl[0])
    if '"' in spl[1]:
      if 'a' in spl[1]:
        aN = num
      elif 'b' in spl[1]:
        bN = num
    else:
      regexRawStrings[num] = @[]
      for orNum in spl[1].split(" | "):
        regexRawStrings[num].add(@[])
        for resNum in orNum.split(' '):
          regexRawStrings[num][^1].add(parseInt(resNum))
  else:
    inputStrings.add(line)

proc expandStr(n: int): string =
  if n == aN:
    return "a"
  elif n == bN:
    return "b"

  result &= '('
  for orBlock in regexRawStrings[n]:
    for val in orBlock:
      if val == n:
        # Super hacky way of dealing with recursive patterns. mmmmm.
        if val == 8:
          result &= '+'
          continue
        elif val == 11:
          let r42 = expandStr(42)
          let r31 = expandStr(31)
          return "(?<r>" & r42 & "(?&r)?" & r31 & ")"
      result &= expandStr(val)
    result &= '|'
  result[^1] = ')' # overwrite last or block

let regexStr = re("^" & expandStr(0) & "$")

let matchesP1 = inputStrings.filterIt(it.match(regexStr).isSome).len
echo "Number of valid strings: ", matchesP1

# 8: 42 | 42 8
# 11: 42 31 | 42 11 31
regexRawStrings[8] = @[@[42], @[42, 8]]
regexRawStrings[11] = @[@[42, 31], @[42, 11, 31]]
let regexStrP2 = re("^" & expandStr(0) & "$")
let matchesP2 = inputStrings.filterIt(it.match(regexStrP2).isSome).len
echo "Number of valid strings with updated rules: ", matchesP2
