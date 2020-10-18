  .macro DEBUG_CHAR,char
    .ifdef DEBUG
      PHA
      LDA #\char
      JSR print_char
      PLA
    .endif
  .endmacro

  .macro DEBUG_CHAR,char
    .ifdef DEBUG
      PHA
      LDA #\char
      JSR print_char
      PLA
    .endif
  .endmacro

  .macro DEBUG_A
    .ifdef DEBUG
      JSR byte_in_a_to_hex
    .endif
  .endmacro

  .ifdef DEBUG
byte_in_a_to_hex:
  .byte_to_hex_table: .ascii "0123456789ABCDEF"
  PHX
  PHA
  TAX
  LDA #"$"
  JSR print_char
  TXA
  AND #%00001111
  TAX
  LDA .byte_to_hex_table,X
  JSR print_char
  PLA
  PHA
  LSR
  LSR
  LSR
  LSR
  TAX
  LDA .byte_to_hex_table,X
  JSR print_char
  PLA
  RTS
  .endif
