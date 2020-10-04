  .include base.s

  .org ROM_START_ADDR

SHIFT_READY = $1000
SHIFTED_BYTE = $1001

reset:
  ; Turn on cursor
  JSR wait_lcd_ready
  LDA #%00001110
  JSR lcd_instruction
  STZ INITIALIZATION_DONE

  ;LITERAL waiting
  ;JSR print_string_stack

  STZ SHIFT_READY

  ; SHIFT ON
  LDA #$FF
  STA T2CL
  LDA ACR
  AND #%11101111
  ORA #%00001100
  STA ACR
  LDA #%10000100
  STA IER
  LDA SR
loop:
  .shift:
  WAI
  LDA SHIFT_READY
  BEQ .shift
  STZ SHIFT_READY
  LDA SHIFTED_BYTE
  JSR print_char
  ;LDA SHIFTED_BYTE
  ;CMP #%10101010
  ;BNE .error
  .continue:
    JMP loop
  .error:
    JSR sr_error

sr_error:
  STZ SHIFTED_BYTE + 1
  LITERAL SHIFTED_BYTE
  JMP error

toggle_led:
  LDA PORTA
  EOR #1
  STA PORTA
  RTS

waiting:
  .asciiz "Reading buttons"

nmi:
irq:
  PHA
  LDA IFR
  BPL .buttons ; Not the VIA?
  AND #%00000100 ; SR
  BEQ .buttons
    JSR toggle_led
    LDA #1
    STA SHIFT_READY
    LDA SR
    STA SHIFTED_BYTE
  .buttons:
    JSR read_buttons
  .done:
  PLA
  RTI

  .org PROGRAM_NMI_VECTOR
  .word nmi
  .org PROGRAM_RESET_VECTOR
  .word reset
  .org PROGRAM_IRQ_VECTOR
  .word irq
