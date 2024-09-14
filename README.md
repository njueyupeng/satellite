
# Satellite-pro ⚡️

[![Support WebAssembly](https://img.shields.io/badge/webAssembly-1)](https://crates.io/crates/satellite)
[![Crates.io License](https://img.shields.io/crates/l/satellite)](https://crates.io/crates/satellite)
[![Static Badge](https://img.shields.io/badge/Rust-v1.8.0+-orange)](https://crates.io/crates/satellite)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/satellite)](https://crates.io/crates/satellite)




A high-performance library for satellite propagation using TLEs and powered by Rust, inspired by [satellite-js](https://github.com/shashwatak/satellite-js). Provides the functions necessary for SGP4/SDP4 calculations. Also provides functions for coordinate transforms.

This library's is a superset of [satellite-js](https://github.com/shashwatak/satellite-js)。Its API is almost identical to that of satellite.js, and due to the use of Rust and WASM, its performance is several times faster.


## Speed 

> 🚧 **Work in Progress**
>
> Ideally, this library should be faster than satellite.js, but currently, not all APIs are meeting the expected speed, and improvements are needed.


## Installation

### npm
Attention: The package name is `satellite-pro`, which is different from the original package name.

```bash
npm install satellite-pro --save
```



### Rust
```toml
[dependencies]
satellite = "0.1"
```


## Usage

### JS
```javascript
import * as satellite from "satellite-pro";

const tleLine1 = "1 44714C 19074B   24257.74770833  .00012054  00000+0  80755-3 0  2576";
const tleLine2 = "2 44714  53.0541  99.4927 0001373  86.0479  80.2511 15.06391223    18";

const satrec = satellite.twoline2satrec(tleLine1, tleLine2);
const _position_and_velocity = satellite.propagate(satrec, 2024, 9, 22, 12, 12, 12, 0);

```
⚠️：Your project should support webAssembly.

If you are using Vite as your build tool, you can follow the steps below.
```bash
npm install vite-plugin-wasm vite-plugin-top-level-await --save-dev
```
vite.config.js
```javascript
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// ...
  plugins: [ 
            wasm(),
            topLevelAwait(),
            //... 
  ]
//
```

### Rust
```rust
use satellite;

pub fn main(){
    let tle_line1 = "1 44714C 19074B   24257.74770833  .00012054  00000+0  80755-3 0  2576";
    let tle_line2 = "2 44714  53.0541  99.4927 0001373  86.0479  80.2511 15.06391223    18";
    let mut satrec = satellite::twoline2satrec(tle_line1, tle_line2);
    let _position_and_velocity = satellite::propagate(&mut satrec, 2024.0, 9.0, 22.0, 12.0, 12.0, 12.0, 0.0);
}
```



## todo


 - [ ] Performance optimization after compiling to wasm
 - [ ] Add testCase

<!-- - [ ] Rust
- [ ] WebAssembly
- [ ] TypeScript
- [ ] Documentation
- [ ] Test
- [ ] Contributing
- [ ] Changelog
- [ ] Contributors
- [ ] Donate
- [ ] FAQ -->



<!-- jdsatepoch
aycof
omgcof
xlcof
gsto -->
<!-- 
9.5 确保结果正确
9.6-9.7 添加测试用例
9.8-9.9 添加wasm支持
9.10-9.11 添加对比测试
9.12-9.13 添加文档，中英文
9.12-9.13 发布正式版本 -->

