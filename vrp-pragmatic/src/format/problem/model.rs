#[cfg(test)]
#[path = "../../../tests/unit/format/problem/model_test.rs"]
mod model_test;

extern crate serde_json;

use crate::format::{FormatError, Location};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter, Error, Read, Write};

// region Plan

/// Relation type.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RelationType {
    /// Relation type which locks jobs to specific vehicle in any order.
    Any,
    /// Relation type which locks jobs in specific order allowing insertion of other jobs in between.
    Sequence,
    /// Relation type which locks jobs in strict order, no insertions in between are allowed.
    Strict,
}

/// Relation is the way to lock specific jobs to specific vehicles.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    /// Relation type.
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub type_field: RelationType,
    /// List of job ids.
    pub jobs: Vec<String>,
    /// Vehicle id.
    pub vehicle_id: String,
    /// Vehicle shift index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_index: Option<usize>,
}

/// A job skills limitation for a vehicle.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSkills {
    /// Vehicle should have all of these skills defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Vec<String>>,
    /// Vehicle should have at least one of these skills defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Vec<String>>,
    /// Vehicle should have none of these skills defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none_of: Option<Vec<String>>,
}

/// Specifies a place for sub job.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct JobPlace {
    /// A job place location.
    pub location: Location,
    /// A job place duration (service time).
    pub duration: f64,
    /// A list of job place time windows with time specified in RFC3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Vec<Vec<String>>>,
}

/// Specifies a job task.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct JobTask {
    /// A list of possible places where given task can be performed.
    pub places: Vec<JobPlace>,
    /// Job place demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demand: Option<Vec<i32>>,
    /// A tag which will be propagated back within corresponding activity in solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// A customer job model. Actual tasks of the job specified by list of pickups and deliveries
/// which follows these rules:
/// * all of them should be completed or none of them.
/// * all pickups must be completed before any of deliveries.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Job {
    /// A job id.
    pub id: String,

    /// A list of pickup tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickups: Option<Vec<JobTask>>,

    /// A list of delivery tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<Vec<JobTask>>,

    /// A list of replacement tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacements: Option<Vec<JobTask>>,

    /// A list of service tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<JobTask>>,

    /// Job priority, bigger value - less important.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// A job skills limitations for serving a job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<JobSkills>,
}

/// A plan specifies work which has to be done.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Plan {
    /// List of jobs.
    pub jobs: Vec<Job>,
    /// List of relations between jobs and vehicles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<Relation>>,
}

// endregion

// region Fleet

/// Specifies vehicle costs.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VehicleCosts {
    /// Fixed is cost of vehicle usage per tour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,

    /// Cost per distance unit.
    pub distance: f64,

    /// Cost per time unit.
    pub time: f64,
}

/// Specifies vehicle shift start.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ShiftStart {
    /// Earliest possible departure date time in RFC3339 format.
    pub earliest: String,

    /// Latest possible departure date time in RFC3339 format. If omitted, departure time
    /// theoretically can be shifted till arrival. Set this value, if you want to limit
    /// departure time optimization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,

    /// Shift start location.
    pub location: Location,
}

/// Specifies vehicle shift end.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ShiftEnd {
    /// Earliest possible arrival date time in RFC3339 format.
    /// At the moment, not supported, reserved for future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest: Option<String>,

    /// Latest possible arrival date time in RFC3339 format.
    pub latest: String,

    /// Shift end location.
    pub location: Location,
}

/// Specifies vehicle shift.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VehicleShift {
    /// Vehicle shift start.
    pub start: ShiftStart,

    /// Vehicle shift end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<ShiftEnd>,

    /// Vehicle cargo dispatch parameters. If defined, vehicle starts empty at location,
    /// defined in ShiftStart, and navigates first to the one of specified places, e.g. to pickup
    /// the goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch: Option<Vec<VehicleDispatch>>,

    /// Vehicle breaks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breaks: Option<Vec<VehicleBreak>>,

    /// Vehicle reloads which allows vehicle to visit place where goods can be loaded or
    /// unloaded during single tour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reloads: Option<Vec<VehicleReload>>,
}

