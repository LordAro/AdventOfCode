import os
import sequtils
import strutils

let publicKeys = toSeq(paramStr(1).lines).mapIt(it.parseInt)

iterator getPublicKeyIterator(subjectNum, loopSizeLimit: int): (int, int) =
  var value = 1
  for loopSize in 1 .. loopSizeLimit:
    value *= subjectNum
    value = value mod 20201227
    yield (loopSize, value)

proc getPublicKey(subjectNum, loopSize: int): int =
  for (loopSize, pubkey) in getPublicKeyIterator(subjectNum, loopSize):
    result = pubkey

var
  matchingPubkey = 0
  matchingLoopSize = 0
for (loopSize, pubkey) in getPublicKeyIterator(7, 10_000_000):
  if pubkey == publicKeys[0]:
    matchingPubkey = publicKeys[1]
    matchingLoopSize = loopSize
  elif pubkey == publicKeys[1]:
    matchingPubkey = publicKeys[0]
    matchingLoopSize = loopSize

  if matchingLoopSize != 0:
    break

echo "Encryption key: ", getPublicKey(matchingPubkey, matchingLoopSize)
