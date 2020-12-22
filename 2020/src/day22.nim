import algorithm
import deques
import math
import os
import sequtils
import sets
import strutils

type
  Hand = Deque[int]
  HandList = seq[int]

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
    initdeck2.addLast(parseInt(line))
    inc deckSize
  else:
    initdeck1.addLast(parseInt(line))
    inc deckSize

proc playGame(deck1, deck2: Hand, shouldRecurse: static bool = false): (int, bool) =
  var roundCache: HashSet[(HandList, HandList)]

  let deckSize = deck1.len + deck2.len
  var player1Deck = deck1
  var player2Deck = deck2
  var i = 1
  while player1Deck.len > 0 and player2Deck.len > 0:
    #echo "Round: ", i, " ", player1Deck, player2Deck
    #if shouldRecurse:
    let handPair = (player1Deck.toSeq, player2Deck.toSeq)
    if handPair in roundCache:
      return (0, true)
    roundCache.incl(handPair)

    let
      player1 = player1Deck.popFirst
      player2 = player2Deck.popFirst
    var player1Winner = false
    if shouldRecurse and player1 <= player1Deck.len and player2 <= player2Deck.len:
      let subgameP1Deck = player1Deck.toSeq[0 .. player1 - 1].toDeque
      let subgameP2Deck = player2Deck.toSeq[0 .. player2 - 1].toDeque
      #echo player1, " ", subgameP1Deck
      #echo player2, " ", subgameP2Deck
      let subgame = playGame(subgameP1Deck, subgameP2Deck, true)
      player1Winner = subgame[1]
    else:
      player1Winner = player1 > player2

    if player1Winner:
      player1Deck.addLast(player1)
      player1Deck.addLast(player2)
    else:
      player2Deck.addLast(player2)
      player2Deck.addLast(player1)
    inc i

  (
    (1 .. deckSize).toSeq.reversed.zip(player1Deck.toSeq & player2Deck.toSeq).mapIt(it[0] * it[1]).sum,
    player1Deck.len > 0
  )

echo "Winning score: ", playGame(initdeck1, initdeck2)[0]
echo "Winning recursive score: ", playGame(initdeck1, initdeck2, true)[0]
