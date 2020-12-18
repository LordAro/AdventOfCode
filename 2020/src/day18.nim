import os
import strutils

proc calcParse(s: string, noPrec: static bool): seq[char] =
  var operatorStack: seq[char]
  for idx, c in s:
    if isDigit(s[idx]):
      result.add(s[idx])
    elif s[idx] == '*' or s[idx] == '+':
      while operatorStack.len > 0 and (noPrec or operatorStack[^1] == '+') and operatorStack[^1] != '(':
        result.add(operatorStack.pop)
      operatorStack.add(s[idx])
    elif s[idx] == '(':
      operatorStack.add('(')
    elif s[idx] == ')':
      while operatorStack[^1] != '(':
        result.add(operatorStack.pop)
      if operatorStack[^1] == '(':
        discard operatorStack.pop
  while operatorStack.len > 0:
    result.add(operatorStack.pop)

proc evalRPN(rpn: seq[char]): int =
  var resultStack: seq[int]
  for c in rpn:
    case c:
      of '+':
        resultStack.add(resultStack.pop + resultStack.pop)
      of '*':
        resultStack.add(resultStack.pop * resultStack.pop)
      else:
        resultStack.add(ord(c) - ord('0'))
  return resultStack[0]


var
  sumP1 = 0
  sumP2 = 0
for line in paramStr(1).lines:
  sumP1 += evalRPN(calcParse(line, true))
  sumP2 += evalRPN(calcParse(line, false))

echo "Homework sum: ", sumP1
echo "Advanced homework sum: ", sumP2
