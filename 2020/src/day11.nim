import os
import sequtils
import tables

type
  SeatState = enum
    None, Vacant, Occupied
  State = seq[seq[SeatState]]
  AdjacentTable = Table[(int, int), seq[(int, int)]]
  AdjacentProc = proc(state: State): AdjacentTable

const deltas = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]

proc FindAdjacents(state: State): AdjacentTable =
  let
    cHigh = state.high()
    rHigh = state[0].high()
  for y in 0 .. cHigh:
    for x in 0 .. rHigh:
      if state[y][x] == None:
        continue
      result[(y, x)] = @[]
      for d in deltas:
        let
          y1 = y + d[0]
          x1 = x + d[1]
        if y1 in 0 .. cHigh and x1 in 0 .. rHigh:
          result[(y, x)].add((y1, x1))

proc FindAdjacentsWithDistance(state: State): AdjacentTable =
  let
    cHigh = state.high()
    rHigh = state[0].high()
  for y in 0 .. cHigh:
    for x in 0 .. rHigh:
      if state[y][x] == None:
        continue
      result[(y, x)] = @[]
      for d in deltas:
        var
          y1 = y + d[0]
          x1 = x + d[1]
        while y1 in 0 .. cHigh and x1 in 0 .. rHigh:
          if state[y1][x1] != None:
            result[(y, x)].add((y1, x1))
            break
          y1 += d[0]
          x1 += d[1]

var inputData: State

for line in paramStr(1).lines:
  inputData.add(@[])
  for c in line:
    let seat = case c
      of '.': None
      of 'L': Vacant
      else: raise newException(ValueError, "unknown char")
    inputData[^1].add(seat)

proc RunSeats(state: State, adjacentProc: AdjacentProc, adjacentLimit: int): int =
  let adjacentTable = adjacentProc(state)
  var
    currState = state
    changed = true
  while changed:
    changed = false
    var newState = currState
    for coord, adjacents in adjacentTable:
      let
        y = coord[0]
        x = coord[1]
        occupiedAdjacents = adjacents.filterIt(currState[it[0]][it[1]] == Occupied).len
      if currState[y][x] == Vacant and occupiedAdjacents == 0:
        newState[y][x] = Occupied
        changed = true
      elif currState[y][x] == Occupied and occupiedAdjacents >= adjacentLimit:
        newState[y][x] = Vacant
        changed = true

    currState = newState

  var occupied = 0
  for row in currState:
    for cell in row:
      if cell == Occupied:
        inc occupied
  return occupied

echo "Final seat count: ", RunSeats(inputData, FindAdjacents, 4)
echo "Final seat count with updated rules: ", RunSeats(inputData, FindAdjacentsWithDistance, 5)
