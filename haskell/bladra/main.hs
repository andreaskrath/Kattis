-- Blaðra
-- https://open.kattis.com/problems/bladra2

main = do
  input <- getLine
  let split = words input
  let v = read (head split) :: Float
  let a = read (head (tail split)) :: Float
  let t = read (last split) :: Float
  print (solution v a t)

solution v a t = v * t + 0.5 * a * t ^ 2
