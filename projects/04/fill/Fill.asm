// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
// Pseudo Code: 
// loop{
//     if key_pressed{
//         draw black in all pixels
//     }
//     else{
//         draw white in all pixels
//     }
// }

(START)
@KBD
D=M

@FLUSH_BLACK
D; JNE // Goto Flush black if any key is pressed

@FLUSH_WHITE 
0; JMP // Else Goto Flush white

(FLUSH_BLACK)

@SCREEN
D=A
@current_in_screen // Load in Screen location variable
M=D

A=M
M=-1

(FLUSH_BLACK_LOOP)

// Compare with end of loop
@24576
D=A
@current_in_screen
D=D-M // Calculate Difference
@START
D ; JEQ // Jump to start if Difference is zero, else continue flushing to screen

@current_in_screen
M = M+1

A=M
M=-1

@KBD
D=M

@FLUSH_WHITE
D; JEQ // Goto Flush black if any key is pressed

@FLUSH_BLACK_LOOP // recur flushing black to screen
0 ; JMP



(FLUSH_WHITE)

@SCREEN
D=A
@current_in_screen // Load in Screen location variable
M=D

A=M
M=0

(FLUSH_WHITE_LOOP)

// Compare with end of loop
@24576
D=A
@current_in_screen
D=D-M // Calculate Difference
@START
D ; JEQ // Jump to start if Difference is zero, else continue flushing to screen

@current_in_screen
M = M+1

A=M
M=0

@KBD
D=M

@FLUSH_BLACK
D; JNE // Goto Flush black if any key is pressed

@FLUSH_WHITE_LOOP // recur flushing white to screen
0 ; JMP