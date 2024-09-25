-- Leyniþjónusta
-- https://open.kattis.com/problems/leynithjonusta

main = do
  input <- getLine
  let output = filter (/= ' ') input
  putStrLn output
