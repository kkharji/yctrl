# Changelog
All notable changes to this project will be documented in this file.

## [unreleased]

### <!-- 0 -->Features

- <a href="https://github.com/kkharji/yctrl/commit/77c07a77550bbec9d7554d1b8430f9b573c6cc66"><tt>77c07a7</tt></a> Support custom configuration
- <a href="https://github.com/kkharji/yctrl/commit/bf4a5f06cc02018c0d896ccabdf1df90e157c116"><tt>bf4a5f0</tt></a> Configure auto-close empty spaces
- <a href="https://github.com/kkharji/yctrl/commit/5dce08a4b3b92d5ff30d4b76d95a2032d5e2bc1e"><tt>5dce08a</tt></a> Keep focus with rules

### <!-- 2 -->Refactor

- <a href="https://github.com/kkharji/yctrl/commit/d2bb654d88b9960ca8a9399fcc18f5dbc62e8e5e"><tt>d2bb654</tt></a> On space switch try focusing using mouse first
- <a href="https://github.com/kkharji/yctrl/commit/3712c4ffba4561b65d37da76da8e1b99712a2126"><tt>3712c4f</tt></a> Query interface
- <a href="https://github.com/kkharji/yctrl/commit/850e09d5d4cadcc91866d4112c462ce2a6a70621"><tt>850e09d</tt></a> Remove redundant code and test

### Miscellaneous Tasks

- <a href="https://github.com/kkharji/yctrl/commit/9d2b697221e7d62de12d8c8c6cd25028e226b1fb"><tt>9d2b697</tt></a> Update docs

## [0.1.3] - 2022-03-06

### <!-- 0 -->Features

- <a href="https://github.com/kkharji/yctrl/commit/da3989c1c036bf9de22b86edbcc35f53440bbd72"><tt>da3989c</tt></a> Setup event listener socket
- <a href="https://github.com/kkharji/yctrl/commit/251be6b8e0589a9fac5eeed872e44be2e92941cd"><tt>251be6b</tt></a> Accept arguments like window_id
- <a href="https://github.com/kkharji/yctrl/commit/1d3174c52d172ecd7dbda87173c068b1399efc7b"><tt>1d3174c</tt></a> Auto destroy space without windows
- <a href="https://github.com/kkharji/yctrl/commit/7e0713788433ebca073bd4fc1881065347fa7fae"><tt>7e07137</tt></a> Auto focus on space change
- <a href="https://github.com/kkharji/yctrl/commit/af68b8c7cb88915c8d30ab451b46b717bfdd4042"><tt>af68b8c</tt></a> Improve tracing info

### <!-- 1 -->Bug Fixes

- <a href="https://github.com/kkharji/yctrl/commit/afec8c131e04e3dd5d9c77392080e92f1b43fb95"><tt>afec8c1</tt></a> Serde_derive not found

### <!-- 2 -->Refactor

- <a href="https://github.com/kkharji/yctrl/commit/c082639b58fa564ae9edbd5b13a7cd9a92deb93c"><tt>c082639</tt></a> Move yabai constants event to constants
- <a href="https://github.com/kkharji/yctrl/commit/f969a64f68fad751e29ef0adf5c2ad4bf30b25fd"><tt>f969a64</tt></a> Rework event listener
- <a href="https://github.com/kkharji/yctrl/commit/0f14c3ed36bbe4e464d9dc9e0df3ec0075e6a1a4"><tt>0f14c3e</tt></a> Restructure source code
- <a href="https://github.com/kkharji/yctrl/commit/31073dcd1e76c04ca08224f09c1d696d248fbf12"><tt>31073dc</tt></a> Remove Yabai* from yabai.rs exports
- <a href="https://github.com/kkharji/yctrl/commit/a6f6efd7c32cc5664d215fa4b610d2c972df127c"><tt>a6f6efd</tt></a> Rename loop.rs runtime.rs
- <a href="https://github.com/kkharji/yctrl/commit/18d51b92b929f8c9cd5dd5538a95f30767fd34f6"><tt>18d51b9</tt></a> Switch to async operations with tokio
- <a href="https://github.com/kkharji/yctrl/commit/f44554a98c3ac42bb0f74b202f14c9a660c92868"><tt>f44554a</tt></a> Async processing

