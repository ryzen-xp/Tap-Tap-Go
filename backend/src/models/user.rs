use sqlx::types::chrono;
use uuid::Uuid;


#[derive( Debug, Clone , sqlx::FromRow)]
pub struct UserModel {
     pub id : Uuid , 
     pub username : String  , 
     pub device_sign : String , 
     pub created_at : chrono::NaiveDateTime
}