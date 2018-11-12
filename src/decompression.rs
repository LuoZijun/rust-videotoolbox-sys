use crate::libc::{c_void};
use crate::core_foundation_sys::base::{ OSStatus, CFTypeID, CFTypeRef, CFAllocatorRef, Boolean };
use crate::core_foundation_sys::string::CFStringRef;
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::corevideo_sys::{ CVImageBufferRef, CVPixelBufferRef, };
use crate::coremedia_sys::{ CMTime, CMVideoFormatDescriptionRef, CMSampleBufferRef, CMFormatDescriptionRef };



pub type VTDecodeInfoFlags = u32;
pub type VTDecodeFrameFlags = u32;




// #[allow(missing_copy_implementations)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompressionOutputCallback: VTDecompressionOutputCallback,
    pub decompressionOutputRefCon: *mut c_void,
}


pub type VTDecompressionSessionRef = CFTypeRef;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpaqueVTDecompressionSession;
// pub type VTDecompressionSessionRef = *mut OpaqueVTDecompressionSession;

pub type VTDecompressionOutputCallback = extern "C" fn(decompressionOutputRefCon: *mut c_void,
                                                       sourceFrameRefCon: *mut c_void,
                                                       status: OSStatus,
                                                       infoFlags: VTDecodeInfoFlags,
                                                       imageBuffer: CVImageBufferRef,
                                                       presentationTimeStamp: CMTime,
                                                       presentationDuration: CMTime);
pub type VTDecompressionOutputHandler = extern "C" fn(status: OSStatus,
                                                      infoFlags: VTDecodeInfoFlags,
                                                      imageBuffer: CVImageBufferRef,
                                                      presentationTimeStamp: CMTime,
                                                      presentationDuration: CMTime);


#[link(name="VideoToolBox", kind="framework")]
extern {
    pub static kVTDecompressionPropertyKey_PixelBufferPool: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelBufferPoolIsShared: CFStringRef;
    pub static kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount: CFStringRef;
    pub static kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded: CFStringRef;
    pub static kVTDecompressionPropertyKey_ContentHasInterframeDependencies: CFStringRef;
    pub static kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder: CFStringRef;
    pub static kVTDecompressionPropertyKey_RealTime: CFStringRef;
    pub static kVTDecompressionPropertyKey_ThreadCount: CFStringRef;
    pub static kVTDecompressionPropertyKey_FieldMode: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_BothFields: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_TopFieldOnly: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_BottomFieldOnly: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_SingleField: CFStringRef;
    pub static kVTDecompressionProperty_FieldMode_DeinterlaceFields: CFStringRef;
    pub static kVTDecompressionPropertyKey_DeinterlaceMode: CFStringRef;
    pub static kVTDecompressionProperty_DeinterlaceMode_VerticalFilter: CFStringRef;
    pub static kVTDecompressionProperty_DeinterlaceMode_Temporal: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedResolutionDecode: CFStringRef;
    pub static kVTDecompressionResolutionKey_Width: CFStringRef;
    pub static kVTDecompressionResolutionKey_Height: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedCoefficientDecode: CFStringRef;
    pub static kVTDecompressionPropertyKey_ReducedFrameDelivery: CFStringRef;
    pub static kVTDecompressionPropertyKey_OnlyTheseFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_AllFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_NonDroppableFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_IFrames: CFStringRef;
    pub static kVTDecompressionProperty_OnlyTheseFrames_KeyFrames: CFStringRef;
    pub static kVTDecompressionProperty_TemporalLevelLimit: CFStringRef;
    pub static kVTDecompressionPropertyKey_SuggestedQualityOfServiceTiers: CFStringRef;
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality: CFStringRef;
    pub static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport: CFStringRef;
    pub static kVTDecompressionPropertyKey_PixelTransferProperties: CFStringRef;
    pub static kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID: CFStringRef;
    pub static kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID: CFStringRef;


    // Creating Sessions
    pub fn VTDecompressionSessionCreate(allocator: CFAllocatorRef,
                                        videoFormatDescription: CMVideoFormatDescriptionRef,
                                        videoDecoderSpecification: CFDictionaryRef,
                                        destinationImageBufferAttributes: CFDictionaryRef,
                                        outputCallback: *const VTDecompressionOutputCallbackRecord,
                                        decompressionSessionOut: *mut VTDecompressionSessionRef) -> OSStatus;

    // Decoding Sample Buffers
    pub fn VTDecompressionSessionDecodeFrame(session: VTDecompressionSessionRef,
                                             sampleBuffer: CMSampleBufferRef,
                                             decodeFlags: VTDecodeFrameFlags,
                                             sourceFrameRefCon: *mut c_void,
                                             infoFlagsOut: *mut VTDecodeInfoFlags) -> OSStatus;
    pub fn VTDecompressionSessionDecodeFrameWithOutputHandler(session: VTDecompressionSessionRef,
                                                              sampleBuffer: CMSampleBufferRef,
                                                              decodeFlags: VTDecodeFrameFlags,
                                                              infoFlagsOut: *mut VTDecodeInfoFlags,
                                                              outputHandler: VTDecompressionOutputHandler) -> OSStatus;
    pub fn VTDecompressionSessionFinishDelayedFrames(session: VTDecompressionSessionRef) -> OSStatus;
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(session: VTDecompressionSessionRef) -> OSStatus;
    pub fn VTDecompressionSessionCopyBlackPixelBuffer(session: VTDecompressionSessionRef,
                                                     pixelBufferOut: *mut CVPixelBufferRef) -> OSStatus;

    // Inspecting Sessions
    pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
    pub fn VTDecompressionSessionCanAcceptFormatDescription(session: VTDecompressionSessionRef,
                                                            newFormatDesc: CMFormatDescriptionRef) -> Boolean;
}