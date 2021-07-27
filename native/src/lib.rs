use rustler::{Error, NifResult};

#[derive(rustler::NifMap)]
pub struct JemallocStats {
    active: u64,
    allocated: u64,
    epoch: u64,
    mapped: u64,
    metadata: u64,
    resident: u64,
    retained: u64,
}

macro_rules! jemalloc_value {
    ($name: ident) => {
        jemalloc_ctl::$name::mib()
            .and_then(|x| x.read())
            .map(|x| x as u64)
            .map_err(|x| Error::Term(Box::new(x.to_string())))
    };
}

macro_rules! jemalloc_stat_value {
    ($name: ident) => {
        jemalloc_ctl::stats::$name::mib()
            .and_then(|x| x.read())
            .map(|x| x as u64)
            .map_err(|x| Error::Term(Box::new(x.to_string())))
    };
}

#[rustler::nif]
pub fn jemalloc_allocation_info() -> NifResult<JemallocStats> {
    jemalloc_ctl::epoch::mib().and_then(|x| x.advance()).ok();

    Ok(JemallocStats {
        active: jemalloc_stat_value!(active)?,
        allocated: jemalloc_stat_value!(allocated)?,
        epoch: jemalloc_value!(epoch)?,
        mapped: jemalloc_stat_value!(mapped)?,
        metadata: jemalloc_stat_value!(metadata)?,
        resident: jemalloc_stat_value!(resident)?,
        retained: jemalloc_stat_value!(retained)?,
    })
}
