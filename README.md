# 用于Rust std标准库的Unsafe API 的SP标注

遇到问题：

1. ValidNum 的false case如何设计？目前遇到的范围都是该类型的Min和Max之间。
2. as_unitit_ref的SP标注需要再考虑一下，该API可以允许指针未初始化
3. 一些API的safety要求没有找到

记录到的API。

1. core::ptr::const_ptr::as_uninit_ref 不需要初始化，重新考虑ValidPtr2Ref这个SP
2. core::ptr::const_ptr::as_uninit_slice 4,5的case存在问题，不知道如何合理设计
3. core::mem::manually_drop::take的Ownning也是hazard，写出来的case不会检测出UB
4. core::char::from_u32_unchecked为什么是ValidString不是ValidNum？
5. core::alloc::shrink的ValidNum检测了参数之间的比较关系，和参数的大小，但是参数的大小只会导致Layout返回Err
6. core::alloc::layout::for_value_raw 的几个SP需要增加或语义, 构造出未初始化长度的slice和trait object有难度
7. ValidCstr: 目前检测不出有问题

检测Unsound：有的情况通过，但是有的情况不通过

三种方法：path, generic, argument

设计方法：针对每个SP，对应该要进行的检查不进行处理

* InBounded: 不进行边界检查
* ValidNum：不验证数值范围，或者使用数值范围外的值， 但是byte_sub的测试结果有点奇怪
* Valid2Ptr:构造三种情况。1. 不检验初始化。 2. 对指针进行偏移。 3. 对指针直接赋值 。问题在情况3，可见const_ptr::as_ref
* ValidPtr: 1. 不检验指针范围 2. 不检验对齐 3. 对于零长度切片不检验对齐（不知道如何处理）4. 不检验是否是同一个分配
* Alias： 使用赋值操作
* Alive： 没有写case
* Init: 不验证初始化
* CopyTrait：使用没有Impl Copy的数据结构
* Allocated: Hazard SP。1. 检测是否Use-after-free。 2. 检测是否Double Drop
* ValidString: 不检测输入
* Layout: 制造不符合原Layout的新Layout作为输入
* ValidSlice, ValidTraitObj ： 没有写case
