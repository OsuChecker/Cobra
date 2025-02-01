use crate::ui::call;

pub mod reading_loop;

pub mod consts;
pub mod gosu_structs;
pub mod structs;

pub mod cobra_struct;
pub mod global;
pub mod reader;
mod ui;
pub mod utils;

mod reader2;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(async {
        crate::reader::controlla().await;
    });
    call();

    Ok(())
}
