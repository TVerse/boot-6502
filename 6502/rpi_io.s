read_byte:
  JSR read_nibble
  PHY
  TAY
  JSR read_nibble
  STA N
  TYA
  CLC
  ASL
  ASL
  ASL
  ASL
  ORA N
  PLY
  RTS

read_nibble:
  RTS

write_byte:
  PHA
  CLC
  LSR
  LSR
  LSR
  LSR
  JSR write_nibble
  PLA
  ORA #$FF
  JSR write_nibble
  RTS

write_nibble:
  RTS
