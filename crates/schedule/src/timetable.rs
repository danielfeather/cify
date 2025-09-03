use serde::{
    de::{self, Visitor},
    Deserialize,
};

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
    pub header: Header,
    pub tiploc_inserts: Vec<TiplocInsert>,
    pub tiploc_amends: Vec<TiplocAmend>,
    pub tiploc_deletes: Vec<TiplocDelete>,
    pub train_schedules: Vec<TrainSchedule>,
}

impl<'de> Deserialize<'de> for Timetable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimetableVisitor;

        impl<'de> Visitor<'de> for TimetableVisitor {
            type Value = Timetable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a sequence of one or more records")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let header: Header = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let tiploc_inserts: Vec<TiplocInsert> = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                let tiploc_amends: Vec<TiplocAmend> = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

                let tiploc_deletes: Vec<TiplocDelete> = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;

                let train_schedules: Vec<TrainSchedule> = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;

                Ok(Timetable {
                    header,
                    tiploc_inserts,
                    tiploc_amends,
                    tiploc_deletes,
                    train_schedules,
                })
            }
        }

        deserializer.deserialize_seq(TimetableVisitor)
    }
}

#[derive(Debug)]
pub struct TrainSchedule {
    bs: BasicSchedule,
    bsx: BasicScheduleExtra,
    origin_location: OriginLocation,
    intermediate_locations: Vec<IntermediateLocation>,
    terminating_location: TerminatingLocation,
}

impl<'de> Deserialize<'de> for TrainSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TrainScheduleVisitor;

        impl<'de> Visitor<'de> for TrainScheduleVisitor {
            type Value = TrainSchedule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "struct TrainSchedule")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let bs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let bsx = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let origin_location = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let intermediate_locations: Vec<IntermediateLocation> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                let terminating_location = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;

                Ok(TrainSchedule {
                    bs,
                    bsx,
                    origin_location,
                    intermediate_locations,
                    terminating_location,
                })
            }
        }

        deserializer.deserialize_seq(TrainScheduleVisitor)
    }
}

mod test {

    const SCHEDULE: &'static str = r#"BSNP132082412152505110000001 PXX1S758044121730001 EMU397 125      B A F        P
BX         TPY                                                                  
LOMNCRIAP 1610 16104A        TB                                                 
LIHLDGWJ            1612 00000000                                               
LIHLDG              1613 000000001                      1                       
LISLDLJN            1619H00000000   SL                                          
LIARDWCKJ           1621H00000000                       1                       
LIMNCRPIC 1624H1626H     1625162614    SL T                                     
LIMNCROXR 1628 1630      162816302        T                                     
LIMNCRDGT           1631 000000002                      1                       
LIWATSTJN           1633 00000000                                               
LIORDSLLJ           1633H00000000                       1                       
LISLFDCT            1635H000000002                                              
LIBDENJT            1641H00000000   DB                                          
LIBOLTON  1642H1644      000016444        U                                     
LILOSTCKJ           1647H00000000                                               
LICHORLEY           1653 00000000                     2                         
LIEUXTONJ           1657H00000000   FL                                          
LIPRSTRJN           1701H00000000   DFL                                         
LIPRST    1703 1705H     170317053  DFLDFLT                                     
LIPRSTNFJ           1706H00000000                                               
LIBBGHGL            1709H00000000                                               
LIGSTANG            1712H00000000                                               
LILANCSTR 1720 1721H     172017213        T                                     
LIMORCMSJ           1723H00000000                                               
LICRNFNJN           1726 00000000                                               
LIOXENHLM 1734 1735H     173417352        T                                     
LIGRIGG             1741H00000000                     1                         
LITEBAY             1746H00000000                                               
LISHAPSMT           1749H00000000                                               
LISHAPHNS           1752 00000000                                               
LIPNTH    1759H1801      180018012        T           2                         
LICARLILE 1816 1818      181618183        T                                     
LIGRETNAJ           1824H00000000                                               
LIKRKP863           1827 00000000                                               
LILCKRBIE 1835H1837      183618371        T                                     
LIBEATCK            1846 00000000DM                                             
LIBEATCKS           1852 00000000DM                                             
LIABINGTN           1857H00000000DM                   2                         
LICRSTRSS           1909 00000000                                               
LICRSTRSE           1910 00000000                                               
LIACHNGRY           1914 00000000                                               
LICOBB713           1916 00000000                                               
LIMDCLDRJ           1921 00000000                     1   1H                    
LISLATEFD           1930 00000000                                               
LIHAYMRKT 1932H1934      193300003  US    D              H                      
LIPRNCSTG           1936H00000000   X                                           
LTEDINBUR 1939 193912 X  TF                                                     "#;

    #[test]
    pub fn test_schedule() -> Result<(), Box<dyn std::error::Error>> {
        let schedule: super::TrainSchedule = crate::from_str(SCHEDULE)?;

        println!("{schedule:?}");

        Ok(())
    }
}
