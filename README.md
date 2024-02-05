# Examples
```rust
use core::str::FromStr;
use bras::{Cpf, ParseCpfError};

fn main() -> Result<(), ParseCpfError>{

    let cpf = Cpf::try_from(1678346063)?;
    let cpf = "01678346063".parse::<Cpf>()?;
    let cpf: Cpf = "01678346063".parse()?;
    let cpf = Cpf::from_str("01678346063")?; // Need: use core::str::FromStr;

    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
