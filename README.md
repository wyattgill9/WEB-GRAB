# WEB-GRAB Challenge (Rust)

Welcome to the **WEB-GRAB Challenge!** The goal is simple: create the **Fastest** web crawler in Rust.

## Quickstart:

1. **Clone the repo** and add your code in `src/your-github-username.rs`, and add it to `lib.rs`
2. Your file must contain a function:
   ```rust
   pub fn Crawler() -> Result<(), Box<dyn Error>> {
       let html = ""; // Your code to fetch the HTML of "example.com"
       html
   }
   ```
3. The function should return the HTML content of `"https://example.com"`.
4. Feel free to use **any optimizations, libraries, techniques or tools** to make your crawler the fastest. (like Nightly or any Compiler Optimizations)
5. **It will be tested on a Server running 32 GB DDR5 RAM and 24 Core Ryzen 9 9900x**

## Rules:

1. The crawler must make an actual HTTP request, no hardcoded responses.
2. **NO OTHER RULES!**

## How to Participate

1. Fork the repository.
2. Implement your crawler.
3. Submit a pull request with your code and your benchmarks!


