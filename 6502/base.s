  .include memory_map.s

  .org ROM_START_ADDR
  .include stack.s
  .include via.s
  .include acia.s
  .include debug.s

  ; Start 5ms clock, 5000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 4998 wait = $1368
  .macro ENABLE_TIMER
  LDA #$0E
  STA VIA_T1CL
  LDA #$27
  STA VIA_T1CH

  LDA VIA_ACR
  AND #%01111111
  ORA #%01000000
  STA VIA_ACR

  LDA #%11000000
  STA VIA_IER
  .endmacro

reset_base:
  ; Reset decimal flag
  CLD

  ; Set hardware stack pointer
  LDX #$FF
  TXS

  ; Set software stack pointer
  LDX #SOFTWARE_STACK_START

  ; Set data direction
  LDA #DEFAULT_DDRA
  STA VIA_DDRA
  LDA #DEFAULT_DDRB
  STA VIA_DDRB

  ; Put ports in known state
  STZ VIA_PORTA
  STZ VIA_PORTB

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

  ; Don't send interrupt to program yet
  LDA #$FF
  STA initialization_done
  STA program_reset

  ; Reset counter
  STZ ten_millisecond_counter_addr
  STZ ten_millisecond_counter_addr + 1

  ENABLE_TIMER

  ; Enable interrupts
  CLI

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
  INITIALIZE_LCD

  LDA #LCD_CLEAR
  JSR lcd_instruction

  JSR init_acia

  LITERAL initialized_base
  JSR print_null_terminated_string_stack
  POP

  LITERAL 50
  JSR delay

  LDA #%00000001
  JSR lcd_instruction

  LITERAL 50
  JSR delay

  JMP reset

nmi_base:
  PHA
  LDA ACIA_STATUS_RESET_REGISTERS
  AND #%00001000
  BEQ .done
  LDA ACIA_DATA_REGISTERS
  ;JSR print_char
.done:
  PLA
  JMP (program_nmi)
irq_base:
  PHA
  LDA VIA_IFR
  ASL ; IRQ
  BCC .program_irq ; Not the VIA
  ASL ; T1
  BCS .timer
  ASL ; T2
  BCS .transmit
  ; ASL ; CB1
  ; ASL ; CB2
  ; ASL ; Shift
  ; ASL ; CA1
  ; ASL ; CA2
  BRA .program_irq
.timer:
  BIT VIA_T1CL
  INC ten_millisecond_counter_addr
  BNE .no_overflow
  INC ten_millisecond_counter_addr + 1
.no_overflow:
  BRA .program_irq
.transmit:
  BIT VIA_T2CL
  PHY
  JSR transmit
  PLY
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
  LDA ten_millisecond_counter_addr
  ADC 0, X
  STA 0, X
  LDA ten_millisecond_counter_addr + 1
  ADC 1, X
  STA 1, X
  .loop:
    WAI
    LDA 0, X
    CMP ten_millisecond_counter_addr
    BNE .loop
    LDA 1, X
    CMP ten_millisecond_counter_addr + 1
    BNE .loop
  POP
  RTS

; ( string_pointer -- )
; Does not return
error:
  SEI
  LDA #%00000001
  JSR lcd_instruction
  LITERAL error_message
  JSR print_null_terminated_string_stack
  LDA 0,X
  ORA 1,X
  BEQ .loop
  ; .has_message:
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
  ;   LDX ten_millisecond_counter_addr + 1
  ;   LDY ten_millisecond_counter_addr
  ;   CPX ten_millisecond_counter_addr + 1
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
  ;     CMP ten_millisecond_counter_addr
  ;     BNE .loop
  ;   POP
  ;   RTS

initialized_base:
  .asciiz "Initialized!"

  .org VECTORS_START_ADDR
  .word nmi_base
  .word reset_base
  .word irq_base

