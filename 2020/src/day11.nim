import os

type
  SeatState = enum
    None, Vacant, Occupied
  State = seq[seq[SeatState]]
  AdjacentProc = proc(state: State, x: int, y: int): int

const deltas = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]

proc CountOccupiedAdjacent(state: State, x: int, y: int): int =
  let cHigh = state.high()
  let rHigh = state[0].high()
  for d in deltas:
    let y1 = y + d[0]
    let x1 = x + d[1]
    if y1 in 0 .. cHigh and x1 in 0 .. rHigh:
      result += int(state[y1][x1] == Occupied)

proc CountOccupiedAdjacentDistant(state: State, x: int, y: int): int =
  let cHigh = state.high()
  let rHigh = state[0].high()
  for d in deltas:
    var y1 = y + d[0]
    var x1 = x + d[1]

    while y1 in 0 .. cHigh and x1 in 0 .. rHigh:
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
  var
    currState = state
    changed = true
  while changed:
    changed = false
    var newState = currState
    for y, row in currState:
      for x, cell in row:
        if cell == None:
          continue
        let adjacent = adjacentProc(currState, x, y)
        if cell == Vacant and adjacent == 0:
          newState[y][x] = Occupied
          changed = true
        elif cell == Occupied and adjacent >= adjacentLimit:
          newState[y][x] = Vacant
          changed = true

    currState = newState

  var occupied = 0
  for row in currState:
    for cell in row:
      if cell == Occupied:
        inc occupied
  return occupied

echo "Final seat count: ", RunSeats(inputData, CountOccupiedAdjacent, 4)
echo "Final seat count with updated rules: ", RunSeats(inputData, CountOccupiedAdjacentDistant, 5)
