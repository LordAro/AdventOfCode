import os
import sets
import sequtils

type CubeCoord = tuple[x, y, z: int]

proc hexMovesToCoord(path: openArray[string]): CubeCoord =
  for move in path:
    case move:
    of "e":
      inc result.x
      dec result.y
    of "w":
      dec result.x
      inc result.y
    of "nw":
      inc result.y
      dec result.z
    of "ne":
      inc result.x
      dec result.z
    of "sw":
      dec result.x
      inc result.z
    of "se":
      dec result.y
      inc result.z
    else:
      raise newException(ValueError, "Unknown move value " & move)

proc stringToHexMoves(path: string): seq[string] =
  for c in path:
    if result.len > 0 and (result[^1] == "s" or result[^1] == "n"):
      result[^1] &= c
    else:
      result.add("" & c)

proc adjacentCoord(coord: CubeCoord): array[6, CubeCoord] =
  return [
    (coord.x - 1, coord.y + 1, coord.z),
    (coord.x - 1, coord.y, coord.z + 1),
    (coord.x, coord.y - 1, coord.z + 1),
    (coord.x + 1, coord.y - 1, coord.z),
    (coord.x + 1, coord.y, coord.z - 1),
    (coord.x, coord.y + 1, coord.z - 1),
  ]

var flippedTiles: HashSet[CubeCoord]

for line in paramStr(1).lines:
  let coord = hexMovesToCoord(stringToHexMoves(line))
  if flippedTiles.missingOrExcl(coord):
    flippedTiles.incl(coord)

echo "Number of flipped tiles: ", flippedTiles.len

for i in 1 .. 100:
  var unflippedTiles: HashSet[CubeCoord]
  for tile in flippedTiles:
    for n in adjacentCoord(tile):
      if n notin flippedTiles:
        unflippedTiles.incl(n)
  var newGrid = flippedTiles
  for tile in flippedTiles:
    let flippedNeighbours = adjacentCoord(tile).filterIt(it in flippedTiles).len
    if flippedNeighbours == 0 or flippedNeighbours > 2:
      newGrid.excl(tile)

  for tile in unflippedTiles:
    let flippedNeighbours = adjacentCoord(tile).filterIt(it in flippedTiles).len
    if flippedNeighbours == 2:
      newGrid.incl(tile)

  flippedTiles = newGrid

echo "Number of flipped tiles after 100 days: ", flippedTiles.len
