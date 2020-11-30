import os
import strutils
import sequtils

echo "Hello, world!"

let inputData = open(paramStr(1)).readAll().strip()
echo "I have read this from a file: '" & inputData & "'"

let charVals = inputData.map(proc(x: char): int = ord(x))
echo "I have read this from a file: '" & $(charVals) & "'"
