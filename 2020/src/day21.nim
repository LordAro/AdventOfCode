import algorithm
import os
import sets
import sequtils
import strutils
import tables

var foods: seq[(seq[string], seq[string])]

var emptySet: HashSet[string]
var allergenMap: Table[string, HashSet[string]]

for line in paramStr(1).lines:
  let paren = line.find('(')
  let ingredients = line[0 .. paren - 2].split
  let allergens = line[paren + 10 .. ^2].split(", ")
  for allergen in allergens:
    allergenMap.mgetOrPut(allergen, emptySet).incl(ingredients.toHashSet)
  foods.add((ingredients, allergens))

#for each food,
#  for each allergen,
#    set the candidates for this allergen to the intersection of the current candidates with the ingredients of this food
for food in foods:
  for allergen in food[1]:
    allergenMap[allergen] = allergenMap[allergen] * food[0].toHashSet

var allergenIngredients: seq[(string, string)]

while allergenMap.len > 0:
  var matchingIngredient: HashSet[string]
  for allergen, ingredients in allergenMap:
    if ingredients.len == 1:
      matchingIngredient = ingredients
      allergenMap.del(allergen)
      for i in ingredients: # can't get a single element of sets??
        allergenIngredients.add((i, allergen))
        break
      break
  for allergen in allergenMap.keys:
    allergenMap[allergen] = allergenMap[allergen] - matchingIngredient

var ingredientCount = 0
for food in foods:
  for ingred in food[0]:
    if ingred notin allergenIngredients.mapIt(it[0]):
      inc ingredientCount

echo "Ingredients without allergens count: ", ingredientCount

echo "Canonical dangerous ingredient list: ",
  allergenIngredients.toSeq.sorted(
    proc(x, y: (string, string)): int = cmp(x[1], y[1])
  ).mapIt(it[0]).join(",")
