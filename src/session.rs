use crate::libc::{c_void};
use crate::core_foundation_sys::base::{ OSStatus, CFTypeRef, CFAllocatorRef };
use crate::core_foundation_sys::string::CFStringRef;
use crate::core_foundation_sys::dictionary::CFDictionaryRef;


pub type VTSessionRef = CFTypeRef;


#[link(name="VideoToolBox", kind="framework")]
extern {
    pub static kVTPropertyTypeKey: CFStringRef;
    pub static kVTPropertyType_Enumeration: CFStringRef;
    pub static kVTPropertyType_Boolean: CFStringRef;
    pub static kVTPropertyType_Number: CFStringRef;
    pub static kVTPropertyReadWriteStatusKey: CFStringRef;
    pub static kVTPropertyReadWriteStatus_ReadOnly: CFStringRef;
    pub static kVTPropertyReadWriteStatus_ReadWrite: CFStringRef;
    pub static kVTPropertyShouldBeSerializedKey: CFStringRef;
    pub static kVTPropertySupportedValueMinimumKey: CFStringRef;
    pub static kVTPropertySupportedValueMaximumKey: CFStringRef;
    pub static kVTPropertySupportedValueListKey: CFStringRef;
    pub static kVTPropertyDocumentationKey: CFStringRef;


    pub fn VTSessionSetProperty(session: VTSessionRef,
                                propertyKey: CFStringRef,
                                propertyValue: CFTypeRef) -> OSStatus;
    pub fn VTSessionSetProperties(session: VTSessionRef,
                                  propertyDictionary: CFDictionaryRef) -> OSStatus;
    pub fn VTSessionCopyProperty(session: VTSessionRef,
                                 propertyKey: CFStringRef,
                                 allocator: CFAllocatorRef,
                                 propertyValueOut: *mut c_void) -> OSStatus;
    pub fn VTSessionCopySerializableProperties(session: VTSessionRef,
                                               allocator: CFAllocatorRef,
                                               dictionaryOut: *mut CFDictionaryRef) -> OSStatus;
    pub fn VTSessionCopySupportedPropertyDictionary(session: VTSessionRef,
                                                    supportedPropertyDictionaryOut: *mut CFDictionaryRef) -> OSStatus;

}