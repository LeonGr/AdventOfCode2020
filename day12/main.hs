main :: IO ()
main = do
    ls <- fmap lines (readFile "input") -- Read strings from file to list
    print $ navigatePart1 $ parseLines ls
    print $ navigatePart2 $ parseLines ls

type Vector = (Int, Int)

-- Convert list of input lines to list of tuples (action, value)
parseLines :: [String] -> [(Char, Int)]
parseLines = map (\ x -> (head x, read (tail x) :: Int))

-- Calculate manhattan distance between start point and reached pointed after following instructions
navigatePart1 :: [(Char, Int)] -> Int
navigatePart1 = rec (0, 0) (1, 0) -- call with starting position (0, 0), direction (1, 0) and instructions
    where
        -- Recursive subfunction rec
        -- Takes the current position, current direction and instructions left
        rec :: Vector -> Vector -> [(Char, Int)] -> Int
        -- If we have no instructions left return the manhattan distance
        rec (x, y) _ [] = abs x + abs y
        -- Otherwise, update current position or direction according to the first instruction (act, val)
        -- and recurse with the new values and the remaining instructions
        rec pos dir ((act, val):ins)
          | act == 'F' = rec (vectorAdd pos (scalarMul val dir))  dir                    ins
          | act == 'N' = rec (vectorAdd pos (0, val))             dir                    ins
          | act == 'E' = rec (vectorAdd pos (val, 0))             dir                    ins
          | act == 'S' = rec (vectorAdd pos (0, -val))            dir                    ins
          | act == 'W' = rec (vectorAdd pos (-val, 0))            dir                    ins
          | act == 'L' = rec pos                                  (rotateVec val    dir) ins
          | act == 'R' = rec pos                                  (rotateVec (-val) dir) ins
          | otherwise     = error "Unknown instruction"

-- See part 1
navigatePart2 :: [(Char, Int)] -> Int
navigatePart2 = rec (0, 0) (10, 1)
    where
        rec :: Vector -> Vector -> [(Char, Int)] -> Int
        rec (x, y) _ [] = abs x + abs y
        rec pos dir ((act, val):ins)
          | act == 'F' = rec (vectorAdd pos (scalarMul val dir)) dir                       ins
          | act == 'N' = rec pos                                 (vectorAdd dir (0, val))  ins
          | act == 'E' = rec pos                                 (vectorAdd dir (val, 0))  ins
          | act == 'S' = rec pos                                 (vectorAdd dir (0, -val)) ins
          | act == 'W' = rec pos                                 (vectorAdd dir (-val, 0)) ins
          | act == 'L' = rec pos                                 (rotateVec val    dir)    ins
          | act == 'R' = rec pos                                 (rotateVec (-val) dir)    ins
          | otherwise     = error "Unknown instruction"


-- Some quick math functions

rotateVec :: Int -> Vector -> Vector
rotateVec deg (x, y) = (round (xf * cos rad - yf * sin rad),
                        round (xf * sin rad + yf * cos rad))
    where
        rad = ((fromIntegral deg :: Float) * pi) / 180
        xf = fromIntegral x
        yf = fromIntegral y

scalarMul :: Int -> Vector -> Vector
scalarMul a (x, y) = (a * x, a * y)

vectorAdd :: Vector -> Vector -> Vector
vectorAdd (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)
