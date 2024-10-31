use std::path::Path;

use crate::{errors::AppError, spec::proto::v1};

const MAX_FRAME_COUNT: i32 = 2000;

const MIN_MEMORY_MIB: u32 = 1024*4;
const MAX_MEMORY_MIB: u32 = 1024*64;

// Validate a create job request
pub fn can_create_job(req: &v1::CreateJobRequest) -> Result<(), AppError> {
    let blend_path = Path::new(&req.file_path);
    let blend_exists = blend_path.exists();
    let blend_is_file = blend_path.is_file();
    if !blend_exists {
        return Err(AppError::BadRequest(
            "Specified blend file does not exist.".to_string(),
        ));
    } else if !blend_is_file {
        return Err(AppError::BadRequest(
            "Specified blend file is a directory.".to_string(),
        ));
    }

    if req.ocio_config_path.is_some() {
        let ocio_exists = Path::new(&req.ocio_config_path.clone().unwrap()).exists();
        if !ocio_exists {
            return Err(AppError::BadRequest(
                "Specified OCIO config file does not exist.".to_string(),
            ));
        }
    }

    if req.frame_count < 0 || req.frame_count > MAX_FRAME_COUNT {
        return Err(AppError::BadRequest("Invalid frame count.".to_string()));
    }

    let memory_mib = req.memory_mib.unwrap_or(1024*4);
    if memory_mib < MIN_MEMORY_MIB || memory_mib > MAX_MEMORY_MIB {
        return Err(AppError::BadRequest("Invalid memory_mib.".to_string()));
    }

    Ok(())
}
