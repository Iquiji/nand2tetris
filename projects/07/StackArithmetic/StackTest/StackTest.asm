// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
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
D; JEQ
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

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_1_TRUE
D; JEQ
@_COMP_LABEL_1_FALSE
0; JMP
(_COMP_LABEL_1_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_1_END
0; JMP
(_COMP_LABEL_1_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_1_END)

// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_2_TRUE
D; JEQ
@_COMP_LABEL_2_FALSE
0; JMP
(_COMP_LABEL_2_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_2_END
0; JMP
(_COMP_LABEL_2_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_2_END)

// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891
@891
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
@_COMP_LABEL_3_TRUE
D; JLT
@_COMP_LABEL_3_FALSE
0; JMP
(_COMP_LABEL_3_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_3_END
0; JMP
(_COMP_LABEL_3_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_3_END)

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 892
@892
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
@_COMP_LABEL_4_TRUE
D; JLT
@_COMP_LABEL_4_FALSE
0; JMP
(_COMP_LABEL_4_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_4_END
0; JMP
(_COMP_LABEL_4_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_4_END)

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891
@891
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
@_COMP_LABEL_5_TRUE
D; JLT
@_COMP_LABEL_5_FALSE
0; JMP
(_COMP_LABEL_5_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_5_END
0; JMP
(_COMP_LABEL_5_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_5_END)

// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_6_TRUE
D; JGT
@_COMP_LABEL_6_FALSE
0; JMP
(_COMP_LABEL_6_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_6_END
0; JMP
(_COMP_LABEL_6_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_6_END)

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_7_TRUE
D; JGT
@_COMP_LABEL_7_FALSE
0; JMP
(_COMP_LABEL_7_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_7_END
0; JMP
(_COMP_LABEL_7_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_7_END)

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=A-D
@_COMP_LABEL_8_TRUE
D; JGT
@_COMP_LABEL_8_FALSE
0; JMP
(_COMP_LABEL_8_TRUE)
@SP
A=M
M=-1
@SP
M=M+1
@_COMP_LABEL_8_END
0; JMP
(_COMP_LABEL_8_FALSE)
@SP
A=M
M=0
@SP
M=M+1
(_COMP_LABEL_8_END)

// push constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 53
@53
D=A
@SP
A=M
M=D
@SP
M=M+1

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

// push constant 112
@112
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

// neg
@SP
M=M-1
A=M
D=M
D=-D
@SP
A=M
M=D
@SP
M=M+1

// and
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=D&A
@SP
A=M
M=D
@SP
M=M+1

// push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

// or
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
A=M
D=D|A
@SP
A=M
M=D
@SP
M=M+1

// not
@SP
M=M-1
A=M
D=M
D=!D
@SP
A=M
M=D
@SP
M=M+1
