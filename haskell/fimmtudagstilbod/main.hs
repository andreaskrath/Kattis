-- Fimmtudagstilboð
-- https://open.kattis.com/problems/fimmtudagstilbod

main = do
  sNum <- getLine
  print (solution (read sNum :: Int))

solution n =
  if n > 2020
    then 1000 + (abs (2020 - n) * 100)
    else 1000
