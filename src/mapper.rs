use rustling_ontology_values::dimension::{Dimension,DatetimeKind};
use rustling_ontology_values::output::OutputKind;
pub use rustling_ontology_moment::{Interval, Moment, Local, TimeZone};

pub fn map_dimension(dimension: &mut Dimension, output_kind_filter: &[OutputKind]) {

    match dimension {

        Dimension::Datetime(datetime_value) => {

            // If the filter contains Datetime but no datetime subkind, then no subtyping is required
            if output_kind_filter.contains(&OutputKind::Datetime) &&
                !output_kind_filter.contains(&OutputKind::Date) &&
                !output_kind_filter.contains(&OutputKind::DatePeriod) &&
                !output_kind_filter.contains(&OutputKind::Time) &&
                !output_kind_filter.contains(&OutputKind::TimePeriod) {}

            // If Datetime IS NOT in the OutputKind filter, then SOME specific subtyping is
            // required, through the field datetime_kind of the datetime_value.

            // Technically if the client filter is None, then the filter will contain all possible
            // output kinds, in which case we want to return more specific subkinds rather than
            // Datetime.

            // So Datetime will be returned only if Datetime has been
            // explicitly defined as the filter

            // Helper values to later determine the datetime subkind, if some datetime subkind
            // hasn't already been set by the grammar. If so then the match will work if the filter
            // allows for it.
            if !is_valid_datetime_kind(&datetime_value.datetime_kind) {

                // Find the subkind: Figure out the Datetime subtype from the Form, Grain and other
                // stuff contained in the Dimension::Datetime(datetime_value)

                let mut date_time_grain = (datetime_value.constraint.grain_left().is_date_grain() &&
                    datetime_value.constraint.grain_right().is_time_grain()) ||
                    (datetime_value.constraint.grain_right().is_date_grain() &&
                        datetime_value.constraint.grain_left().is_time_grain());

                let date_grain = !date_time_grain && datetime_value.constraint.grain_min().is_date_grain();

                let time_grain = datetime_value.is_today_date_and_time() ||
                    !date_time_grain && datetime_value.constraint.grain_min().is_time_grain();

                let mut has_direction = false;
                if let Some(_bounded_direction) = datetime_value.direction { has_direction = true };
                let is_period = datetime_value.has_period_form().unwrap_or(false) ||
                    datetime_value.has_period_grain().unwrap_or(false) ||
                    has_direction;

                // Assign the relevant Datetime subtype depending on above helper values and
                // subkinds in the filter
                if check_filter(output_kind_filter, &OutputKind::Date) &&
                    !is_period && date_grain {
                    datetime_value.datetime_kind = DatetimeKind::Date;
                } else if check_filter(output_kind_filter, &OutputKind::Time) &&
                    !is_period && time_grain {
                    datetime_value.datetime_kind = DatetimeKind::Time
                } else if check_filter(output_kind_filter, &OutputKind::DatePeriod)
                    && is_period && date_grain {
                    datetime_value.datetime_kind = DatetimeKind::DatePeriod;
                } else if check_filter(output_kind_filter, &OutputKind::TimePeriod) &&
                    is_period && time_grain {
                    datetime_value.datetime_kind = DatetimeKind::TimePeriod;
                } else {
                    // If the dimension is datetime and none of the 4 subtypes, then it's the
                    // complement subtype, hence Datetime
                    datetime_value.datetime_kind = DatetimeKind::Datetime;
                }
            }
        },
        // If the dimension is other than Datetime, then no specific mapping is required.
        _ => {},
    }

    fn check_filter(output_kind_filter: &[OutputKind], output_kind: &OutputKind) -> bool {
        output_kind_filter.is_empty() || output_kind_filter.contains(&output_kind)
    }

    // This is here and not e.g. on the DatetimeKind type because it's up to the mapper to decide
    // which kinds are valid
    fn is_valid_datetime_kind(datetime_kind: &DatetimeKind) -> bool {
        match datetime_kind {
            &DatetimeKind::Empty => false,
            &DatetimeKind::DatetimeComplement { .. } => false,
            _ => true,
        }
    }

}