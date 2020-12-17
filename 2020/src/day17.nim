import os
import sets
import algorithm
import sequtils

type
  Coord3D = tuple[x: int, y: int, z: int]
  Coord4D = tuple[x: int, y: int, z: int, w: int]
  Coord = Coord4D
  CoordSeq = seq[int]
  State3D = HashSet[Coord3D]
  State4D = HashSet[Coord4D]
  State = State4D

iterator getAllNeighbours(coord: Coord): Coord =
  for x1 in coord.x - 1 .. coord.x + 1:
    for y1 in coord.y - 1 .. coord.y + 1:
      for z1 in coord.z - 1 .. coord.z + 1:
        for w1 in coord.w - 1 .. coord.w + 1:
          let n = (x1, y1, z1, w1)
          if n == coord:
            continue
          yield (x1, y1, z1, w1)


proc countNeighbours(state: State, coord: Coord): int =
  for neighbour in getAllNeighbours(coord):
    if state.contains(neighbour):
      inc result


var state: State

var y = 0 # iterators don't have indexes? :(
for line in paramStr(1).lines:
  for x, c in line:
    if c == '#':
      state.incl((x, y, 0, 0))
  inc y

for cycle in 1 .. 6:
  var newState: State
  var possibleActiveNeighbours: HashSet[Coord]
  for c in state:
    for n in getAllNeighbours(c):
      if n notin state:
        possibleActiveNeighbours.incl(n)
    if countNeighbours(state, c) in 2 .. 3:
      newState.incl(c)
  for n in possibleActiveNeighbours:
    if countNeighbours(state, n) == 3:
      newState.incl(n)
  state = newState

echo "Active cells: ", state.len
