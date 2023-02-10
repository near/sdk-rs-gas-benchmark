use near_sdk::{env};

#[no_mangle]
pub unsafe fn lowlevel_storage_write() {
    let data: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    env::storage_write(&data, &data);
}