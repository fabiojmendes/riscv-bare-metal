.globl _start

.section ".text.boot"
_start:
  la t0, _memory_end
  mv sp, t0
  call k_main

.global system_off
system_off:
  beq t1, t1, system_off
