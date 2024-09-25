-- Takkar
-- https://open.kattis.com/problems/takkar

main = do
  a <- getLine
  b <- getLine
  solution (read a :: Int) (read b :: Int)

solution a b
  | a < b = putStrLn "FAKE NEWS!"
  | a > b = putStrLn "MAGA!"
  | a == b = putStrLn "WORLD WAR 3!"
