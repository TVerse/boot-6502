.include "stack.inc"
.include "via.inc"

.importzp SOFTWARE_STACK_START
.import INITIALIZATION_DONE
.import TEN_MS_COUNTER_ADDR
.importzp LCD_CLEAR
.import lcd_instruction
.import init_acia
.import print_null_terminated_string_stack
.import acia_receive
.import acia_transmit
.import initialize_lcd
.import delay
.import reset
.import init_via
.import Via

.code

reset_base:
  ; Reset decimal flag
    cld

  ; Set hardware stack pointer
    ldx #$FF
    txs

  ; Set software stack pointer
    ldx #SOFTWARE_STACK_START

    jsr init_via

  ; Don't send interrupt to program yet
    lda #$FF
    sta INITIALIZATION_DONE

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
;  lda ACIA_STATUS_RESET_REGISTER
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
    lda Via+Via::IFR
    asl                             ; IRQ
    bcc @done                       ; Not the VIA
    asl                             ; T1
    bcs @timer
    asl                             ; T2
    bcs @transmit
  ; ASL ; CB1
  ; ASL ; CB2
  ; ASL ; Shift
  ; ASL ; CA1
  ; ASL ; CA2
    bra @done
@timer:
    bit Via+Via::T1CL
    inc TEN_MS_COUNTER_ADDR
    bne @no_overflow
    inc TEN_MS_COUNTER_ADDR + 1
@no_overflow:
    bra @done
@transmit:
    bit Via+Via::T2CL
    phy
    jsr acia_transmit
    ply
@done:
    pla
    bit INITIALIZATION_DONE
    rti

loop_base:
    wai
    jmp loop_base



.rodata
initialized_base:
.asciiz "Initialized!"

.segment "VECTORS"
.word nmi_base
.word reset_base
.word irq_base
