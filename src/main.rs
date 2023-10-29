use std::env;

mod transformations;

mod model;
use model::transform_mode::TransformMode;

mod modes;
use modes::real_time::real_time_processing;
use modes::one_off::one_off_processing;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_param = args.get(1).cloned();
    let program_param_curated = program_param
                                                .clone()
                                                .map(|x| x.to_lowercase());

    let program_mode_parsed = match program_param.as_deref() {
        Some(s) => TransformMode::from_str(&s),
        None => Some(TransformMode::RealTime),
    };

    let program_mode = match program_mode_parsed {
        Some(mode) => mode,
        None => {
            eprintln!("Wrong argument({:?}) given. Eligible arguments are 
                [csv, lowercase, uppercase, no-spaces, slugify] or no argument.Exiting...", program_param_curated);
            return;
        },
    };

    match program_mode {
        TransformMode::RealTime => real_time_processing(),
        one_off_mode => one_off_processing(one_off_mode),
    }
}
