1. Hướng dẫn flash 

- Tạo folder .cargo và file .cargo/config.xml

```
mkdir -p .cargo
vim .cargo/config.toml
```

- Tạo file memory.x

```
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}

```
- Copy to folder target
```
cp memory.x target/thumbv6m-none-eabi/release/build/
```
- flash
```
cargo flash --chip STM32F072RBTx --release
```


