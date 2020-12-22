import algorithm
import deques
import math
import os
import sequtils
import strutils

var
  deck1, deck2: Deque[int]
  deckSize = 0
  isPlayer2 = false

for line in paramStr(1).lines:
  if line.startsWith("Player"):
    discard
  elif line == "":
    isPlayer2 = true
  elif isPlayer2:
    deck2.addLast(parseInt(line))
    inc deckSize
  else:
    deck1.addLast(parseInt(line))
    inc deckSize

proc playGame(deck1, deck2: Deque[int]): int =
  let deckSize = deck1.len + deck2.len
  var player1Deck = deck1
  var player2Deck = deck2
  while player1Deck.len > 0 and player2Deck.len > 0:
    #echo "Round: ", i, " ", player1Deck, player2Deck
    let
      player1 = player1Deck.popFirst
      player2 = player2Deck.popFirst
    if player1 > player2:
      player1Deck.addLast(player1)
      player1Deck.addLast(player2)
    else:
      player2Deck.addLast(player2)
      player2Deck.addLast(player1)

  (1 .. deckSize).toSeq.reversed.zip(player1Deck.toSeq & player2Deck.toSeq).mapIt(it[0] * it[1]).sum

echo "Winning score: ", playGame(deck1, deck2)
