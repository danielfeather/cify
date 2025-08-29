use crate::extract::{
    header::Header, tiploc::TiplocInsert, BasicSchedule, BasicScheduleExtra, IntermediateLocation,
    OriginLocation, TerminatingLocation, TiplocAmend, TiplocDelete,
};

#[derive(Debug)]
/// Type representing a timetable
///
/// To be a valid table, the input must contain a minimum of a header and a trailer
/// record type.
pub struct Timetable {
    header: Header,
    tiploc_inserts: Vec<TiplocInsert>,
    tiploc_amends: Vec<TiplocAmend>,
    tiploc_deletes: Vec<TiplocDelete>,
}

#[derive(Debug)]
pub struct TrainSchedule {
    bs: BasicSchedule,
    bsx: BasicScheduleExtra,
    origin_location: OriginLocation,
    intermediate_locations: Vec<IntermediateLocation>,
    terminating_location: TerminatingLocation,
}
