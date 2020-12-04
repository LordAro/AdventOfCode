import os
import strutils
import sequtils
import tables

let inputData = open(paramStr(1)).readAll.split("\n\n").map(proc(
    record: string): Table[string, string] =
  record.strip.split.map(proc(x: string): (string, string) =
    let y = x.split(':')
    (y[0], y[1])
  ).toTable
)

let passportsWithCorrectFields = inputData.filter(proc(rec: Table[string, string]): bool =
  "byr" in rec and "iyr" in rec and "eyr" in rec and "hgt" in rec and
    "hcl" in rec and "ecl" in rec and "pid" in rec
)

echo "Number of passports with correct fields: ", $(passportsWithCorrectFields.len)

let validPassports = passportsWithCorrectFields.filter(proc(rec: Table[string, string]): bool =
  let byr = parseInt(rec["byr"])
  byr >= 1920 and byr <= 2002
).filter(proc(rec: Table[string, string]): bool =
  let iyr = parseInt(rec["iyr"])
  iyr >= 2010 and iyr <= 2020
).filter(proc(rec: Table[string, string]): bool =
  let eyr = parseInt(rec["eyr"])
  eyr >= 2020 and eyr <= 2030
).filter(proc(rec: Table[string, string]): bool =
  try:
    let hgt = parseInt(rec["hgt"][0 .. ^3]) # assume suffix exists and trim it off
    result = (rec["hgt"].endsWith("cm") and hgt >= 150 and hgt <= 193) or
      (rec["hgt"].endsWith("in") and hgt >= 59 and hgt <= 76)
  except ValueError:
    result = false
).filter(proc(rec: Table[string, string]): bool =
  rec["hcl"][0] == '#' and rec["hcl"][1 .. ^1].filter(proc(x: char): bool =
    (x >= 'a' and x <= 'f') or (x >= '0' and x <= '9')
  ).len == 6
).filter(proc(rec: Table[string, string]): bool =
  any(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], proc(col: string): bool = rec["ecl"] == col)
).filter(proc(rec: Table[string, string]): bool =
  rec["pid"].len == 9 and rec["pid"].filter(isDigit).len == 9
)

echo "Number of valid passports: ", $(validPassports.len)
