Map Parser and PNG Generator Libraries
======================================

This document provides usage instructions for various implementations of a map parser and PNG generator in different programming languages.

Python
------

The Python implementation allows you to parse a map file and generate PNG images for textures, colliders, and triggers.

### Usage

1.  Install the required libraries (e.g., Pillow for image handling).
2.  Call the \`generate\_image\_from\_data\` function with the pixel data.
3.  Use \`write\_png\` to save the PNG file to disk.

    python script.py

C++
---

The C++ implementation uses libpng to handle PNG encoding. It provides functions to create and save PNG files based on map data.

### Usage

1. use cmake to build
2.  Use the \`generate\_pixel\_data\` function to create pixel data.
3.  Call \`write\_png\` to save the PNG files.

C#
--

The C# implementation generates PNG files from pixel data and is designed to work with .NET.

### Usage

1.  Add the \`PngGenerator\` class to your project.
2.  Call \`GeneratePixelData\` to create the pixel data.
3.  Use \`WritePng\` to write the PNG files to disk.


Rust
----

The Rust implementation provides functions to generate and write PNG files using only the standard library.

### Usage

1.  import the Rust code into your program
2.  Call \`generate\_pixel\_data\` to produce pixel data.
3.  Use \`write\_png\` to save the PNG files.

    cargo build

Haskell
-------

The Haskell implementation generates PNG files using standard libraries. It handles image creation and data generation.

### Usage

1.  import the Haskell code into your progect
2.  Use \`generatePixelData\` to generate image data.
3.  Call \`writePng\` to output the PNG files.


Lua
---

The Lua implementation handles PNG file creation and pixel data generation using only Lua's standard libraries.

### Usage

1.  import the Lua script into your program
2.  Use \`generatePixelData\` to prepare image data.
3.  Call \`writePng\` to save the PNG files.

    lua script.lua

JavaScript (Node.js)
--------------------

The Node.js implementation generates PNG files using built-in modules. It includes functions to handle pixel data and PNG encoding.

### Usage

1.  import the script into your files 
2.  Use \`generatePixelData\` to create image data.
3.  Call \`writePng\` to output the PNG files.

    node script.js

Java
----

The Java implementation provides functionality to generate PNG images using standard Java libraries. It includes methods for pixel data and PNG file creation.

### Usage

1.  import the Java code into your project
2.  Call \`generatePixelData\` to prepare the image data.
3.  Use \`writePng\` to save the PNG files to disk.
