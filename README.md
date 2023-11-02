## cecelia

A very simple "programming language" which compiles to aarch64 assembly.

### Usage

**`main.cec`**:

```
//     Typing is optional here, as it will be inferred at compile-time.
//     ↓
let x: Integer = 5
let y: Integer = 6

// A top-level return statement will be treated as the process' exit code.
return x + y
```

**Equivalent aarch64 `.S` file**:

```S
.global _start
.align 2

_start:
        mov     w0, 5
        mov     w1, 4
        add     w0, w1, w0
        ret
```

### License

This project is licensed under the [MIT](https://choosealicense.com/licenses/mit/) license.
