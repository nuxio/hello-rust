//! # Hello Rust
//! 
//! `hello_rust` 是一个学习项目
//! 
//! 这种以 `//!` 开头的注释 叫做 包含注释
//! 通常放在 crate 跟文件 src/lib.rs 的头部，为模块整体提供文档
//! 注释模块所包含的内容

/**
 * Notes
 *
 * 在 Rust 中 发布配置 release profiles 有两个主要的配置
 * 1. 运行 cargo build 时采用的 dev 配置
 * 2. 运行 cargo build --release 的 release 配置
 *
 * 可以在 Cargo.toml 文件中对每个配置进行更改
 *
 * 更改 dev 的默认配置
 * [profile.dev]
 * opt-level = 1 // 优化等级，dev 配置默认是 0，release 配置默认是 3
 *
 * 更改 release 的默认配置
 * [profile.release]
 * opt-level = 2
 *
 * */

// 三斜杠开头的 称为文档注释，支持以 Markdown 格式书写文档
// 使用 cargo doc 生成文档，使用 cargo doc --open 可以直接打开浏览器
// 文档测试中的示例代码块也会被 cargo test 运行

/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

fn main() {}
