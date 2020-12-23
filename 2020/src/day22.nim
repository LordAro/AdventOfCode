import algorithm
import hashes
import math
import os
import sequtils
import sets
import strutils

type Hand = seq[int]

proc hash(handPair: (Hand, Hand)): Hash =
  var h: Hash = 0
  h = h !& hash(handPair[0])
  h = h !& hash(handPair[1])
  result = !$h

proc playGame(deck1, deck2: Hand, shouldRecurse: static bool = false): (int, bool) =
  var roundCache: HashSet[Hash]

  let deckSize = deck1.len + deck2.len
  var player1Deck = deck1
  var player2Deck = deck2
  while player1Deck.len > 0 and player2Deck.len > 0:
    let handPairHash = hash((player1Deck, player2Deck))
    if handPairHash in roundCache:
      return (0, true)
    roundCache.incl(handPairHash)

    let
      player1 = player1Deck[0]
      player2 = player2Deck[0]
    player1Deck.delete(0)
    player2Deck.delete(0)
    var player1Winner = false
    if shouldRecurse and player1 <= player1Deck.len and player2 <= player2Deck.len:
      let subgameP1Deck = player1Deck[0 .. player1 - 1]
      let subgameP2Deck = player2Deck[0 .. player2 - 1]
      let subgame = playGame(subgameP1Deck, subgameP2Deck, true)
      player1Winner = subgame[1]
    else:
      player1Winner = player1 > player2

    if player1Winner:
      player1Deck.add(player1)
      player1Deck.add(player2)
    else:
      player2Deck.add(player2)
      player2Deck.add(player1)

  (
    (1 .. deckSize).toSeq.reversed.zip(player1Deck & player2Deck).mapIt(it[0] * it[1]).sum,
    player1Deck.len > 0
  )

var
  initdeck1, initdeck2: Hand
  deckSize = 0
  isPlayer2 = false

for line in paramStr(1).lines:
  if line.startsWith("Player"):
    discard
  elif line == "":
    isPlayer2 = true
  elif isPlayer2:
    initdeck2.add(parseInt(line))
    inc deckSize
  else:
    initdeck1.add(parseInt(line))
    inc deckSize

echo "Winning score: ", playGame(initdeck1, initdeck2)[0]
echo "Winning recursive score: ", playGame(initdeck1, initdeck2, true)[0]
