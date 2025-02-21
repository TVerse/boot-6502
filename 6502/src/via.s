.import "constants.inc"

.export init
.export inc_timer
.export read_timer

.struct Via
    .org __VIA_LOAD__
    PORTB .byte
    PORTA .byte
    DDRB .byte
    DDRA .byte
    T1CL .byte
    T1CH .byte
    T1LL .byte
    T1LH .byte
    T2CL .byte
    T2CH .byte
    SR .byte
    ACR .byte
    PCR .byte
    IFR .byte
    IER .byte
    PORTA_NOHS .byte
.endstruct

.struct Timer
    ten_ms_counter .word
.endstruct

.bss
.tag T

INTERRUPT_FREQ_MS = 10
; Minus 2 cycles for starting the interrupt
CYCLES_PER_INTERRUPT = (Constants::CLOCK_SPEED_HZ / 1000) * INTERRUPT_FREQ_MS - 2

LCD_DATA_MASK = %00111100 ; On port B
LCD_ENABLE = %10000000
LCD_RW = %00000010
LCD_RS = %00000001

.code

.proc init
    jsr init_timer
    cli
    jmp init_lcd
.endproc

.proc init_timer
    stz T + Timer::ten_ms_counter
    stz T + Timer::ten_ms_counter + 1

    lda #<CYCLES_PER_INTERRUPT
    sta Via::T1CL
    lda #>CYCLES_PER_INTERRUPT
    sta Via::T1CH

    lda Via::ACR
    and #%01111111 ; TODO constant
    ora #%01000000
    sta Via::ACR

    lda #%11000000
    sta Via::IER
    rts
.endproc

; To be called from IRQ handler
.proc inc_timer
    BIT Via::T1CL ; Ack interrupt
    inc T + Timer::ten_ms_counter
    bne @no_overflow
    inc T + Timer::ten_ms_counter + 1
@no_overflow:
    rts
.endproc

.proc read_timer
; Returns hi in A, low in Y
    ldy T + Timer::ten_ms_counter
    lda T + Timer::ten_ms_counter + 1
    rts
.endproc

.proc delay
; A: num 10ms cycles to wait
; May return up to 9ms early
    clc
@loop:
    ; Read low byte only, we only get one byte in
    cmp T + Timer::ten_ms_counter
    wai
    bne @loop
    rts
.endproc

.proc init_lcd
    ; Reset
    lda #13
    jsr delay
    lda #%00110000
    jsr lcd_command_send_upper_nibble
    lda #3
    jsr delay
    lda #%00110000
    jsr lcd_command_send_upper_nibble
    lda #3
    jsr delay
    lda #%00110000
    jsr lcd_command_send_upper_nibble
    lda #3
    jsr delay
    lda #%00110000
    jsr lcd_command_send_upper_nibble
    ; Enable 4bit interface
    lda #%00100000
    jsr lcd_command_send_upper_nibble
    ; Now we can start doing busy flag checks
    ; Function set: lines and font
    lda #%00100000
    jsr lcd_command
    ; Display off
    lda #%00001000
    jsr lcd_command
    ; Display clear
    lda #%00000001
    jsr lcd_command
    ; Set entry mode: increment, no shift
    lda #%00000110
    jsr lcd_command
    ; Set: display on, cursor off, blink off
    lda #%00001100
    jsr lcd_command
    rts
.endproc

.proc lcd_command
    jsr lcd_wait_for_busy
    pha
    jsr lcd_command_send_upper_nibble
    pla
    jmp lcd_command_send_lower_nibble
.endproc

.proc lcd_wait_for_busy
    pha
    lda Via::DDRB
    pha
    ; Set data pins as input
    ; TODO make sure PB6 stays correct for ACIA!
    lda #(LCD_ENABLE | LCD_RS | LCD_RW)
    sta Via::DDRB
@poll:
    ; Read first nibble
    lda #RW
    sta Via::PORTB
    eor #E
    sta Via::PORTB
    lda Via::PORTB
    pha ; Store it somewhere so we can read the second
    ; Throw away second nibble
    lda #RW
    sta Via::PORTB
    eor #E
    sta Via::PORTB
    lda Via::PORTB ; Do we need to actually read this out?

    pla
    and #%00100000
    cmp #%00100000
    bne @poll ; BF set: loop
    ; Unset E bit
    lda #RW
    sta Via::PORTB
    pla
    sta Via::DDRB
    pla
    rts
.endproc

.proc lcd_command_send_upper_nibble
    lsr
    lsr
    and #LCD_DATA_MASK
    sta Via::PORTB
    eor #LCD_ENABLE
    sta Via::PORTB
    eor #LCD_ENABLE
    sta Via::PORTB
    rts
.endproc

.proc lcd_command_send_lower_nibble
    asl
    asl
    and #LCD_DATA_MASK
    sta Via::PORTB
    eor #LCD_ENABLE
    sta Via::PORTB
    eor #LCD_ENABLE
    sta Via::PORTB
    rts
.endproc
