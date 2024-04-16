

https://www.cnblogs.com/RioTian/p/18072690

- 引言
- Trait 基础
- 自动 Trait
- 泛型 Trait
- 格式化 Trait
- 操作符 Trait
- 转换 Trait
- 错误处理
- 迭代器 Trait
- I/O Trait
- 总结

`use std::prelude::v1::*`

- AsMut
- AsRef
- Clone
- Copy
- Default
- Drop
- Eq
- Fn
- FnMut
- FnOnce
- From
- Into
- ToOwned
- IntoIterator
- Iterator
- PartialEq
- PartialOrd
- Send
- Sized
- Sync
- ToString
- Ord

Derive Macros

- Clone
- Copy
- Debug
- Default
- Eq
- Hash
- Ord
- PartialEq
- PartialOrd


### Formatting Traits

Trait | Placeholder | Description
--|--|--
Display | {} | 显示表示
Debug | {:?} | 调试表示
Octal | {:o} | 八进制表示
LowerHex | {:x} | 小写十六进制表示
UpperHex | {:X} | 大写十六进制表示
Pointer | {:p} | 内存地址
Binary | {:b} | 二进制表示
LowerExp | {:e} | 小写指数表示
UpperExp | {:E} | 大写指数表示