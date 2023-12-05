pub mod cs {
    pub mod client;
}

use std::error::Error;

mod structtun;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: more shit also test Tun pls

    Ok(())
}
