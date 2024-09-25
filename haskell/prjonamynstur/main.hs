-- Prjónamynstur
-- https://open.kattis.com/problems/prjonamynstur

import Control.Monad

main = do
  line <- getLine
  rows <- replicateM (read (head (words line)) :: Int) getLine
  print (solution rows 0)

solution [] summed = summed
solution rows summed =
  solution newRows (summed + rowSum)
  where
    newRows = tail rows
    rowSum = sumRow (head rows) 0

sumRow [] summed = summed
sumRow row summed =
  sumRow newRow (summed + symbolValue)
  where
    newRow = tail row
    symbolValue = symbolToInt (head row)

symbolToInt i
  | i == '.' = 20
  | i == 'O' = 10
  | i == '\\' || i == '/' = 25
  | i == 'A' = 35
  | i == '^' = 5
  | i == 'v' = 22
