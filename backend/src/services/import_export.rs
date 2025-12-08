use crate::database::CIRepository;
use crate::error::{AppError, AppResult};

pub struct ImportExportService {
    ci_repository: CIRepository,
}

impl ImportExportService {
    pub fn new(ci_repository: CIRepository) -> Self {
        Self { ci_repository }
    }

    // TODO: Implement import/export service methods
}