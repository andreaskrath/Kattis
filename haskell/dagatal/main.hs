-- Dagatal
-- https://open.kattis.com/problems/dagatal

main = do
  month <- getLine
  putStrLn (solution month)

solution s
  | s == "1" = "31"
  | s == "2" = "28"
  | s == "3" = "31"
  | s == "4" = "30"
  | s == "5" = "31"
  | s == "6" = "30"
  | s == "7" = "31"
  | s == "8" = "31"
  | s == "9" = "30"
  | s == "10" = "31"
  | s == "11" = "30"
  | s == "12" = "31"
