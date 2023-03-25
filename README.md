# BASIC-compiler
BASIC compiler built in rust

# PROCESS
The program starts by reading the input file located in src names "input.bas". The contents of this file are read into a string that the Lexer will analyse. The Parser then gets each token from the lexer and creates basic assembly code that would theoretically work on a processor. The process is as follows:

1. Parser request a token from the Lexer
2. Lexer then reads the next character or characters and creates an appropiate token for the given word or symbol
3. Parser then checks which token it is currently looking at
4. Parser will then construct basic assembly code with the tokens following the first token recieved
5. this process is repeated until an EOF token is seen by the Parser

The Parser request the tokens from the Lexer as it runs. Meaning this compiler does the parsing and lexing at the same time.
