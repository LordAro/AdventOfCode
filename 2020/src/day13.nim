import strutils
import os
import sequtils
import bigints # nimble install bigints

proc bezout_coeffs(a: BigInt, b: BigInt): (BigInt, BigInt) =
  var
    (old_r, r) = (a, b)
    (old_s, s) = (1.initBigInt, 0.initBigInt)
    (old_t, t) = (0.initBigInt, 1.initBigInt)

  while r != 0.initBigInt:
    let quotient = old_r div r
    (old_r, r) = (r, old_r - (quotient * r))
    (old_s, s) = (s, old_s - (quotient * s))
    (old_t, t) = (t, old_t - (quotient * t))
  return (old_s, old_t)

let inputData = paramStr(1).readLines(2)

let targetTime = parseInt(inputData[0])
let allBusPositions = inputData[1].split(',').map(proc(x: string): int = (if x == "x": 0 else: parseInt(x)))

let busIDs = allBusPositions.filter(proc(x: int): bool = x != 0)
let nextBusTimes = busIDs.map(proc(x: int): int = ((targetTime div x) + 1) * x)
let nextBusIdx = nextBusTimes.minIndex

echo "Next bus: ", (nextBusTimes[nextBusIdx] - targetTime) * busIDs[nextBusIdx]

var aTotal = 0.initBigInt
var nTotal = 1.initBigInt

for a, n in allBusPositions:
  # Where t + a = 0 mod n
  if n != 0:
    let n_big = n.initBigInt
    let (m1, m2) = bezout_coeffs(nTotal, n_big)
    # therefore, t = -a mod n
    aTotal = (aTotal * n_big * m2) + ((-a).initBigInt * nTotal * m1) # super big numbers generated here
    nTotal *= n_big

let firstTime = aTotal mod nTotal
echo "Winning timestamp: ", firstTime
