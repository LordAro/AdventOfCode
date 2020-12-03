import os
import sequtils

let inputData = toSeq(paramStr(1).lines)

var treeCounts = @[0, 0, 0, 0, 0]

for i, line in inputData:
  if line[(1 * i) mod line.len] == '#':
    inc treeCounts[0]
  if line[(3 * i) mod line.len] == '#':
    inc treeCounts[1]
  if line[(5 * i) mod line.len] == '#':
    inc treeCounts[2]
  if line[(7 * i) mod line.len] == '#':
    inc treeCounts[3]
  if i mod 2 == 0 and line[(i div 2) mod line.len] == '#':
    inc treeCounts[4]

echo "Tree count: ", $(treeCounts[1])
echo "Trees multiple: ", $(treeCounts.foldl(a * b))

