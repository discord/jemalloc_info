use rustler::{rustler_atoms, Encoder, Env, Term};

rustler_atoms! [
    atom ok;
    atom error;

    atom epoch;
    atom active;
    atom allocated;
    atom mapped;
    atom metadata;
    atom resident;
    atom retained;
];

macro_rules! jemalloc_stat_value {
    ($name: ident) => {
        jemalloc_ctl::stats::$name::mib()
            .and_then(|x| x.read())
            .map_err(|x| format!("{}", x))
    };
}

macro_rules! jemalloc_value {
    ($name: ident) => {
        jemalloc_ctl::$name::mib()
            .and_then(|x| x.read())
            .map_err(|x| format!("{}", x))
    };
}

pub fn allocation_info<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Term<'a> {
    fn values(env: Env) -> Result<Term, String> {
        // epoch increment required to refresh jemalloc stats.
        jemalloc_ctl::epoch::mib().and_then(|x| x.advance()).ok();

        let values = &[
            (epoch(), jemalloc_value![epoch]? as u64),
            (active(), jemalloc_stat_value![active]? as u64),
            (allocated(), jemalloc_stat_value![allocated]? as u64),
            (mapped(), jemalloc_stat_value![mapped]? as u64),
            (metadata(), jemalloc_stat_value![metadata]? as u64),
            (resident(), jemalloc_stat_value![resident]? as u64),
            (retained(), jemalloc_stat_value![retained]? as u64),
        ];

        let map = Term::map_new(env);
        let map = values.iter().fold(map, |x, (name, value)| {
            let name = name.encode(env);
            let value = value.encode(env);

            // according to documentation, this only fails if `x` is not a map.
            x.map_put(name, value).map_err(|_| ()).unwrap()
        });

        Ok(map)
    }

    match values(env) {
        Ok(x) => (ok(), x).encode(env),
        Err(e) => (error(), e).encode(env),
    }
}
