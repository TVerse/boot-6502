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

  ; Start 10ms clock, 10000 cycles @ 1MHz
  ; 2 cycles for starting the interrupt = 9998 wait = $270E
  .macro ENABLE_TIMER
  lda #$0E
  sta VIA_T1CL
  lda #$27
  sta VIA_T1CH

  lda VIA_ACR
  and #%01111111
  ora #%01000000
  sta VIA_ACR

  lda #%11000000
  sta VIA_IER
  .endmacro

reset_base:
  ; Reset decimal flag
  cld

  ; Set hardware stack pointer
  ldx #$FF
  txs

  ; Set software stack pointer
  ldx #SOFTWARE_STACK_START

  ; Set data direction
  lda #DEFAULT_DDRA
  sta VIA_DDRA
  lda #DEFAULT_DDRB
  sta VIA_DDRB

  ; Put ports in known state
  stz VIA_PORTA
  stz VIA_PORTB

  ; Don't send interrupt to program yet
  lda #$FF
  sta INITIALIZATION_DONE

  ; Reset counter
  stz TEN_MS_COUNTER_ADDR
  stz TEN_MS_COUNTER_ADDR + 1

  ENABLE_TIMER

  ; Enable interrupts
  cli

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
  jsr initialize_lcd

  lda #LCD_CLEAR
  jsr lcd_instruction

  jsr init_acia

  literal initialized_base
  jsr print_null_terminated_string_stack
  pop

  literal 50
  jsr delay

  lda #%00000001
  jsr lcd_instruction

  literal 50
  jsr delay

  jmp reset

nmi_base:
  pha
  lda ACIA_STATUS_RESET_REGISTERS
  and #%00001000
  beq @done
  phy
  jsr acia_receive
  ply
@done:
  pla
  rti
irq_base:
  pha
  lda VIA_IFR
  asl ; IRQ
  bcc @program_irq ; Not the VIA
  asl ; T1
  bcs @timer
  asl ; T2
  bcs @transmit
  ; ASL ; CB1
  ; ASL ; CB2
  ; ASL ; Shift
  ; ASL ; CA1
  ; ASL ; CA2
  bra @program_irq
@timer:
  bit VIA_T1CL
  inc TEN_MS_COUNTER_ADDR
  bne @no_overflow
  inc TEN_MS_COUNTER_ADDR + 1
@no_overflow:
  bra @program_irq
@transmit:
  bit VIA_T2CL
  phy
  jsr acia_transmit
  ply
@program_irq:
  pla
  bit INITIALIZATION_DONE
  bne @not_done
  rti
@not_done:
  rti

loop_base:
  wai
  jmp loop_base

; ( string_pointer -- )
; Does not return
error:
  sei
  lda #%00000001
  jsr lcd_instruction
  literal error_message
  jsr print_null_terminated_string_stack
  lda 0,X
  ora 1,X
  beq @loop
  ; .has_message:
    lda #%11000000 ; Jump to second row
    jsr lcd_instruction
    jsr print_null_terminated_string_stack
  @loop:
    wai
    jmp @loop

error_message: .asciiz "ERROR: "

  ; Harder and ATM broken version
  ; Should do atomic reads into X, Y
  ; read_timer:
  ;   ldx TEN_MS_COUNTER_ADDR + 1
  ;   ldy TEN_MS_COUNTER_ADDR
  ;   cpx TEN_MS_COUNTER_ADDR + 1
  ;   bne read_timer
  ;   rts
  ;
  ; ; ( 5ms_cycle_count -- )
  ; delay:
  ;   phy
  ;   phx
  ;   jsr read_timer
  ;   txa
  ;   plx
  ;   inx2
  ;   sta 1, X
  ;   tya
  ;   sta 0, X
  ;   ply
  ;   clc
  ;   lda 0, X
  ;   adc 2, X
  ;   sta 2, X
  ;   lda 1, X
  ;   adc 3, X
  ;   sta 3, X
  ;   pop
  ;   .loop:
  ;     phy
  ;   .high_byte:
  ;     wai
  ;     phx
  ;     jsr read_timer
  ;     txa
  ;     plx
  ;     cmp 1, X
  ;     bne .high_byte
  ;   .low_byte:
  ;     tya
  ;     ply
  ;     cmp TEN_MS_COUNTER_ADDR
  ;     bne .loop
  ;   pop
  ;   rts

initialized_base:
  .asciiz "Initialized!"

  .segment "VECTORS"
  .word nmi_base
  .word reset_base
  .word irq_base

