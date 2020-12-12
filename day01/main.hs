main :: IO ()
main = do
    ls <- fmap lines (readFile "input") -- Read strings from file to list
    print $ product $ sum2020 $ cartesianProdTwo $ toInts ls
    print $ product $ sum2020 $ cartesianProdThree $ toInts ls

toInts :: [String] -> [Int]
toInts = map (\ x -> read x :: Int)

cartesianProdTwo :: [Int] -> [[Int]]
cartesianProdTwo xs = [ [x, y] | x <- xs, y <- xs ]

cartesianProdThree :: [Int] -> [[Int]]
cartesianProdThree xs = [ [x, y, z] | x <- xs, y <- xs, z <- xs ]

sum2020 :: [[Int]] -> [Int]
sum2020 xs = head $ filter (\ x -> sum x == 2020) xs
