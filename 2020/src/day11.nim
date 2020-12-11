import os
import sequtils

type
  SeatState = enum
    None, Vacant, Occupied
  State = seq[seq[SeatState]]
  AdjacentProc = proc(state: State, x: int, y: int): int

proc CountOccupiedAdjacent(state: State, x : int, y : int): int =
  let deltas = @[(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]
  for d in deltas:
    var y1 = y + d[0]
    var x1 = x + d[1]
    if x1 in 0 .. state[0].high() and y1 in 0 .. state.high():
      result += int(state[y1][x1] == Occupied)

proc CountOccupiedAdjacentDistant(state: State, x : int, y : int): int =
  let deltas = @[(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]

  for d in deltas:
    var y1 = y + d[0]
    var x1 = x + d[1]

    while x1 in 0 .. state[0].high() and y1 in 0 .. state.high():
      if state[y1][x1] != None:
        result += int(state[y1][x1] == Occupied)
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
  var prevOccupied = -1
  var nowOccupied = 0
  var currState = state
  while prevOccupied != nowOccupied:
    prevOccupied = nowOccupied
    nowOccupied = 0
    var newState = newSeqWith(currState.len(), newSeq[SeatState](currState[0].len()))
    for y, row in currState:
      for x, cell in row:
        if cell == None:
          newState[y][x] = None
        else:
          let adjacent = adjacentProc(currState, x, y)
          let newSeat =
            if adjacent == 0:
              Occupied
            elif adjacent >= adjacentLimit:
              Vacant
            else:
              cell  # No change

          if newSeat == Occupied:
            inc nowOccupied
          newState[y][x] = newSeat

    currState = newState
  return nowOccupied

echo "Final seat count: ", RunSeats(inputData, CountOccupiedAdjacent, 4)
echo "Final seat count with updated rules: ", RunSeats(inputData, CountOccupiedAdjacentDistant, 5)
