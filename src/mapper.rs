use rustling_ontology_values::dimension::{Dimension,DatetimeKind};
use rustling_ontology_values::output::OutputKind;

pub fn map_dimension(dimension: &mut Dimension, output_kind_filter: &[OutputKind]) {

    match dimension {

        Dimension::Datetime(datetime_value) => {
            // If Datetime IS NOT in the OutputKind filter, then SOME specific subtyping is required.
            // Technically if the client filter is None, then the filter will contain all possible
            // output kinds. This case is equivalent to check if the subkind in question is
            // the filter or not.
            // Find the subkind: Figure out the Datetime subtype from the Form, Grain and other
            // stuff contained in the Dimension::Datetime(datetime_value)
            let date_time_grain = (datetime_value.constraint.grain_left().is_date_grain() &&
                datetime_value.constraint.grain_right().is_time_grain()) ||
                (datetime_value.constraint.grain_right().is_date_grain() &&
                    datetime_value.constraint.grain_left().is_time_grain());
            let date_grain = !date_time_grain && datetime_value.constraint.grain_min().is_date_grain();
            let time_grain = !date_time_grain && datetime_value.constraint.grain_min().is_time_grain();
            let mut has_direction = false;
            if let Some(_bounded_direction) = datetime_value.direction { has_direction = true };
            let period_form = datetime_value.has_period_form().unwrap_or(false) || has_direction;

            // Assign the relevant Datetime subtype (field datetime_kind of the datetime_value)
            if (output_kind_filter.is_empty() || output_kind_filter.contains(&OutputKind::Date)) &&
                !period_form && date_grain {
                datetime_value.datetime_kind = DatetimeKind::Date;
            } else if (output_kind_filter.is_empty() || output_kind_filter.contains(&OutputKind::Time)) &&
                !period_form && time_grain {
                datetime_value.datetime_kind = DatetimeKind::Time;
            } else if (output_kind_filter.is_empty() || output_kind_filter.contains(&OutputKind::DatePeriod)) &&
                period_form && date_grain {
                datetime_value.datetime_kind = DatetimeKind::DatePeriod;
            } else if (output_kind_filter.is_empty() || output_kind_filter.contains(&OutputKind::TimePeriod)) &&
                period_form && time_grain {
                datetime_value.datetime_kind = DatetimeKind::TimePeriod;
            } else {
                // If the dimension is datetime and none of the 4 subtypes, then it's the
                // complement subtype, hence Datetime
                datetime_value.datetime_kind = DatetimeKind::Datetime;
            }
        },
        // If the dimension is other than Datetime, then no specific mapping is required.
        _ => {},
    }
}