### <!-- 3 -->Enhancement

- <a href="https://github.com/kkharji/yctrl/commit/1917ddec9431dd867ca9bc7dbae1f7e0e2151fc5"><tt>1917dde</tt></a> Start yctrl runtime with no arguments

### Miscellaneous Tasks

- <a href="https://github.com/kkharji/yctrl/commit/e10f36d75f6b1f9d738175b87bdbb3503ee2b6d8"><tt>e10f36d</tt></a> Define yabai events
- <a href="https://github.com/kkharji/yctrl/commit/8c0105ad7c26ff01614c7c2f2091e967da2a122d"><tt>8c0105a</tt></a> Parse events
- <a href="https://github.com/kkharji/yctrl/commit/14ff105ad5e3d53ec9697acece9141ab7a052a82"><tt>14ff105</tt></a> Rename event predicts to contain _event
- <a href="https://github.com/kkharji/yctrl/commit/e3f2970fa9bb73aa05d1f4383929080b85ff935e"><tt>e3f2970</tt></a> Switch event parser to accept vec<u8> type
- <a href="https://github.com/kkharji/yctrl/commit/e7b000e003d70feea014aa00f613a74c4531c4c3"><tt>e7b000e</tt></a> Add documentation to events
- <a href="https://github.com/kkharji/yctrl/commit/8cd8d02892cf148312d8f730721b9c8d988fce19"><tt>8cd8d02</tt></a> Ignore irrelevant application events
- <a href="https://github.com/kkharji/yctrl/commit/887d77ba7d21fe9458e497356b74d2174e76dbac"><tt>887d77b</tt></a> Reignore buffer overflow error
- <a href="https://github.com/kkharji/yctrl/commit/6419afd7793ab47706df8b68c233e6264332a954"><tt>6419afd</tt></a> Changelog update
- <a href="https://github.com/kkharji/yctrl/commit/e3164cb9afb51e4b53a4b513e48786e6d0d8dec2"><tt>e3164cb</tt></a> Setup window event handlers

### Wip

- <a href="https://github.com/kkharji/yctrl/commit/0adff102ac6418f6332f2bb2657d9883ebe69d84"><tt>0adff10</tt></a> Keep focus on current space on window destroy.

## [0.1.2] - 2022-02-20

### <!-- 0 -->Features

