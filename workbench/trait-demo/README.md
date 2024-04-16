

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


### Operator Traits

- Comparison Traits
- Arithmetic Traits
- Closure Traits
- Other Traits

Trait(s) | 分类（Category） | 操作符（Operator(s)） | 描述（Description）
--|--|--|--
Eq, PartialEq | 比较 | == | 相等
Ord, PartialOrd | 比较 | <, >, <=, >= | 比较
Add | 算术 | + | 相加
AddAssign | 算术 | += | 相加并赋值
BitAnd | 算术 | & | 按位与
BitAndAssign | 算术 | &= | 按位与并赋值
BitXor | 算术 | ^ | 按位异或
BitXorAssign | 算术 | ^= | 按位异或并赋值
Div | 算术 | / | 除
DivAssign | 算术 | /= | 除并赋值
Mul | 算术 | * | 乘
MulAssign | 算术 | *= | 乘并赋值
Neg | 算术 | - | 一元求反
Not | 算术 | ! | 一元逻辑求反
Rem | 算术 | % | 求余
RemAssign | 算术 | %= | 求余并赋值
Shl | 算术 | << | 左移
ShlAssign | 算术 | <<= | 左移并赋值
Shr | 算术 | >> | 右移
ShrAssign | 算术 | >>= | 右移并赋值
Sub | 算术 | - | 减
SubAssign | 算术 | -= | 减并赋值
Fn | 闭包 | (...args) | 不可变闭包调用
FnMut | 闭包 | (...args) | 可变闭包调用
FnOnce | 闭包 | (...args) | 一次性闭包调用
Deref | 其他 | * | 不可变解引用
DerefMut | 其他 | * | 可变解引用
Drop | 其他 | - | 类型析构
Index | 其他 | [] | 不可变索引
IndexMut | 其他 | [] | 可变索引
RangeBounds | 其他 | .. | 区间

### Conversion Traits

- From
- Into
- TryFrom
- TryInto
- AsRef
- AsMut
- Borrow
- BorrowMut
- ToOwned

### Iteration Traits

- IntoIterator
- FromIterator
