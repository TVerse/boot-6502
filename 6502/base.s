.include "stack.inc"
.include "via.inc"
.include "lcd.inc"
.include "acia.inc"
.include "zeropage.inc"
.include "memory.inc"

.import TEN_MS_COUNTER_ADDR

.code

reset_base:
  ; Reset decimal flag
    cld

  ; Set hardware stack pointer
    ldx #$FF
    txs

    ; Copy data segment
    jsr copy_data

  ; Set software stack pointer
    ldx #SOFTWARE_STACK_START

    jsr via_init

  ; Enable interrupts
    cli

  ; Initialize LCD:
  ; 4-bit, 2 line, 5x8 characters, move right
    jsr initialize_lcd

    lda #LCD_CLEAR
    jsr lcd_instruction

    jsr acia_init

    literal initialized_base
    jsr print_null_terminated_string_stack
    pop

    literal 50
    jsr delay

    lda #%00000001
    jsr lcd_instruction

    literal 50
    jsr delay

loop:
    ;jsr acia_block_handle_message
    wai
    jmp loop

nmi_base:
    pha
    lda ACIA_STATUS_RESET_REGISTER
    and #%00001000
    beq @done
    phy
    jsr acia_receive_byte
    ply
@done:
    pla
    rti

irq_base:
    pha
    lda VIA_IFR
    asl                             ; IRQ
    bcc @done                       ; Not the VIA
    asl                             ; T1
    bcs @timer
    asl                             ; T2
    ;bcs @transmit
  ; ASL ; CB1
  ; ASL ; CB2
  ; ASL ; Shift
  ; ASL ; CA1
  ; ASL ; CA2
    bra @done
@timer:
    bit VIA_T1CL
    inc TEN_MS_COUNTER_ADDR
    bne @no_overflow
    inc TEN_MS_COUNTER_ADDR + 1
@no_overflow:
    phy
    jsr acia_parse_buffer
    ply
    bra @done
@transmit:
    bit VIA_T2CL
    phy
    ;jsr acia_transmit_byte
    ply
@done:
    pla
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
