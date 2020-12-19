import os

proc calcParse(s: string, noPrecedence: static bool): seq[char] =
  var operatorStack: seq[char]
  for c in s:
    if c in '0' .. '9':
      result.add(c)
    elif c == '*' or c == '+':
      while operatorStack.len > 0 and
          (noPrecedence or operatorStack[^1] == '+') and
          operatorStack[^1] != '(':
        result.add(operatorStack.pop)
      operatorStack.add(c)
    elif c == '(':
      operatorStack.add('(')
    elif c == ')':
      while operatorStack[^1] != '(':
        result.add(operatorStack.pop)
      discard operatorStack.pop # always a '(' unless bracket mismatching has happened
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
