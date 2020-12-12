import os
import strutils

type
  Direction = enum
    N, E, S, W

var position = (0, 0)
var direction = E

var waypointPos = (10, 1) # 1N, 10E
var shipPos2 = (0, 0)

proc TranslateCoord(coord: (int, int), translation: int): (int, int) =
  # Assumes translation is n*90 degrees clockwise
  case translation:
    of 0:
      return coord
    of 1:
      return (coord[1], -coord[0])
    of 2:
      return (-coord[0], -coord[1])
    of 3:
      return (-coord[1], coord[0])
    else:
      raise newException(ValueError, "NOTREACHED")

for line in paramStr(1).lines:
  let action = line[0]
  let value = parseInt(line[1 .. ^1])

  case action:
    of 'N':
      position[1] += value
      waypointPos[1] += value
    of 'S':
      position[1] -= value
      waypointPos[1] -= value
    of 'E':
      position[0] += value
      waypointPos[0] += value
    of 'W':
      position[0] -= value
      waypointPos[0] -= value
    of 'L':
      assert value mod 90 == 0
      direction = Direction((ord(direction) + (4 - value div 90)) mod 4)
      waypointPos = TranslateCoord(waypointPos, 4 - value div 90)
    of 'R':
      assert value mod 90 == 0
      direction = Direction((ord(direction) + (value div 90)) mod 4)
      waypointPos = TranslateCoord(waypointPos, value div 90)
    of 'F':
      case direction:
        of N:
          position[1] += value
        of E:
          position[0] += value
        of S:
          position[1] -= value
        of W:
          position[0] -= value
      shipPos2 = (shipPos2[0] + (waypointPos[0] * value), shipPos2[1] + (waypointPos[1] * value))
    else:
      raise newException(ValueError, "Unknown action")

echo "Final position: ", position, ", Manhattan: ", abs(position[0]) + abs(position[1])
echo "Relative ship position: ", shipPos2, ", Manhattan: ", abs(shipPos2[0]) + abs(shipPos2[1])
