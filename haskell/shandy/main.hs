-- Shandy
-- https://open.kattis.com/problems/shandy

main = do
  a <- getLine
  b <- getLine
  print (2 * min (read a :: Int) (read b :: Int))
