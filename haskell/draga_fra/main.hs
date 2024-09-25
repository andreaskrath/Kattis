-- Draga Frá
-- https://open.kattis.com/problems/dragafra

main = do
  a_s <- getLine
  b_s <- getLine
  let a = (read a_s :: Int)
  let b = (read b_s :: Int)
  let c = show (a - b)
  putStrLn c
