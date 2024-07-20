# Log Scheduler Demo

This repository has been created for education purposed-only.

### Introduction

My goal with this project is to take hands-on asynchronous programming using Rust programming language. To precise, I have been already using async programming in Rust, however I have only been using it on the surface and with the specific `tokio` runtime.

Thanks to this project, I have been able to dive more on the creation of `Future`s as well as `Stream`s which are the core of asynchronous programming in Rust.

### Avoiding tokio on-purpose

`tokio` ois the most-used asynchronous runtime for Rust. It simplifies a lot of
async programming providing a lot of modules making way more than a simple runtime. [Here is an interesting article on the subject](https://corrode.dev/blog/async/)

To make sure I am not depending `tokio`, I used another async runtime named `smol`.

### How to run ?

Using very standard commands:

```shell
git clone https://github.com/pr0m3th3usEx/log-scheduler-rs.git
cd log-scheduler-rs
```
```shell
cargo run -- <delay in second>
# Exemple cargo run -- 3
```

### Some sources used for this project

- [Rust Book about Async Programming in Rust](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)
- [Functions & Closures](https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html)
- [Accept async function as argument](https://stackoverflow.com/questions/60717746/how-to-accept-an-async-function-as-an-argument)
