{-# LANGUAGE OverloadedStrings #-}

import Data.List (stripPrefix, isPrefixOf)
import Data.Maybe (fromMaybe)
import Data.Map (Map)
import qualified Data.Map as Map
import Data.Word (Word8)
import Data.Bits (shiftR, (.&.))
import System.IO (readFile, writeFile)
import Control.Monad (forM_)

-- Constants
tileSize :: Int
tileSize = 16

width :: Int
width = 256

height :: Int
height = 256

-- Data Types
data Binding = Binding
  { texturePaths :: Map String String
  , colliderColors :: Map String (Word8, Word8, Word8, Word8)
  , triggerColors :: Map String (Word8, Word8, Word8, Word8)
  }

-- Functions to parse the map file
parseMapFile :: FilePath -> IO (Map String (Int, Int), Map String (Int, Int), Map String (Int, Int), Binding)
parseMapFile path = do
    content <- readFile path
    let lines' = lines content
        (metadata, rest) = break ("[TEXTURES]" `isPrefixOf`) lines'
        (textures, rest') = break ("[COLLIDERS]" `isPrefixOf`) (drop 1 rest)
        (colliders, rest'') = break ("[TRIGGERS]" `isPrefixOf`) (drop 1 rest')
        triggers = drop 1 rest''
        (textureMap, bindings) = parseSection textures
        (colliderMap, bindings') = parseSection colliders
        (triggerMap, bindings'') = parseSection triggers
        finalBindings = Binding (texturePaths bindings) (colliderColors bindings') (triggerColors bindings'')
    
    return (textureMap, colliderMap, triggerMap, finalBindings)

parseSection :: [String] -> (Map String (Int, Int), Binding)
parseSection lines' = (Map.fromList parsedData, Binding Map.empty Map.empty Map.empty)
  where
    parsedData = map parseLine lines'
    parseLine line = let (key, value) = break (=='=') line
                      in (key, read (drop 1 value) :: (Int, Int))

-- Functions to generate PNG data
generatePixelData :: Map String (Int, Int) -> Binding -> [Word8]
generatePixelData items binding = generateImageData (width, height) (mapColor binding)

generateImageData :: (Int, Int) -> (Int -> Int -> [Word8]) -> [Word8]
generateImageData (w, h) colorFunc = concatMap generateRow [0..(h-1)]
  where
    generateRow y = concatMap (generatePixel y) [0..(w-1)]
    generatePixel y x = colorFunc x y

mapColor :: Binding -> Int -> Int -> [Word8]
mapColor binding x y =
    let color = [0xFF, 0x00, 0x00, 0xFF] -- Default color
    in color -- You would add logic to get colors from Binding based on x, y

-- Simple PNG writing (No actual PNG encoding, just raw RGBA)
writePng :: FilePath -> [Word8] -> IO ()
writePng path pixels = do
    let header = "P6\n" ++ show width ++ " " ++ show height ++ "\n255\n"
        pixelData = concatMap show pixels
    writeFile path (header ++ pixelData)

-- Main function
main :: IO ()
main = do
    (textureData, colliderData, triggerData, binding) <- parseMapFile "example.map"
    let texturePixels = generatePixelData textureData binding
        colliderPixels = generatePixelData colliderData binding
        triggerPixels = generatePixelData triggerData binding
    writePng "textures.png" texturePixels
    writePng "colliders.png" colliderPixels
    writePng "triggers.png" triggerPixels
