use std::ffi::{CString, c_void, c_char};

#[link(name = "meme_processor")]
extern "C" {
    fn meme_processor_new() -> *mut c_void;
    fn meme_processor_delete(processor: *mut c_void);
    fn meme_processor_create_classic(
        processor: *mut c_void,
        input_path: *const c_char,
        top_text: *const c_char,
        bottom_text: *const c_char,
        output_path: *const c_char,
    ) -> bool;
}

pub struct MemeProcessor {
    processor: *mut c_void,
}

impl MemeProcessor {
    pub fn new() -> Self {
        unsafe {
            MemeProcessor {
                processor: meme_processor_new(),
            }
        }
    }
    
    pub fn create_classic_meme(
        &self,
        input_path: &str,
        top_text: &str,
        bottom_text: &str,
        output_path: &str,
    ) -> Result<(), String> {
        let input_path_c = CString::new(input_path).map_err(|e| e.to_string())?;
        let top_text_c = CString::new(top_text).map_err(|e| e.to_string())?;
        let bottom_text_c = CString::new(bottom_text).map_err(|e| e.to_string())?;
        let output_path_c = CString::new(output_path).map_err(|e| e.to_string())?;
        
        unsafe {
            let success = meme_processor_create_classic(
                self.processor,
                input_path_c.as_ptr(),
                top_text_c.as_ptr(),
                bottom_text_c.as_ptr(),
                output_path_c.as_ptr(),
            );
            
            if success {
                Ok(())
            } else {
                Err("Failed to create meme".to_string())
            }
        }
    }
}

impl Drop for MemeProcessor {
    fn drop(&mut self) {
        unsafe {
            meme_processor_delete(self.processor);
        }
    }
}

unsafe impl Send for MemeProcessor {}
unsafe impl Sync for MemeProcessor {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_meme_processor_creation() {
        let processor = MemeProcessor::new();
        // Processor should be created successfully
        assert!(!processor.processor.is_null());
    }
}
