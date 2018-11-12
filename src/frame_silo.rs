use crate::libc::{c_void, c_float};
use crate::core_foundation_sys::url::CFURLRef;
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::base::{ OSStatus, CFTypeID, CFTypeRef, CFAllocatorRef };
use crate::coremedia_sys::{ CMItemCount, CMTimeRange, CMSampleBufferRef };


pub type VTFrameSiloRef = CFTypeRef;
pub type Float32 = c_float;


#[link(name="VideoToolBox", kind="framework")]
extern {

    pub fn VTFrameSiloCreate(allocator: CFAllocatorRef,
                                    fileURL: CFURLRef,
                                    timeRange: CMTimeRange,
                                    options: CFDictionaryRef,
                                    multiPassStorageOut: *mut VTFrameSiloRef) -> OSStatus;
    pub fn VTFrameSiloAddSampleBuffer(silo: VTFrameSiloRef, sampleBuffer: CMSampleBufferRef) -> OSStatus;
    pub fn VTFrameSiloSetTimeRangesForNextPass(silo: VTFrameSiloRef,
                                               timeRangeCount: CMItemCount,
                                               timeRangeArray: *const CMTimeRange) -> OSStatus;

    pub fn VTFrameSiloCallBlockForEachSampleBuffer(silo: VTFrameSiloRef,
                                                   timeRange: CMTimeRange,
                                                   handler: extern "C" fn(sampleBuffer: CMSampleBufferRef) -> OSStatus ) -> OSStatus;
    pub fn VTFrameSiloCallFunctionForEachSampleBuffer(silo: VTFrameSiloRef,
                                                      timeRange: CMTimeRange,
                                                      refcon: *mut c_void,
                                                      callback: extern "C" fn (refcon: *mut c_void,
                                                                               sampleBuffer: CMSampleBufferRef) -> OSStatus
                                                      ) -> OSStatus;

    pub fn VTFrameSiloGetProgressOfCurrentPass(silo: VTFrameSiloRef,
                                               progressOut: *mut Float32) -> OSStatus;
    pub fn VTFrameSiloGetTypeID() -> CFTypeID;

}