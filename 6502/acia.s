; Considerations:
; * Control RTS based on buffer size (need to be a bit early to account for in-transit data!)
; * Hook up to NMI
; * NMI is the only thing that writes the rx buffer
; * Do tx from IRQ with T2/PB6 count cause bug
;   * Transmit bit in status register is stuck on, irq constantly triggers and can't poll it either.
; * At 115200 baud we have 86 cycles @ 1MHz to finish up, but can't go that fast due to transmit bug.
;   * To get good counts we can't be faster than PIH2 / 4 == 250KHz, 9600 -> 153.6KHz (16 counts per symbol).
; * TODO can I replace counting with RC + Schmitt trigger on tx line?
;
; Is it possible to make the NMI overlap-safe?

  .include "via.inc"

  .import via_prep_for_transmit
  .import Via

  .export acia_receive
  .export acia_transmit
  .export init_acia
  .export blocking_transmit
  .export initiate_transmit
  .export write_transmit_byte

.struct Acia
  DataRegister .byte
  StatusResetRegister .byte
  CommandRegister .byte
  ControlRegister .byte
.endstruct

.enum RTSbEnabled
  Disabled = 0
  Enabled = 1
.endenum

.enum TxInProgress
  False = $FF
  True = $00
.endenum

; One byte with page-aligned buffers. (TODO not needed with indexed addressing?)
; Both pointers are increment-then-use (so pointing at the last byte read/written)
; Buffer is full if (write + 1) == read
; Buffer is empty if write == read
; If these are full pointers instead of an increment, does addressing become simpler?
; Might free up a register. But addition doesn't auto-carry anymore so align to page or do 16-bit add.
.struct AciaData
  RTSbEnabled .byte
  TxInProgress .byte
  TxBufferWritePtr .byte
  TxBufferReadPtr .byte
  RxBufferWritePtr .byte
  RxBufferReadPtr .byte
.endstruct

.struct AciaBuffers
  TxBuffer .res $0100
  RxBuffer .res $0100
.endstruct

TX_T2_PULSES = 180 ; 160 breaks in a weird way (on memcpy?) No further optimization

.segment "ACIA"
Acia: .tag Acia

; 10 symbols * 16 counts/symbol

.data
AciaData: .tag AciaData

.segment "BUFFERS"
.align $0100
AciaBuffers: .tag AciaBuffers

.code

init_acia:
  ; Set buffer pointers
  lda #$FF
  sta AciaData+AciaData::TxBufferWritePtr
  sta AciaData+AciaData::TxBufferReadPtr
  sta AciaData+AciaData::RxBufferWritePtr
  sta AciaData+AciaData::RxBufferReadPtr

  lda #TxInProgress::False
  sta AciaData+AciaData::TxInProgress
  lda #RTSbEnabled::Enabled
  sta AciaData+AciaData::RTSbEnabled

  ; Init rx buffer to all FF for easier testing
  ldy #0
  lda #$FF
@loop_rx:
  sta AciaBuffers+AciaBuffers::RxBuffer, Y
  iny
  bne @loop_rx

  ; Same but tx
  ldy #0
  lda #$FE
@loop_tx:
  sta AciaBuffers+AciaBuffers::TxBuffer, Y
  iny
  bne @loop_tx

  ; 1 stop bit, 8 bits, rcv baud rate, 9600 on crystal
  lda #%00011110
  ; 1 stop bit, 8 bits, rcv baud rate, 600 on crystal
  lda #%00010111
  sta Acia+Acia::ControlRegister
  ; No parity, normal mode, RTSB low, no tx interrupt, rx interrupt, data terminal ready (unused)
  lda #%11001001
  sta Acia+Acia::CommandRegister

  jsr via_prep_for_transmit
  rts

; Will start the transmit on the next T2 tick
initiate_transmit:
  bit AciaData+AciaData::TxInProgress
  bmi @done
  dec AciaData+AciaData::TxInProgress
  ; Start T2 by writing to the high byte
  pha
  lda #TX_T2_PULSES
  sta Via+Via::T2CL
  stz Via+Via::T2CH
  pla
@done:
  rts

blocking_transmit:
  jsr initiate_transmit
@block:
  bit AciaData+AciaData::TxInProgress
  bmi @block
  rts


; Clobbers Y
; Return a value instead of just initiating transmit?
write_transmit_byte:
  pha
  ; Check if buffer full
  lda AciaData+AciaData::TxBufferWritePtr
  inc
  cmp AciaData+AciaData::TxBufferReadPtr
  bne @ready
  ; Buffer full, initiate transmit and wait a bit
  jsr initiate_transmit
@wait:
  wai
  cmp AciaData+AciaData::TxBufferReadPtr
  bne @wait
@ready:
  tay
  pla
  sty AciaData+AciaData::TxBufferWritePtr
  sta AciaBuffers+AciaBuffers::TxBuffer, Y
  rts

; Called from NMI
; TODO set RTS line to prevent overflow
; TODO handle overrun n stuff
acia_receive:
  lda Acia+Acia::DataRegister
  inc AciaData+AciaData::RxBufferWritePtr
  ldy AciaData+AciaData::RxBufferWritePtr
  sta AciaBuffers+AciaBuffers::RxBuffer, Y
  tya
  clc
  sbc AciaData+AciaData::RxBufferReadPtr
  clc
  sbc #5
  bpl @done
  ; Less than 5 slots? Bring RTSb high to stop receive
  dec AciaData+AciaData::RTSbEnabled
  lda Acia+Acia::ControlRegister
  and #%11110011
  sta Acia+Acia::ControlRegister
@done:
  rts

; Must be called after reading the RX buffer
acia_allow_receive:
  lda AciaData+AciaData::RTSbEnabled
  bne @done
  lda AciaData+AciaData::RxBufferWritePtr
  clc
  sbc AciaData+AciaData::RxBufferReadPtr
  clc
  sbc #10
  bne @done
  ; More than 10 slots? Bring RTSb low to start receive
  lda Acia+Acia::ControlRegister
  and #%11111011
  ora #%00001000
  sta Acia+Acia::ControlRegister
  inc AciaData+AciaData::RTSbEnabled
@done:
  rts

; Called from IRQ
; Ignore CTS, we cannot read the line directly and transmit status is stuck on
; TODO can AND CTS with PB6?
acia_transmit:
  ; Check if buffer empty
  lda AciaData+AciaData::TxBufferWritePtr
  cmp AciaData+AciaData::TxBufferReadPtr
  beq @empty
  ; If not, send a byte and reinit T2
  inc AciaData+AciaData::TxBufferReadPtr
  ldy AciaData+AciaData::TxBufferReadPtr
  lda AciaBuffers+AciaBuffers::TxBuffer, Y
  sta Acia+Acia::DataRegister
  stz Via+Via::T2CH
  bra @done
@empty:
  lda #TxInProgress::False
  sta AciaData+AciaData::TxInProgress
@done:
  rts
