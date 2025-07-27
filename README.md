# 用于Rust std标准库的Unsafe API 的SP标注

遇到问题：

1. ValidNum 的false case如何设计？目前遇到的范围都是该类型的Min和Max之间。
2. as_unitit_ref的SP标注需要再考虑一下，该API可以允许指针未初始化
3. 一些API的safety要求没有找到

记录到的API。

1. core::ptr::const_ptr::byte_offset，由于其参数为isize，无法构造出超出范围的参数。对应的false case实际上对应的是out-of-bound case
