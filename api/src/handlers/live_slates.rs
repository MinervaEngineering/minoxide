use minoa::postgres::live_slates::{Entity as LiveSlateEntity, Model as LiveSlateModel, ActiveModel as LiveSlateActiveModel};
use crate::models::live_slates::{CreateLiveSlateDto, UpdateLiveSlateDto};

crate::generate_crud_handlers!(
    LiveSlateEntity,
    LiveSlateModel,
    LiveSlateActiveModel,
    CreateLiveSlateDto,
    UpdateLiveSlateDto
);

pub use list as list_live_slates;
pub use get as get_liveslates;
pub use create as create_liveslates;
pub use update as update_liveslates;
pub use delete as delete_liveslates;
