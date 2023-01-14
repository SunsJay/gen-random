# gen-random
A rust lib for generating custom random numbers

In most development scenarios, we need to generate random numbers that include letters, numbers, and sometimes sybomls.

So, this lib includes these functions, we just need to set length and kind of randmon number.

![Example](https://repo-1256831547.cos.ap-shanghai.myqcloud.com/2023/202301141355517.png)


```rust
use gen_random
// Custom random number's length and kind(number ? letter ? symbol ? or number and letter ...)
let random = Custom::new(5, CharSetKind::Number).generate();
```

`