-- Besta gjöfin
-- https://open.kattis.com/problems/bestagjofin

import Control.Monad

main = do
  n <- getLine
  let num = read n :: Int
  list <- replicateM num getLine
  putStrLn (solution list "" 0)

solution [] name score = name
solution list name score =
  let current = head list
      split = words current
      current_score = read (last split) :: Int
   in if current_score > score
        then solution (tail list) (head split) current_score
        else solution (tail list) name score
