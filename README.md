# 用于Rust std标准库的Unsafe API 的SP标注

遇到问题：

1. ValidNum 的false case如何设计？目前遇到的范围都是该类型的Min和Max之间。
2. as_unitit_ref的SP标注需要再考虑一下，该API可以允许指针未初始化
