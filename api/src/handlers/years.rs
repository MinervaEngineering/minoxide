use minoa::postgres::years::{Entity as YearEntity, Model as YearModel, ActiveModel as YearActiveModel};
use crate::models::years::{CreateYearDto, UpdateYearDto};

crate::generate_crud_handlers!(
    YearEntity,
    YearModel,
    YearActiveModel,
    CreateYearDto,
    UpdateYearDto
);

pub use list as list_years;
pub use get as get_years;
pub use create as create_years;
pub use update as update_years;
pub use delete as delete_years;
