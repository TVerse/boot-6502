  .import print_char

  .macro DEBUG_CHAR char
    .ifdef DEBUG
      PHA
      LDA #char
      JSR print_char
      PLA
    .endif
  .endmacro

byte_in_a_to_hex:
  PHX
  PHA
  TAX
  LDA #'$'
  JSR print_char
  TXA
  LSR
  LSR
  LSR
  LSR
  TAX
  LDA @byte_to_hex_table,X
  JSR print_char
  PLA
  PHA
  AND #%00001111
  TAX
  LDA @byte_to_hex_table,X
  JSR print_char
  PLA
  PLX
  RTS
  @byte_to_hex_table: .literal "0123456789ABCDEF"

  .macro DEBUG_A
    .ifdef DEBUG
      PHP
      JSR byte_in_a_to_hex
      PLP
    .endif
  .endmacro
