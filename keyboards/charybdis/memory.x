MEMORY
{
  /* NOTE 1 K = 1 KiB = 1024 bytes */
  /* nRF52840 with Adafruit nRF52 bootloader (nice!nano_v2), used by dongle,
     left, and right — all three are the same board. */
  /* Reserve 0xCC000..0xEC000 for RMK storage (keyboard.toml [storage]). */
  FLASH : ORIGIN = 0x00001000, LENGTH = 812K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K
}
