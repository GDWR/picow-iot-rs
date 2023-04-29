/// Include the WiFi firmware and Country Locale Matrix (CLM) blobs.
#[macro_export]
macro_rules! include_wifi_firmware {
    () => {
        const WIFI_FIRMWARE_BLOB: &'static[u8] = include_bytes!("../../firmware/43439A0.bin");
        const WIFI_CLM_BLOB: &'static[u8] = include_bytes!("../../firmware/43439A0_clm.bin");
    };
}


#[macro_export]
macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        STATIC_CELL.init_with(move || $val)
    }};
}