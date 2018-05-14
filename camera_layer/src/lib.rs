extern crate image;
extern crate mozjpeg_sys;
extern crate gphoto2_sys as gphoto;
extern crate libc;


use std::mem;
use std::slice;


use gphoto::{Camera, GPContext, CameraFile};



/// Create the gphoto2 camera and context objects that will be necessary for most actions
/// Simple wrapper around gphoto calls specialized for the photobooth functionality
// TODO: Error handling
pub fn initialize() -> (*mut Camera, *mut GPContext){
    let mut camera = unsafe { mem::uninitialized() };
    let mut context = unsafe {gphoto::gp_context_new()};

    unsafe{
        gphoto::gp_camera_new(&mut camera);
        gphoto::gp_camera_init(camera, context);
    }

    (camera, context)
}

/// super simple wrapper for creating a CameraFile in memory
pub fn new_camera_file() -> *mut CameraFile {
    let mut camera_file = unsafe { mem::uninitialized() };
    unsafe {gphoto::gp_file_new(&mut camera_file)};
    camera_file
}

/// Fetches a single preview image using the given CameraFile
/// Returns an image with format Rgba for easy use
pub fn get_preview_image(camera: *mut Camera, context: *mut GPContext, camera_file: *mut CameraFile) -> image::RgbaImage {
    unsafe {
        gphoto::gp_camera_capture_preview(camera, camera_file, context)
    };

    let data = camera_file_to_slice(camera_file);

    decode_jpeg_slice(data)
}


/// Extract the data from a CameraFile and return it in a slice.
/// Might break on other platforms than Linux because of platform types
pub fn camera_file_to_slice<'a>(camera_file: *mut CameraFile) -> &'a[u8]{
    unsafe {
        // c_ulong should be u64
        let mut size: u64 = mem::uninitialized();
        // c_char should be i8
        let mut data: *const i8 = mem::uninitialized();

        gphoto::gp_file_get_data_and_size(camera_file, &mut data, &mut size);

        slice::from_raw_parts(data as *const u8, size as usize)
    }
}

/// Basically the example code from the mozjpeg-sys page on github
pub fn decode_jpeg_slice(data: &[u8]) -> image::RgbaImage {
    use mozjpeg_sys::{jpeg_error_mgr, jpeg_decompress_struct, jpeg_std_error};

    unsafe {
        let mut err: jpeg_error_mgr = mem::zeroed();
        let mut cinfo: jpeg_decompress_struct = mem::zeroed();
        cinfo.common.err = jpeg_std_error(&mut err);
        mozjpeg_sys::jpeg_create_decompress(&mut cinfo);

        let data_len = data.len();

        mozjpeg_sys::jpeg_mem_src(&mut cinfo, data.as_ptr(), data_len as std::os::raw::c_ulong);
        mozjpeg_sys::jpeg_read_header(&mut cinfo, true as mozjpeg_sys::boolean);

        let width = cinfo.image_width;
        let height = cinfo.image_height;

        cinfo.out_color_space = mozjpeg_sys::J_COLOR_SPACE::JCS_EXT_RGBA;

        mozjpeg_sys::jpeg_start_decompress(&mut cinfo);

        let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
        let buffer_size = row_stride * cinfo.image_height as usize;

        let mut buffer = vec![0u8; buffer_size];

        while cinfo.output_scanline < cinfo.output_height {
            let offset = cinfo.output_scanline as usize * row_stride;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            mozjpeg_sys::jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
        }

        mozjpeg_sys::jpeg_finish_decompress(&mut cinfo);
        mozjpeg_sys::jpeg_destroy_decompress(&mut cinfo);

        let image_buffer = image::ImageBuffer::from_raw(width, height, buffer).unwrap();

        image_buffer
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
