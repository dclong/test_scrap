# test_scrap

## Comments 

1. The raw u8 array output of the screen capture is in BGRA format. 
    That is if we consider the u8 array as a 3-d array, it has the dimension (heigh, width, 4)
    and the last dimension has the order `B`, `G`, `R`, `A`. 
1. No need to use the alpha channel. You might save space and time by ignoring the alpha channel.
2. You can avoid decoding and encoding by using the raw u8 array directly. This saves time at the cost of space. 
3. For image processing purpose, 
    the order of the last dimension (whether it is RGB or BGR)
    as long as you use for both training and inference.

## Question:
1. How to take screenshot of a small area? We can of course take the corresponding part of the underlying u8 array, but is this the most efficient way? 
