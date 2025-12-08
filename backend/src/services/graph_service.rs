use crate::database::GraphRepository;
use crate::error::{AppError, AppResult};

pub struct GraphService {
    graph_repository: GraphRepository,
}

impl GraphService {
    pub fn new(graph_repository: GraphRepository) -> Self {
        Self { graph_repository }
    }

    // TODO: Implement graph service methods
}