  .include constants.s

  .org ROM_START_ADDR

jump_table:


  .include stack.s
  .include peripherals.s

reset_base:
  ; Disable and stop interrupts
  SEI
  LDA #%01111111
  STA IER
  STA IFR

  ; Reset decimal flag
  CLD

  ; Set hardware stack pointer
  LDX #$FF
  TXS

  ; Set software stack pointer
  LDX #SOFTWARE_STACK_START

  ; Set data direction
  LDA #DEFAULT_DDRA
  STA DDRA
  LDA #DEFAULT_DDRB
  STA DDRB

  ; Put ports in known state
  STZ PORTA
  STZ PORTB

  ; Reset counter
  STZ FIVE_MILLISECOND_COUNTER_ADDR
  STZ FIVE_MILLISECOND_COUNTER_ADDR + 1

  ; Don't send interrupt to program yet
  LDA $FF
  STA INITIALIZATION_DONE

  LDA #%00000001
  ; Start 5ms clock, 5000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 4998 wait
  LITERAL $1386
  JSR enable_timer

  ; Enable interrupts
  CLI

  LDA #%00000001
  STA PORTA

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
  JSR initialize_lcd

  LDA #%00001100
  JSR lcd_instruction

  LITERAL initialized_base
  JSR print_string_stack

  STZ PORTA

  LITERAL 400
  JSR delay

  LDA #%00000001
  JSR lcd_instruction
  
  JMP (PROGRAM_RESET_VECTOR)
  
nmi_base:
  JMP (PROGRAM_NMI_VECTOR)
irq_base:
  BIT IFR
  BPL .program_irq ; If VIA:
  BVC .program_irq ; If timer 1:
    BIT T1CL
    INC FIVE_MILLISECOND_COUNTER_ADDR
    BNE .no_overflow
    INC FIVE_MILLISECOND_COUNTER_ADDR + 1
  .no_overflow:
  .program_irq:
    BIT INITIALIZATION_DONE
    BMI .not_done
    JMP (PROGRAM_IRQ_VECTOR)
  .not_done:
    RTI

; ( cycles -- )
enable_timer:
  LDA 0, X
  STA T1CL
  LDA 1, X
  STA T1CH

  POP

  LDA ACR
  AND #$7F
  ORA #$40
  STA ACR
  
  LDA #%11000000
  STA IER

  RTS

initialize_lcd:
  ; Reset
  LITERAL_8BIT 25
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay 
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay
  LDA #%00110000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay
  ; Set 4bit interface
  LDA #%00100000
  JSR lcd_send_upper_nibble
  LITERAL_8BIT 5
  JSR delay

  ; Software initialize
  LDA #%00101000
  JSR lcd_instruction
  LDA #%00001000
  JSR lcd_instruction
  LDA #%00000001
  JSR lcd_instruction

  LITERAL_8BIT 200
  JSR delay

  LDA #%00000110
  JSR lcd_instruction
  RTS

; ( 5ms_cycle_count -- )
; Clobbers A
delay:
  CLC
  LDA FIVE_MILLISECOND_COUNTER_ADDR
  ADC 0, X
  STA 0, X
  LDA FIVE_MILLISECOND_COUNTER_ADDR + 1
  ADC 1, X
  STA 1, X
  .loop:
    wai
    LDA 0, X
    CMP FIVE_MILLISECOND_COUNTER_ADDR
    BNE .loop
    LDA 1, X
    CMP FIVE_MILLISECOND_COUNTER_ADDR + 1
    BNE .loop
  POP
  RTS

  ; Harder and ATM broken version
  ; Should do atomic reads into X, Y
  ; read_timer:
  ;   LDX FIVE_MILLISECOND_COUNTER_ADDR + 1
  ;   LDY FIVE_MILLISECOND_COUNTER_ADDR
  ;   CPX FIVE_MILLISECOND_COUNTER_ADDR + 1
  ;   BNE read_timer
  ;   RTS
  ; 
  ; ; ( 5ms_cycle_count -- )  
  ; delay:
  ;   PHY
  ;   PHX
  ;   JSR read_timer
  ;   TXA
  ;   PLX
  ;   INX2
  ;   STA 1, X
  ;   TYA
  ;   STA 0, X
  ;   PLY
  ;   CLC
  ;   LDA 0, X
  ;   ADC 2, X
  ;   STA 2, X
  ;   LDA 1, X
  ;   ADC 3, X
  ;   STA 3, X
  ;   POP
  ;   .loop:
  ;     PHY
  ;   .high_byte:
  ;     WAI
  ;     PHX
  ;     JSR read_timer
  ;     TXA
  ;     PLX
  ;     CMP 1, X
  ;     BNE .high_byte
  ;   .low_byte:
  ;     TYA
  ;     PLY
  ;     CMP FIVE_MILLISECOND_COUNTER_ADDR
  ;     BNE .loop
  ;   POP
  ;   RTS

initialized_base:
  .asciiz "Initialized!"

  .org VECTORS_START_ADDR
  .word nmi_base
  .word reset_base
  .word irq_base