- <a href="https://github.com/kkharji/yctrl/commit/07c9b98d463d12d51f6194d851a3e95f83a2b3aa"><tt>07c9b98</tt></a> Basic impl of dwm make master
- <a href="https://github.com/kkharji/yctrl/commit/6ca518d5a9e27e2ac182a445eecb73f198ee5764"><tt>6ca518d</tt></a> Expose packages.yctrl
- <a href="https://github.com/kkharji/yctrl/commit/25171afa80f3500f6a0e368dfe8c39e7ee9d3528"><tt>25171af</tt></a> Only support darwin
- <a href="https://github.com/kkharji/yctrl/commit/4d31eadb30d0c5c292938d9115fb93a526aa09a1"><tt>4d31ead</tt></a> Build on macos
- <a href="https://github.com/kkharji/yctrl/commit/c5f8d868637b567dd0503a04cd1b67a7680d46f8"><tt>c5f8d86</tt></a> Setup (#1)

### Styling

- <a href="https://github.com/kkharji/yctrl/commit/ff5d48153a435f77d95dae0933fec10d80b0feac"><tt>ff5d481</tt></a> Per cargo clippy suggestions

## [0.1.1] - 2022-02-19

### <!-- 0 -->Features

- <a href="https://github.com/kkharji/yctrl/commit/da875f1dbe276bccd28441c3459edd576674d271"><tt>da875f1</tt></a> Port yabai models
- <a href="https://github.com/kkharji/yctrl/commit/78b8cccd80b2a8aa473c9494c24f660beec0f7af"><tt>78b8ccc</tt></a> Change focus next/prev window
- <a href="https://github.com/kkharji/yctrl/commit/837e955cdef561ee00db85f31864c70746ce7db6"><tt>837e955</tt></a> On available windows even if it's float
- <a href="https://github.com/kkharji/yctrl/commit/d38718654c11be6c05bb774d6fb2906d6afd6109"><tt>d387186</tt></a> Switch to next/prev space
- <a href="https://github.com/kkharji/yctrl/commit/70abd46a92e4abe1be227edb5d358954dc8f6de4"><tt>70abd46</tt></a> Move window to space
- <a href="https://github.com/kkharji/yctrl/commit/a698103860974ed7eb7b8b13d4a3df24a3f92915"><tt>a698103</tt></a> Watch command
- <a href="https://github.com/kkharji/yctrl/commit/f2507f22fa43b89afc4318aeb96a07a2057ed9f3"><tt>f2507f2</tt></a> Increment/decrement window
- <a href="https://github.com/kkharji/yctrl/commit/8ced83b807d4d5c2c1b2bbfbeed2305a07495592"><tt>8ced83b</tt></a> Rework flake-compat + overlay
- <a href="https://github.com/kkharji/yctrl/commit/a0859c4e64dce0878706c96b018ae4b79e026878"><tt>a0859c4</tt></a> Use nix run
- <a href="https://github.com/kkharji/yctrl/commit/3dcd9f1a0cc6df0b6281da95d188c401052b1445"><tt>3dcd9f1</tt></a> Describe package and installation

### <!-- 1 -->Bug Fixes

- <a href="https://github.com/kkharji/yctrl/commit/10d08ea4b74dc45812182d51f2b56ed502cf49ab"><tt>10d08ea</tt></a> Allow two args to be processed
- <a href="https://github.com/kkharji/yctrl/commit/f7132ee1302d914114e5474bc2669f9e0e0af17a"><tt>f7132ee</tt></a> Output bin name
- <a href="https://github.com/kkharji/yctrl/commit/21a3444b700c3451e0669d1aa564e65d34790ac0"><tt>21a3444</tt></a> Inc command

### <!-- 2 -->Refactor

- <a href="https://github.com/kkharji/yctrl/commit/a8e3b8c32b3a8c0a17923ee1c21c83cc7983c21b"><tt>a8e3b8c</tt></a> Simplify and improve readability
- <a href="https://github.com/kkharji/yctrl/commit/00eab4b558bc89bb6f0d04d94e125c23b1309a27"><tt>00eab4b</tt></a> Cleanup + rearrange

### <!-- 3 -->Enhancement

- <a href="https://github.com/kkharji/yctrl/commit/0b39a2d87d72c1fd288081961e9f633c4df101d5"><tt>0b39a2d</tt></a> Function to ignore yabai response

### Miscellaneous Tasks

- <a href="https://github.com/kkharji/yctrl/commit/0de9e12bfedad07d8662925f67fded13a7bad82c"><tt>0de9e12</tt></a> Init
- <a href="https://github.com/kkharji/yctrl/commit/6a2e43da945d025bd1308ef6030905a94e7a8ca8"><tt>6a2e43d</tt></a> Add required deps
- <a href="https://github.com/kkharji/yctrl/commit/e6d123425c59e86e36b4c4bcda090f8c66b34337"><tt>e6d1234</tt></a> Rename to yctrl
- <a href="https://github.com/kkharji/yctrl/commit/67c967e9d36fbbe5db554eef37848347f23abab6"><tt>67c967e</tt></a> License

<!-- generated by git-cliff -->
