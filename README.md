# pwgen

*pwgen* (as the name suggests) is a very simple cli tool written in [Rust](https://www.rust-lang.org/) to generate passwords.
It lets you generate any number of passwords with any length (only limit are the language limitation/data types) and you can choose whether your password(s) should include:
- Lowercase letters (a-z)
- Uppercase letters (A-Z)
- Numbers (0-9)
- Symbols (e.g., @#$%)


### Note:
This project was just a fun thing I did while learning Rust.
I believe it is a great beginner project to get used to a language.
Therefore, there are quite possibly things in the code which can be done better.
Because I wanted to try to work with an external package which is a bit more sophisticated than the rand package,
I used dialoguer ([github](https://github.com/mitsuhiko/dialoguer), [crates.io](https://crates.io/crates/dialoguer)) to create the amazing command line prompts.

