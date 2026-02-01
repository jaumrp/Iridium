use std::io::Error;

use iridium::protocol;

#[iridium::main]
async fn main() -> Result<(), Error> {
    protocol::test();

    Ok(())
}
