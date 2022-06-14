D=-1
@LCL
M=D
@ARG
M=D
@THIS
M=D
@THAT
M=D
@256
D=A
@SP
M=D

@bootstrap
D=A
@SP
A=M
M=D

@SP
M=M+1


@LCL
D=M
@SP
A=M
M=D

@SP
M=M+1


@ARG
D=M
@SP
A=M
M=D

@SP
M=M+1


@THIS
D=M
@SP
A=M
M=D

@SP
M=M+1


@THAT
D=M
@SP
A=M
M=D

@SP
M=M+1


@SP
D=M
@5
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
0; JMP
(bootstrap)
// End of Bootstrap


// function Main.fibonacci 0
(Main.fibonacci)

// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_0_TRUE
D; JLT
@_COMP_LABEL_0_FALSE
0; JMP
(_COMP_LABEL_0_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_0_END
0; JMP
(_COMP_LABEL_0_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_0_END)

// if-goto IF_TRUE
@SP
M=M-1
A=M
D=M
@Main.fibonacci$IF_TRUE
D; JNE

// goto IF_FALSE
@Main.fibonacci$IF_FALSE
0; JMP

// label IF_TRUE
(Main.fibonacci$IF_TRUE)

// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// return
@LCL
D=M
@R13
M=D
@R13
D=M
@5
A=D-A
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
@R13
A=M
D=A
@1
A=D-A
D=M
@THAT
M=D
@R13
A=M
D=A
@2
A=D-A
D=M
@THIS
M=D
@R13
A=M
D=A
@3
A=D-A
D=M
@ARG
M=D
@R13
A=M
D=A
@4
A=D-A
D=M
@LCL
M=D
@R14
A=M
0 ; JMP // Return from function

// label IF_FALSE
(Main.fibonacci$IF_FALSE)

// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1
@Main.fibonacci$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0; JMP // Jump to Function
(Main.fibonacci$ret.0) // return label

// push argument 0
@ARG
D=M
@0
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1
@Main.fibonacci$ret.1
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0; JMP // Jump to Function
(Main.fibonacci$ret.1) // return label

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=D+A
@SP
A=M
M=D
@SP
M=M+1

// return
@LCL
D=M
@R13
M=D
@R13
D=M
@5
A=D-A
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
@R13
A=M
D=A
@1
A=D-A
D=M
@THAT
M=D
@R13
A=M
D=A
@2
A=D-A
D=M
@THIS
M=D
@R13
A=M
D=A
@3
A=D-A
D=M
@ARG
M=D
@R13
A=M
D=A
@4
A=D-A
D=M
@LCL
M=D
@R14
A=M
0 ; JMP // Return from function


//NEW FILE!

// function Sys.init 0
(Sys.init)

// push constant 4
@4
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1
@Sys.init$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0; JMP // Jump to Function
(Sys.init$ret.0) // return label

// label WHILE
(Sys.init$WHILE)

// goto WHILE
@Sys.init$WHILE
0; JMP
