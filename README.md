# nekopoi-rs
A NekoPoi direct API, written in Rust, without scraping.

### Run example
```sh
cargo run --example get_detail
```

### Build
```sh
cargo build

# build specific example file
# e.g: cargo build --example <filename_example>
cargo build --example search
```

### Example
You see all examples in the [examples](./examples/) directory.

### FAQ
Q: How do you get their API?<br>
A: Simple, through reverse engineering concepts.<br><br>
Q: How do you do that?<br>
A: Sorry, it's secret 😅.<br><br>
Q: Is the headers affecting my account?<br>
A: So far, no, NekoPoi does not use an account login except for chat using Chatango and the headers is not include for Chatango. Any headers are the responsibility of the NekoPoi developer.

### LICENSE
[MIT License](LICENSE)

### DISCLAIMER
I'm not a NekoPoi developer nor the author, any copyrights goes to NekoPoi themselves, I just creating the API wrapper.
