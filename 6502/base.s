  .include constants.s

  .org ROM_START_ADDR
  .include stack.s
  .include peripherals.s
  .include debug.s
  .include comms.s

  ; Start 5ms clock, 5000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 4998 wait = $1368
  .macro ENABLE_TIMER
  LDA #$86
  STA T1CL
  LDA #$13
  STA T1CH

  LDA ACR
  AND #%01111111
  ORA #%01000000
  STA ACR

  LDA #%11000000
  STA IER
  .endmacro

  .macro INITIALIZE_LCD
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
  .endmacro

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
  STZ five_millisecond_counter_addr
  STZ five_millisecond_counter_addr + 1

  ; Put vectors in known state
  LDA #<rti
  STA program_nmi
  STA program_irq
  LDA #>rti
  STA program_nmi + 1
  STA program_irq + 1
  LDA #<loop_base
  STA program_reset
  LDA #>loop_base
  STA program_reset + 1

  ; Program not loaded
  STZ program_load_done

  ; Don't send interrupt to program yet
  LDA $FF
  STA initialization_done

  ENABLE_TIMER

  ; Enable interrupts
  CLI

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
  INITIALIZE_LCD

  LDA #%00001100
  JSR lcd_instruction

  LITERAL initialized_base
  JSR print_null_terminated_string_stack

  LITERAL 100
  JSR delay

  LDA #%00000001
  JSR lcd_instruction

  JSR set_input
  JSR init_comms

  .wait_for_program_loaded:
    WAI
    LDA program_load_done
    BEQ .wait_for_program_loaded
  
  JMP (program_reset)
  
nmi_base:
  JMP (program_nmi)
irq_base:
  PHA
  LDA IFR
  ASL ; IRQ
  BCC .program_irq ; Not the VIA
  ASL ; T1
  BCS .timer
  ASL ; T2
  ASL ; CB1
  ASL ; CB2
  ASL ; Shift
  ASL ; CA1
  BCS .comms
  ASL ; CA2
  BRA .program_irq
  .timer:
    BIT T1CL
    INC five_millisecond_counter_addr
    BNE .no_overflow
    INC five_millisecond_counter_addr + 1
  .no_overflow:
    BRA .program_irq
  .comms:
    JSR dispatch
  .program_irq:
    PLA
    BIT initialization_done
    BNE .not_done
    JMP (program_irq)
    .not_done:
      RTI

rti:
  RTI

loop_base:
  WAI
  JMP loop_base

; ( 5ms_cycle_count -- )
; Clobbers A
delay:
  CLC
  LDA five_millisecond_counter_addr
  ADC 0, X
  STA 0, X
  LDA five_millisecond_counter_addr + 1
  ADC 1, X
  STA 1, X
  .loop:
    WAI
    LDA 0, X
    CMP five_millisecond_counter_addr
    BNE .loop
    LDA 1, X
    CMP five_millisecond_counter_addr + 1
    BNE .loop
  POP
  RTS

; ( string_pointer -- )
; Does not return
error:
  SEI
  LDA #%00000001
  ;JSR lcd_instruction
  LITERAL error_message
  JSR print_null_terminated_string_stack
  LDA 0,X
  ORA 1,X
  BEQ .loop
  .has_message:
    LDA #%11000000 ; Jump to second row
    JSR lcd_instruction
    JSR print_null_terminated_string_stack
  .loop:
    WAI
    JMP .loop

error_message: .asciiz "ERROR: "

  ; Harder and ATM broken version
  ; Should do atomic reads into X, Y
  ; read_timer:
  ;   LDX five_millisecond_counter_addr + 1
  ;   LDY five_millisecond_counter_addr
  ;   CPX five_millisecond_counter_addr + 1
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
  ;     CMP five_millisecond_counter_addr
  ;     BNE .loop
  ;   POP
  ;   RTS

initialized_base:
  .asciiz "Initialized!"

  .org VECTORS_START_ADDR
  .word nmi_base
  .word reset_base
  .word irq_base

