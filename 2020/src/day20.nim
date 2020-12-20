import algorithm
import math
import os
import sets
import sequtils
import strutils
import tables

type
  Tile = seq[seq[bool]]

proc getEdges(t: Tile): seq[seq[bool]] =
  result.add(t[0])
  result.add(t[^1])
  result.add(t.mapIt(it[0]))
  result.add(t.mapIt(it[^1]))
  result.add(result.mapIt(it.reversed)) # reverse everything

proc rotateTile[T](t: seq[seq[T]]): seq[seq[T]] =
  result = t # easy way of creating same size tile
  for i in 0 .. t.high:
    for j in 0 .. t.high: # assumes square
      result[i][j] = t[t.len - j - 1][i]

proc flipTile[T](t: seq[seq[T]]): seq[seq[T]] =
  result = t # easy way of creating same size tile
  for i in 0 .. t.high:
    for j in 0 .. t.high: # assumes square
      result[j][i] = t[i][j]

iterator getTileVariants(t: Tile): Tile =
  var tile = t
  for f in 0 .. 1:
    yield tile
    for r in 1 .. 3:
      tile = rotateTile(tile)
      yield tile
    tile = flipTile(t)

iterator choose*[T](a: openarray[T], num_choose: int): seq[T] =
  var
    chosen = newSeqOfCap[T](num_choose)
    i = 0
    i_stack = newSeqOfCap[int](num_choose)

  while true:
    if chosen.len == num_choose:
      yield chosen
      discard chosen.pop()
      i = i_stack.pop() + 1
    elif i != a.len:
      chosen.add(a[i])
      i_stack.add(i)
      inc i
    elif i_stack.len > 0:
      discard chosen.pop()
      i = i_stack.pop() + 1
    else:
      break

proc `$`(t: Tile): string =
  t.mapIt(it.mapIt(if it: '#' else: '.').join).join("\n")


# Parse
var tiles: Table[int, Tile]
var tileid: int
var parsedTile: Tile
for line in paramStr(1).lines:
  if line == "":
    tiles[tileid] = parsedTile
    parsedTile.reset()
  elif line[0] == 'T':
    tileid = parseInt(line[5 .. 8]) # always 4 digit number
  else:
    parsedTile.add(line.mapIt(it == '#'))
if parsedTile.len > 0:
  tiles[tileid] = parsedTile # last one

# Find common edges
var tileEdges: Table[int, seq[int]]
for tPair in choose(toSeq(tiles.keys), 2):
  let
    t1Edges = getEdges(tiles[tPair[0]]).toHashSet
    t2Edges = getEdges(tiles[tPair[1]]).toHashSet
  let commonEdges = intersection(t1Edges, t2Edges)
  if commonEdges.len > 0:
    tileEdges.mgetOrPut(tPair[0], @[]).add(tPair[1])
    tileEdges.mgetOrPut(tPair[1], @[]).add(tPair[0])

# Get corners
var cornerProduct = 1
var corners: seq[int]
for t, edges in tileEdges:
  if edges.len == 2:
    corners.add(t)
    cornerProduct *= t

echo "Corner product: ", cornerProduct


# Algorithm:
# Position corner piece in top left, using both neighbours to determine correct orientation
# For each neighbour of a placed piece (pp)
#   If neighbour already placed, continue
#   For each free position around pp
#     For each rotational variant of neighbour
#       If edges match
#         Place piece

let gridSize = sqrt(tiles.len.float).int
let emptyTile = newSeqWith(0, newSeq[bool](0)) # something to initialise the 2d array (that's actually a 4d array) with
var positionedTiles = newSeqWith(gridSize, newSeqWith[Tile](gridSize, emptyTile))
var positionedTileIndexes: Table[int, (int, int)]

# Find correct orientation for first corner
block outer:
  for t in getTileVariants(tiles[corners[0]]):
    let rightEdge = t.mapIt(it[^1])
    let bottomEdge = t[^1]
    let rightTIdx = tileEdges[corners[0]][0]
    let bottomTIdx = tileEdges[corners[0]][1]
    for rightT in getTileVariants(tiles[rightTIdx]):
      let leftEdge = rightT.mapIt(it[0])
      for bottomT in getTileVariants(tiles[bottomTIdx]):
        let topEdge = bottomT[0]
        if rightEdge == leftEdge and bottomEdge == topEdge:
          positionedTiles[0][0] = t
          positionedTiles[0][1] = rightT
          positionedTiles[1][0] = bottomT
          positionedTileIndexes[corners[0]] = (0, 0)
          positionedTileIndexes[rightTIdx] = (0, 1)
          positionedTileIndexes[bottomTIdx] = (1, 0)
          break outer

