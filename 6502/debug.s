  .macro DEBUG_CHAR,char
    .ifdef DEBUG
      PHA
      LDA #\char
      JSR print_char
      PLA
    .endif
  .endmacro
