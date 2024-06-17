// macro_rules! build_model {
//     ( $struct_name:ident; $new_struct_name:ident; $table:expr => {
//         $( $attr_name:ident : $attr_type:ty ),*
//     }) => {
//         #[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize, PartialEq, Debug, Clone)]
//         #[diesel(check_for_backend(diesel::pg::Pg))]
//         pub struct $struct_name {
//             pub id: i32,
//             $( pub $attr_name : $attr_type ),*
//         }
//
//         #[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
//         #[diesel(table_name = $table)]
//         #[diesel(check_for_backend(diesel::pg::Pg))]
//         pub struct $new_struct_name {
//             $( pub $attr_name : $attr_type ),*
//         }
//     }
// }
//
// pub(crate) use build_model;