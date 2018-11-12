
use crate::core_foundation_sys::url::CFURLRef;
use crate::core_foundation_sys::string::CFStringRef;
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::base::{ OSStatus, CFTypeID, CFTypeRef, CFAllocatorRef };
use crate::coremedia_sys::CMTimeRange;


pub type VTMultiPassStorageRef = CFTypeRef;


#[link(name="VideoToolBox", kind="framework")]
extern {
    pub static kVTMultiPassStorageCreationOption_DoNotDelete: CFStringRef;


    pub fn VTMultiPassStorageCreate(allocator: CFAllocatorRef,
                                    fileURL: CFURLRef,
                                    timeRange: CMTimeRange,
                                    options: CFDictionaryRef,
                                    multiPassStorageOut: *mut VTMultiPassStorageRef) -> OSStatus;
    pub fn VTMultiPassStorageClose(multiPassStorage: VTMultiPassStorageRef) -> OSStatus;
    pub fn VTMultiPassStorageGetTypeID() -> CFTypeID;

}