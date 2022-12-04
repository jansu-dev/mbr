
define_error_codes!(
    "MBR:Cmd:",

    INCOMPATIBLE => ("Imcompatible", "", ""),
    UNKNOWN => ("Unknown", "", "")
);


// quick_error! {
//     #[derive(Debug)]
//     pub enum Error {
//         Incompatible {
//             description("compatible error")
//             display("feature is not supported in other cluster components")
//         }
//     }
// }
