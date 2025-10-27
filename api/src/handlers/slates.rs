use minoa::postgres::slates::{Entity as SlateEntity, Model as SlateModel, ActiveModel as SlateActiveModel};
use crate::models::slates::{CreateSlateDto, UpdateSlateDto};

crate::generate_crud_handlers!(
    SlateEntity,
    SlateModel,
    SlateActiveModel,
    CreateSlateDto,
    UpdateSlateDto
);

pub use list as list_slates;
pub use get as get_slates;
pub use create as create_slates;
pub use update as update_slates;
pub use delete as delete_slates;
