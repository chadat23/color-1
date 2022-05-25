pub fn rgb_to_ycbcr(r: u32, g: u32, b: u32) -> (u32, u32, u32) {
    // let y = 0.299 * r + 0.587 * g + 0.114 * b;
    // let cb = 128.0 - 0.1687 * r - 0.3313 * g + 0.5 * b;
    // let cr = 128.0 + 0.5 * r - 0.4187 * g - 0.0813 * b;

    // let y = (16.0 + 65.481 * (r as f64) + 128.553 * (g as f64) + 24.966 * (b as f64)) as u32;
    // let cb = (128.0 - 37.797 * (r as f64) - 74.203 * (g as f64)+ 112.0 * (b as f64)) as u32;
    // let cr = (128.0 + 112.0 * (r as f64) - 93.786 * (g as f64)- 18.214 * (b as f64)) as u32;

    // https://sistenix.com/rgb2ycbcr.html
    // let y = 16 + (((r <<6 )+(r << 1)+(g << 7) + g + (b << 4) + (b << 3) + b) >> 8);
    // let cb = 128 + ((-(((r << 5) + (r << 2) + (r << 1)) as i32) - ((g << 6) + (g << 3) + (g << 1)) as i32 + (b << 7) as i32 - (b << 4) as i32) >> 8);
    // let cr = 128 + (((r << 7) - (r << 4) - ((g << 6) + (g << 5) - (g << 1)) - ((b << 4) + (b << 1))) >> 8);

    // https://web.archive.org/web/20180421030430/http://www.equasys.de/colorconversion.html
    let r = r as f64;
    let g = g as f64;
    let b = b as f64;
    let y = (0.0 + 0.299 * r + 0.587 * g + 0.114 * b) as u32;
    let cb = (128.0 - 0.169 * r - 0.331 * g + 0.500 * b) as u32;
    let cr = (128.0 + 0.500 * r - 0.419 * g - 0.081 * b) as u32;

    (y, cb, cr)
}

pub fn ycbcr_to_rgb(y: u32, cb: u32, cr: u32) -> (u32, u32, u32) {
    // https://stackoverflow.com/questions/4041840/function-to-convert-ycbcr-to-rgb
    // let r = (y as f64 + 1.40200 * (cr - 0x80) as f64) as u32;
    // let g = (y as f64 - 0.34414 * (cb - 0x80) as f64 - 0.71414 * (cr - 0x80) as f64) as u32;
    // let b = (y as f64 + 1.77200 * (cb - 0x80) as f64) as u32;

    // https://web.archive.org/web/20180421030430/http://www.equasys.de/colorconversion.html
    let y = y as f64;
    let cb = cb as f64;
    let cr = cr as f64;
    let r = (1.0 * y + 0.000 * (cb - 128.) + 1.400 * (cr - 128.)) as u32;
    let g = (1.0 * y - 0.343 * (cb - 128.) - 0.711 * (cr - 128.)) as u32;
    let b = (1.0 * y + 1.765 * (cb - 128.) + 0.000 * (cr - 128.)) as u32;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