/// Specifies a dispatch place where vehicle can load cargo and start the tour.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleDispatch {
    /// A dispatch place location.
    pub location: Location,
    /// Specifies vehicle dispatch parameters.
    pub limits: Vec<VehicleDispatchLimit>,
    /// A tag which will be propagated back within corresponding activity in solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Specifies dispatch place limits to handle vehicles.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleDispatchLimit {
    /// Max amount of vehicles which can be dispatched during given period.
    pub max: usize,
    /// A dispatch start time in RFC3339 time format.
    pub start: String,
    /// A dispatch end time in RFC3339 time format.
    pub end: String,
}

/// Specifies a place where vehicle can load or unload cargo.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VehicleReload {
    /// A place location.
    pub location: Location,

    /// A total loading/reloading duration (service time).
    pub duration: f64,

    /// A list of time windows with time specified in RFC3339 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Vec<Vec<String>>>,

    /// A tag which will be propagated back within corresponding activity in solution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Vehicle limits.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleLimits {
    /// Max traveling distance per shift/tour.
    /// No distance restrictions when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,

    /// Max time per shift/tour.
    /// No time restrictions when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_time: Option<f64>,

    /// Max amount job activities.
    /// No job activities restrictions when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tour_size: Option<usize>,

    /// Specifies a list of areas where vehicle can serve jobs.
    /// No area restrictions when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_areas: Option<Vec<AreaLimit>>,
}

/// Specifies area limit.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaLimit {
    /// An area priority, bigger value - less important.
    /// Default is 1.
    pub priority: Option<usize>,
    /// An area outer shape.
    pub outer_shape: Vec<Location>,
}

/// Vehicle break time variant.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum VehicleBreakTime {
    /// Break time is defined by a time window with time specified in RFC3339 format.
    TimeWindow(Vec<String>),
    /// Break time is defined by a time offset range.
    TimeOffset(Vec<f64>),
}

/// Vehicle break.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct VehicleBreak {
    /// Break time.
    pub time: VehicleBreakTime,

    /// Break duration.
    pub duration: f64,

    /// Break locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

/// Specifies a vehicle type.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleType {
    /// Vehicle type id.
    pub type_id: String,

    /// Concrete vehicle ids.
    pub vehicle_ids: Vec<String>,

    /// Vehicle profile name.
    pub profile: String,

    /// Vehicle costs.
    pub costs: VehicleCosts,

    /// Vehicle shifts.
    pub shifts: Vec<VehicleShift>,

    /// Vehicle capacity.
    pub capacity: Vec<i32>,

    /// Vehicle skills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,

    /// Vehicle limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<VehicleLimits>,
}

/// Specifies routing profile.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Profile {
    /// Profile name.
    pub name: String,

    /// Profile type.
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub profile_type: String,

    /// Approximation speed (meters per second). Used only when routing matrix is not specified.
    /// Default value is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

/// Specifies fleet.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Fleet {
    /// Vehicle types.
    pub vehicles: Vec<VehicleType>,
    /// Routing profiles.
    pub profiles: Vec<Profile>,
}

// endregion

// region Objective

/// Specifies a group of objective functions.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Objectives {
    /// A list of primary objective functions. An accepted solution should not
    /// be worse of any of these.
    pub primary: Vec<Objective>,
    /// A list of secondary objective functions. An accepted solution can be worse
    /// by the secondary objective if it improves the primary one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Vec<Objective>>,
}

/// Specifies objective function types.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
pub enum Objective {
    /// An objective to minimize total cost.
    #[serde(rename(deserialize = "minimize-cost", serialize = "minimize-cost"))]
    MinimizeCost,

    /// An objective to minimize total tour amount.
    #[serde(rename(deserialize = "minimize-tours", serialize = "minimize-tours"))]
    MinimizeTours,

    /// An objective to maximize total tour amount.
    #[serde(rename(deserialize = "maximize-tours", serialize = "maximize-tours"))]
    MaximizeTours,

