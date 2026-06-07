MEMORY
{
  /* 总共 512k Flash */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  
  /* 原本 640K 的 RAM，拿出 1K 给 PANDUMP，剩下 639K 给普通程序 */
  RAM   : ORIGIN = 0x20000000, LENGTH = 127K
  
  /* 专门用于存放 panic 信息的 1KB 区域 */
  PANIC_MSG : ORIGIN = 0x20000000 + 127K, LENGTH = 1K
}
_panic_dump_start = ORIGIN(PANIC_MSG);
_panic_dump_end   = ORIGIN(PANIC_MSG) + LENGTH(PANIC_MSG);
