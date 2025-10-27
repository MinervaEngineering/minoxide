use minoa::postgres::contest_list_parameters::{Entity as ContestListParameterEntity, Model as ContestListParameterModel, ActiveModel as ContestListParameterActiveModel};
use crate::models::contest_list_parameters::{CreateContestListParameterDto, UpdateContestListParameterDto};

crate::generate_crud_handlers!(
    ContestListParameterEntity,
    ContestListParameterModel,
    ContestListParameterActiveModel,
    CreateContestListParameterDto,
    UpdateContestListParameterDto
);

pub use list as list_contest_list_parameters;
pub use get as get_contestlistparameters;
pub use create as create_contestlistparameters;
pub use update as update_contestlistparameters;
pub use delete as delete_contestlistparameters;
