OUTPUT_ARCH( "riscv" )


MEMORY
{
  flash : ORIGIN = 0x20400000, LENGTH = 512K
  ram   : ORIGIN = 0x80000000, LENGTH = 16K
}

PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

ENTRY(_start)
SECTIONS
{
  .text : {
    PROVIDE(_text_start = .);
    *(.text.boot) *(.text .text.*)
    PROVIDE(_text_end = .);
  } >flash AT>flash :text

  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
  } >flash AT>flash :text

  .data : {
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss :{
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
}
