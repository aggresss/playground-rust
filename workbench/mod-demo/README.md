

## Cheat Sheet

- 每一个文件代表一个 mod，文件中可以定义 mod，作为 nest mod；
- 文件夹表示为一个 mod 时，需要使用 mod.rs 进行 pub 声明；
- mod.rs 除了作为对外声明文件外，还可以作为同目录下的公用 mod，子 mod 通过 `use super::*` 可以访问 private elements；

## Reference

- https://blog.csdn.net/weixin_52437323/article/details/129007092
