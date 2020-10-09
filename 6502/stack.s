  .macro INX2
     INX
     INX     
  .endmacro

  .macro DEX2
     DEX
     DEX
  .endmacro

  .macro POP
     INX2
  .endmacro

  .macro ZERO
    DEX2
    STZ 0,X
    STZ 1,X
  .endmacro

  .macro LITERAL,lit
    DEX2
    LDA #<\lit
    STA 0,X
    LDA #>\lit
    STA 1,X
  .endmacro

  .macro LITERAL_8BIT,lit
    DEX2
    LDA #\lit
    STA 0,X
    STZ 1,X
  .endmacro

  .macro AT_ADDRESS,addr
    DEX2
    LDA \addr
    STA 0,X
    LDA \addr + 1
    STA 1,X
  .endmacro

  .macro AT_ADDRESS_8BIT,addr
    DEX2
    LDA \addr
    STA 0,X
    STZ 1,X
  .endmacro
