  .include "stack.s"

  .importzp SOFTWARE_STACK_START
  .import INITIALIZATION_DONE
  .import TEN_MS_COUNTER_ADDR
  .import VIA_DDRA
  .import VIA_DDRB
  .importzp DEFAULT_DDRA
  .importzp DEFAULT_DDRB
  .import VIA_PORTA
  .import VIA_PORTB
  .importzp LCD_CLEAR
  .import lcd_instruction
  .import init_acia
  .import print_null_terminated_string_stack
  .import ACIA_STATUS_RESET_REGISTERS
  .import acia_receive
  .import VIA_IFR
  .import acia_transmit
  .import initialize_lcd
  .import VIA_T2CL
  .import delay
  .import VIA_IER
  .import VIA_ACR
  .import VIA_T1CH
  .import VIA_T1CL
  .import reset

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

  ; Don't send interrupt to program yet
  LDA #$FF
  STA INITIALIZATION_DONE

  ; Reset counter
  STZ TEN_MS_COUNTER_ADDR
  STZ TEN_MS_COUNTER_ADDR + 1

  ENABLE_TIMER

  ; Enable interrupts
  CLI

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
  JSR initialize_lcd

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
  BEQ @done
  PHY
  JSR acia_receive
  PLY
@done:
  PLA
  RTI
irq_base:
  PHA
  LDA VIA_IFR
  ASL ; IRQ
  BCC @program_irq ; Not the VIA
  ASL ; T1
  BCS @timer
  ASL ; T2
  BCS @transmit
  ; ASL ; CB1
  ; ASL ; CB2
  ; ASL ; Shift
  ; ASL ; CA1
  ; ASL ; CA2
  BRA @program_irq
@timer:
  BIT VIA_T1CL
  INC TEN_MS_COUNTER_ADDR
  BNE @no_overflow
  INC TEN_MS_COUNTER_ADDR + 1
@no_overflow:
  BRA @program_irq
@transmit:
  BIT VIA_T2CL
  PHY
  JSR acia_transmit
  PLY
@program_irq:
  PLA
  BIT INITIALIZATION_DONE
  BNE @not_done
  RTI
@not_done:
  RTI

loop_base:
  WAI
  JMP loop_base

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
  BEQ @loop
  ; .has_message:
    LDA #%11000000 ; Jump to second row
    JSR lcd_instruction
    JSR print_null_terminated_string_stack
  @loop:
    WAI
    JMP @loop

error_message: .asciiz "ERROR: "

  ; Harder and ATM broken version
  ; Should do atomic reads into X, Y
  ; read_timer:
  ;   LDX TEN_MS_COUNTER_ADDR + 1
  ;   LDY TEN_MS_COUNTER_ADDR
  ;   CPX TEN_MS_COUNTER_ADDR + 1
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
  ;     CMP TEN_MS_COUNTER_ADDR
  ;     BNE .loop
  ;   POP
  ;   RTS

initialized_base:
  .asciiz "Initialized!"

  .segment "VECTORS"
  .word nmi_base
  .word reset_base
  .word irq_base

