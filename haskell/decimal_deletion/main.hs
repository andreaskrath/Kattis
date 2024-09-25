-- Decimal deletion
-- https://open.kattis.com/problems/decimaldeletion

main = do
  input <- getLine
  let f = read input :: Float
  print (round f)
