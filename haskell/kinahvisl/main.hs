-- Kínahvísl
-- https://open.kattis.com/problems/kinahvisl

main = do
  initialWord <- getLine
  finalWord <- getLine
  print (solution initialWord finalWord 1)

solution [] [] count = count
solution initialWord finalWord count =
  let currentInitial = head initialWord
      currentFinal = head finalWord
   in if currentInitial == currentFinal
        then solution newInitial newFinal count
        else solution newInitial newFinal (count + 1)
  where
    newInitial = tail initialWord
    newFinal = tail finalWord
