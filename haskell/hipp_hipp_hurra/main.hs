-- Hipp Hipp Húrra
-- https://open.kattis.com/problems/hipphipphurra

main = do
  name <- getLine
  n <- getLine
  solution name (read n :: Int)

solution name 0 = return ()
solution name n = do
  putStrLn ("Hipp hipp hurra, " ++ name ++ "!")
  solution name (n - 1)
