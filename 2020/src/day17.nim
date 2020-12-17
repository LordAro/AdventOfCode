import os
import sets
import algorithm
import sequtils

type
  Coord[N: static int] = array[0 .. N - 1, int]
  State[N: static int] = HashSet[Coord[N]]

const deltas = @[-1, 0, 1]

iterator getAllNeighbours(coord: Coord): Coord =
  for c in product(repeat(deltas, coord.len)):
    var n = coord
    for i, v in c:
      n[i] += v
    if n == coord:
      continue
    yield n

proc countNeighbours(state: State, coord: Coord): int =
  for neighbour in getAllNeighbours(coord):
    if state.contains(neighbour):
      inc result

proc runStep[N](inputState: State[N]): State[N] =
  var
    possibleActiveNeighbours: HashSet[Coord[N]]
  for c in inputState:
    for n in getAllNeighbours(c):
      if n notin inputState:
        possibleActiveNeighbours.incl(n)
    if countNeighbours(inputState, c) in 2 .. 3:
      result.incl(c)
  for n in possibleActiveNeighbours:
    if countNeighbours(inputState, n) == 3:
      result.incl(n)

var state3d: State[3]
var state4d: State[4]

var y = 0 # iterators don't have indexes? :(
for line in paramStr(1).lines:
  for x, c in line:
    if c == '#':
      state3d.incl([x, y, 0])
      state4d.incl([x, y, 0, 0])
  inc y

for cycle in 1 .. 6:
  state3d = runStep(state3d)
  state4d = runStep(state4d)

echo "Active cells (3D): ", state3d.len
echo "Active cells (4D): ", state4d.len
