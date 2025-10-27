use minoa::postgres::weather::{Entity as WeatherEntity, Model as WeatherModel, ActiveModel as WeatherActiveModel};
use crate::models::weather::{CreateWeatherDto, UpdateWeatherDto};

crate::generate_crud_handlers!(
    WeatherEntity,
    WeatherModel,
    WeatherActiveModel,
    CreateWeatherDto,
    UpdateWeatherDto
);

pub use list as list_weather;
pub use get as get_weather;
pub use create as create_weather;
pub use update as update_weather;
pub use delete as delete_weather;
