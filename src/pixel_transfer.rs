
use crate::core_foundation_sys::base::{ OSStatus, CFTypeID, CFTypeRef, CFAllocatorRef };
use crate::corevideo_sys::CVPixelBufferRef;


pub type VTPixelTransferSessionRef = CFTypeRef;


#[link(name="VideoToolBox", kind="framework")]
extern {
    pub fn VTPixelTransferSessionCreate(allocator: CFAllocatorRef,
                                        pixelTransferSessionOut: VTPixelTransferSessionRef) -> OSStatus;
    pub fn VTPixelTransferSessionTransferImage(session: VTPixelTransferSessionRef,
                                               sourceBuffer: CVPixelBufferRef,
                                               destinationBuffer: CVPixelBufferRef) -> OSStatus;
    pub fn VTPixelTransferSessionGetTypeID() -> CFTypeID;
    pub fn VTPixelTransferSessionInvalidate(session: VTPixelTransferSessionRef) -> ();

}