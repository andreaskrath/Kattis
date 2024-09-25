-- Blandað Best
-- https://open.kattis.com/problems/blandadbest

import Control.Monad

main = do
  n <- getLine
  let num = read n :: Int
  list <- replicateM num getLine
  putStrLn (solution n list)

solution n list =
  if n == "1"
    then head list
    else "blandad best"
