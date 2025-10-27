use minoa::postgres::dates::{Entity as DateEntity, Model as DateModel, ActiveModel as DateActiveModel};
use crate::models::dates::{CreateDateDto, UpdateDateDto};

crate::generate_crud_handlers!(
    DateEntity,
    DateModel,
    DateActiveModel,
    CreateDateDto,
    UpdateDateDto
);

pub use list as list_dates;
pub use get as get_dates;
pub use create as create_dates;
pub use update as update_dates;
pub use delete as delete_dates;