    /// An objective to minimize amount of unassigned jobs.
    #[serde(rename(deserialize = "minimize-unassigned", serialize = "minimize-unassigned"))]
    MinimizeUnassignedJobs {
        /// A multiplier to increase/decrease break is importance.
        /// Default is 1.
        #[serde(skip_serializing_if = "Option::is_none")]
        breaks: Option<f64>,
    },

    /// An objective to balance max load across all tours.
    #[serde(rename(deserialize = "balance-max-load", serialize = "balance-max-load"))]
    BalanceMaxLoad {
        /// A relative load in single tour before balancing takes place.
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<BalanceOptions>,
    },

    /// An objective to balance activities across all tours.
    #[serde(rename(deserialize = "balance-activities", serialize = "balance-activities"))]
    BalanceActivities {
        /// An options which can be used to specify minimum activity amount in a tour before
        /// it considered for balancing.
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<BalanceOptions>,
    },

    /// An objective to balance distance across all tours.
    #[serde(rename(deserialize = "balance-distance", serialize = "balance-distance"))]
    BalanceDistance {
        /// An options which can be used to specify minimum distance of a tour before
        /// it considered for balancing.
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<BalanceOptions>,
    },

    /// An objective to balance duration across all tours.
    #[serde(rename(deserialize = "balance-duration", serialize = "balance-duration"))]
    BalanceDuration {
        /// An options which can be used to specify minimum duration of a tour before
        /// it considered for balancing.
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<BalanceOptions>,
    },
}

/// Specifies balance objective options. At the moment, it uses coefficient of variation as
/// balancing measure.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct BalanceOptions {
    /// A balancing threshold specifies desired balancing level. Lower values can be ignored in
    /// favor of another objective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,

    /// A balancing tolerance specifies how much balance values should differ in order
    /// to be considered as not-equal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
}

// endregion

// region Common

/// A VRP problem definition.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Problem {
    /// Problem plan: customers to serve.
    pub plan: Plan,

    /// Problem resources: vehicles to be used, routing info.
    pub fleet: Fleet,

    /// Specifies objective functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Objectives>,
}

/// A routing matrix.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    /// A name of profile.
    pub profile: Option<String>,

    /// A date in RFC3999 for which routing info is applicable.
    pub timestamp: Option<String>,

    /// Travel distances (used to be in seconds).
    #[serde(alias = "durations")]
    pub travel_times: Vec<i64>,

    /// Travel durations (use to be in meters).
    pub distances: Vec<i64>,

    /// Error codes to mark unreachable locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_codes: Option<Vec<i64>>,
}

// endregion

/// Deserializes problem in json format from `BufReader`.
pub fn deserialize_problem<R: Read>(reader: BufReader<R>) -> Result<Problem, Vec<FormatError>> {
    serde_json::from_reader(reader).map_err(|err| {
        vec![FormatError::new(
            "E0000".to_string(),
            "cannot deserialize problem".to_string(),
            format!("check input json: '{}'", err),
        )]
    })
}

/// Deserializes routing matrix in json format from `BufReader`.
pub fn deserialize_matrix<R: Read>(reader: BufReader<R>) -> Result<Matrix, Vec<FormatError>> {
    serde_json::from_reader(reader).map_err(|err| {
        vec![FormatError::new(
            "E0001".to_string(),
            "cannot deserialize matrix".to_string(),
            format!("check input json: '{}'", err),
        )]
    })
}

/// Deserializes json list of locations from `BufReader`.
pub fn deserialize_locations<R: Read>(reader: BufReader<R>) -> Result<Vec<Location>, Vec<FormatError>> {
    serde_json::from_reader(reader).map_err(|err| {
        vec![FormatError::new(
            "E0000".to_string(),
            "cannot deserialize locations".to_string(),
            format!("check input json: '{}'", err),
        )]
    })
}

/// Serializes `problem` in json from `writer`.
pub fn serialize_problem<W: Write>(writer: BufWriter<W>, problem: &Problem) -> Result<(), Error> {
    serde_json::to_writer_pretty(writer, problem).map_err(Error::from)
}
