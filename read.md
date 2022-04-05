1. 考量一个变量的可变性 “重型”数据结构 适当的使用 可变去修改一个实例 可能比赋值或者是 重新返回以后一个新分配的 实例更加的有效率 ？ 但是当数据结构 较为 “轻量” 采用函数式 创建一个新变量来进行 赋值 可能会使代码 更加的容易理解 是否要为可读性 而去损失 少许的性能
2. 变量的不可变性 ？== 常量 默认不可变 而且总是不可变
   - const
   - 显示的标注值的类型
   - 可以存在于任何的作用域
   - 只能将常量绑定到一个表达式上 无法讲一个函数的返回值 或者将其他的需要在 运行时 计算的值 绑定到常量上
     > 约定全部大写 并且以 下划线链接 （数字类型的常插入 \_ 来提高可读性）

```rust
    const MAX_POINTS:u32 = 100_000;
```

note: 常量只有在自己声明的作用于中有效

3. 隐藏 新声明的变量 覆盖掉了 同名的变量 称之为 shadow
   - 隐藏的机制不同于一个变量的 mut ， 始终是为了 这个变量在操作完成之后 能够保证自己的不可变性
   - 在重新对同名的变量赋值的条件下 可以对类型进行 变化 与之相不 mut 会在编译的 时候报错

```rust

    let x = 4;

    let x = 5;

```

## 数据类型

### 整数类型

- i 有符号 u 无符号
- n (8 16 32[默认] 64 128 safe[主要用于一些集合的索引])
- 有符号的整数的范围 `-2^(n - 1) -1 ~ 2^(n - 1) - 1`
- 无符号的整数范围 `0 ~ 2^n - 1`

```rust
       let int = 10u8; // let int:u8 = 10;

```

note: 十六进制 0X 八进制 0o 二进制 0b 好有一种特殊的 u8 类型的 b'A'

> 整数溢出