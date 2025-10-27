use minoa::postgres::projections::{Entity as ProjectionEntity, Model as ProjectionModel, ActiveModel as ProjectionActiveModel};
use crate::models::projections::{CreateProjectionDto, UpdateProjectionDto};

crate::generate_crud_handlers!(
    ProjectionEntity,
    ProjectionModel,
    ProjectionActiveModel,
    CreateProjectionDto,
    UpdateProjectionDto
);

pub use list as list_projections;
pub use get as get_projections;
pub use create as create_projections;
pub use update as update_projections;
pub use delete as delete_projections;
