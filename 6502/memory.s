; Stack: start stop -
; Max 256 chars including the null or infinite loop
; Clobbers Y
copy_string_from_start:
  ; Copy to N so we don't have to use X
  LDA 0, X
  STA N
  LDA 1, X
  STA N + 1
  LDA 2, X
  STA N + 2
  LDA 3, X
  STA N + 3
  LDY #-1
.loop:
  INY
  LDA (N), Y
  STA (N + 2), Y
  BNE .loop
  RTS

