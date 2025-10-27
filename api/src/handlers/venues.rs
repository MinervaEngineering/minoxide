use minoa::postgres::venues::{Entity as VenueEntity, Model as VenueModel, ActiveModel as VenueActiveModel};
use crate::models::venues::{CreateVenueDto, UpdateVenueDto};

crate::generate_crud_handlers!(
    VenueEntity,
    VenueModel,
    VenueActiveModel,
    CreateVenueDto,
    UpdateVenueDto
);

pub use list as list_venues;
pub use get as get_venues;
pub use create as create_venues;
pub use update as update_venues;
pub use delete as delete_venues;
