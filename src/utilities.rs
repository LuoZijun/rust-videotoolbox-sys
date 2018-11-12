use crate::libc::int32_t;
use crate::core_foundation_sys::array::CFArrayRef;
use crate::core_foundation_sys::string::CFStringRef;
use crate::core_foundation_sys::base::{ CFTypeRef, OSStatus };
use crate::core_foundation_sys::dictionary::CFDictionaryRef;

use crate::corevideo_sys::CVPixelBufferRef;
use crate::coremedia_sys::CMVideoCodecType;


pub type CGImageRef = CFTypeRef;


#[link(name="VideoToolBox", kind="framework")]
extern {

    pub fn VTRegisterProfessionalVideoWorkflowVideoDecoders() -> ();
    pub fn VTRegisterProfessionalVideoWorkflowVideoEncoders() -> ();

    pub fn VTCreateCGImageFromCVPixelBuffer(pixelBuffer: CVPixelBufferRef,
                                            options: CFDictionaryRef,
                                            imageOut: *mut CGImageRef) -> OSStatus;


    pub static kVTVideoEncoderList_CodecType: CFStringRef;
    pub static kVTVideoEncoderList_EncoderID: CFStringRef;

    pub static kVTVideoEncoderList_CodecName: CFStringRef;
    pub static kVTVideoEncoderList_EncoderName: CFStringRef;
    pub static kVTVideoEncoderList_DisplayName: CFStringRef;

    pub fn VTCopyVideoEncoderList(options: CFDictionaryRef,
                                  listOfVideoEncodersOut: *mut CFArrayRef) -> OSStatus;
    pub fn VTCopySupportedPropertyDictionaryForEncoder(width: int32_t,
                                                       height: int32_t,
                                                       codecType: CMVideoCodecType,
                                                       encoderSpecification: CFDictionaryRef,
                                                       outEncoderID: *mut CFStringRef,
                                                       outSupportedProperties: *mut CFDictionaryRef) -> OSStatus;

}