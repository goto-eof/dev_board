// use crate::structure::structure::DevBoardGenericError;
// use crate::structure::structure::Response;
// use serde::Serialize;
// use warp::{reply::json, Reply};

// pub fn generate_response<T: Serialize>(
//     data: Result<T, DevBoardGenericError>,
// ) -> crate::GenericResult<impl Reply> {
//     match data {
//         Ok(result) => Ok(json::<_>(&Response {
//             success: true,
//             result: &result,
//         })),
//         Err(err) => Ok(json::<_>(&err)),
//     }
// }
