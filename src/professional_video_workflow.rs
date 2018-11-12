

#[link(name="VideoToolBox", kind="framework")]
extern {
    pub fn VTRegisterProfessionalVideoWorkflowVideoDecoders() -> ();
    pub fn VTRegisterProfessionalVideoWorkflowVideoEncoders() -> ();
}