var fullyPlacedTiles: seq[int] = @[corners[0]]
var addNeighbours = tileEdges[corners[0]]
while addNeighbours.len > 0:
  let tile = addNeighbours.pop
  for neighbour in tileEdges[tile]:
    if neighbour in fullyPlacedTiles:
      continue
    let tilePosition = positionedTileIndexes[tile]
    block neighbourSearch:
      for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        let neighbourPosition = (tilePosition[0] + delta[0], tilePosition[1] + delta[1])
        if neighbourPosition[0] notin 0 .. positionedTiles.high or neighbourPosition[1] notin 0 .. positionedTiles.high:
          # out of range
          continue
        if positionedTiles[neighbourPosition[0]][neighbourPosition[1]].len > 0:
          # position already filled
          continue
        let positionedTile = positionedTiles[tilePosition[0]][tilePosition[1]]
        var positionedTileEdge: seq[bool]
        if delta == (-1, 0):
          positionedTileEdge = positionedTile[0] # top edge
        elif delta == (1, 0):
          positionedTileEdge = positionedTile[^1] # bottom edge
        elif delta == (0, -1):
          positionedTileEdge = positionedTile.mapIt(it[0]) # left edge
        elif delta == (0, 1):
          positionedTileEdge = positionedTile.mapIt(it[^1]) # right edge

        for t in getTileVariants(tiles[neighbour]):
          var neighbourTileEdge: seq[bool]
          if delta == (-1, 0):
            neighbourTileEdge = t[^1] # bottom edge
          elif delta == (1, 0):
            neighbourTileEdge = t[0] # top edge
          elif delta == (0, -1):
            neighbourTileEdge = t.mapIt(it[^1]) # right edge
          elif delta == (0, 1):
            neighbourTileEdge = t.mapIt(it[0]) # left edge

          if neighbourTileEdge == positionedTileEdge:
            positionedTiles[neighbourPosition[0]][neighbourPosition[1]] = t
            positionedTileIndexes[neighbour] = neighbourPosition
            fullyPlacedTiles.add(neighbour)
            addNeighbours.add(neighbour)
            # Found!
            break neighbourSearch

# Trim off edges
for y in 0 .. positionedTiles.high:
  for x in 0 .. positionedTiles[y].high:
    positionedTiles[y][x] = positionedTiles[y][x][1 .. ^2]
    for j in 0 .. positionedTiles[y][x].high:
      positionedTiles[y][x][j] = positionedTiles[y][x][j][1 .. ^2]

#for gridrow in positionedTiles:
#  for y in 0 .. gridrow[0][0].high:
#    echo concat(gridrow.mapIt(it[y])).mapIt(if it: '#' else: '.').join

# Flatten
var flattenedGrid: Tile
for gridrow in positionedTiles:
  for y in 0 .. gridrow[0][0].high:
    flattenedGrid.add(concat(gridrow.mapIt(it[y])))

## v (1, 0)          #
## #    ##    ##    ###
##  #  #  #  #  #  #
##                tail            hump                            hump                                head
let seaMonster = [(1, 0), (2, 1), (2, 4), (1, 5), (1, 6), (2, 7), (2, 10), (1, 11), (1, 12), (2, 13), (2, 16), (1, 17), (1, 18), (1, 19), (0, 18)]
var monsterCount = 0
for grid in getTileVariants(flattenedGrid):
  #echo grid, "\n"
  for y in 0 .. grid.high:
    for x in 0 .. grid[y].high:
      if seaMonster.all(proc(d: (int, int)): bool =
          let y1 = y + d[0]
          let x1 = x + d[1]
          return y1 in 0 .. grid.high and x1 in 0 .. grid[y1].high and grid[y1][x1]
      ):
        inc monsterCount
  if monsterCount > 0:
    # This is the correct image orientation, no need to go any further
    break

let numberHashes = flattenedGrid.foldl(a + b.filterIt(it).len, 0)
echo "Number of waves: ", numberHashes - (monsterCount * seaMonster.len), " (", monsterCount, " sea monsters)"
