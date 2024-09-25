-- Hipp Hipp
-- https://open.kattis.com/problems/hipphipp?editresubmit=14409137

main = do
  solution 20

solution 0 = return ()
solution n = do
  putStrLn "Hipp hipp hurra!"
  solution next
  where
    next = n - 1
