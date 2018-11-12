use crate::core_foundation_sys::base::CFTypeRef;
use crate::core_foundation_sys::base::OSStatus;
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::corevideo_sys::CVPixelBufferRef;


pub type CGImageRef = CFTypeRef;


#[link(name="VideoToolBox", kind="framework")]
extern {
    pub fn VTCreateCGImageFromCVPixelBuffer(pixelBuffer: CVPixelBufferRef,
                                            options: CFDictionaryRef,
                                            imageOut: *mut CGImageRef) -> OSStatus;

}