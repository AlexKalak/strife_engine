use strife::{error_client, info_client};

fn main() {
    match strife::sf_log::init() {
        Some(..) => info_client!("LOGGER WAS INIT"),
        None => {
            error_client!("ERROR INITIALIZING LOGGER")
        }
    };

    strife::entry_point();
}
