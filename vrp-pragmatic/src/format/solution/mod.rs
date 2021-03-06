//! Specifies logic to create a "pragmatic" solution and write it into json format.

mod model;
pub use self::model::*;

pub(crate) mod activity_matcher;

mod geo_serializer;
pub use self::geo_serializer::serialize_solution_as_geojson;

mod initial_reader;
pub use self::initial_reader::read_init_solution;

mod extensions;

mod writer;
pub use self::writer::create_solution;
pub use self::writer::PragmaticSolution;

use super::*;

fn map_code_reason(code: i32) -> (&'static str, &'static str) {
    match code {
        SKILL_CONSTRAINT_CODE => ("SKILL_CONSTRAINT", "cannot serve required skill"),
        TIME_CONSTRAINT_CODE => ("TIME_WINDOW_CONSTRAINT", "cannot be visited within time window"),
        CAPACITY_CONSTRAINT_CODE => ("CAPACITY_CONSTRAINT", "does not fit into any vehicle due to capacity"),
        REACHABLE_CONSTRAINT_CODE => ("REACHABLE_CONSTRAINT", "location unreachable"),
        DISTANCE_LIMIT_CONSTRAINT_CODE => {
            ("MAX_DISTANCE_CONSTRAINT", "cannot be assigned due to max distance constraint of vehicle")
        }
        DURATION_LIMIT_CONSTRAINT_CODE => {
            ("SHIFT_TIME_CONSTRAINT", "cannot be assigned due to shift time constraint of vehicle")
        }
        BREAK_CONSTRAINT_CODE => ("BREAK_CONSTRAINT", "break is not assignable"),
        LOCKING_CONSTRAINT_CODE => ("LOCKING_CONSTRAINT", "cannot be served due to relation lock"),
        PRIORITY_CONSTRAINT_CODE => ("PRIORITY_CONSTRAINT", "cannot be served due to priority"),
        AREA_CONSTRAINT_CODE => ("AREA_CONSTRAINT", "cannot be assigned due to area constraint"),
        DISPATCH_CONSTRAINT_CODE => ("DISPATCH_CONSTRAINT", "cannot be assigned due to vehicle dispatch"),
        TOUR_SIZE_CONSTRAINT_CODE => {
            ("TOUR_SIZE_CONSTRAINT", "cannot be assigned due to tour size constraint of vehicle")
        }
        _ => ("NO_REASON_FOUND", "unknown"),
    }
}

fn map_reason_code(reason: &str) -> i32 {
    match reason {
        "SKILL_CONSTRAINT" => SKILL_CONSTRAINT_CODE,
        "TIME_WINDOW_CONSTRAINT" => TIME_CONSTRAINT_CODE,
        "CAPACITY_CONSTRAINT" => CAPACITY_CONSTRAINT_CODE,
        "REACHABLE_CONSTRAINT" => REACHABLE_CONSTRAINT_CODE,
        "MAX_DISTANCE_CONSTRAINT" => DISTANCE_LIMIT_CONSTRAINT_CODE,
        "SHIFT_TIME_CONSTRAINT" => DURATION_LIMIT_CONSTRAINT_CODE,
        "BREAK_CONSTRAINT" => BREAK_CONSTRAINT_CODE,
        "LOCKING_CONSTRAINT" => LOCKING_CONSTRAINT_CODE,
        "PRIORITY_CONSTRAINT" => PRIORITY_CONSTRAINT_CODE,
        "AREA_CONSTRAINT" => AREA_CONSTRAINT_CODE,
        "DISPATCH_CONSTRAINT" => DISPATCH_CONSTRAINT_CODE,
        "TOUR_SIZE_CONSTRAINT" => TOUR_SIZE_CONSTRAINT_CODE,
        _ => -1,
    }
}
