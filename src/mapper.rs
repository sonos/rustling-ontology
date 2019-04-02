use rustling_ontology_values::dimension::{Dimension, DatetimeKind};
use rustling_ontology_values::output::OutputKind;


// Internal cooking to determine the subtype of a Datetime value. This could ideally be performed
// by the client itself, it doesn't have (shouldn't have?) to be an internal rustling thing.
// We would prefer not changing the Dimension itself, but for now that's how we carry the mapping
// info throughout the tagger and context, to be passed to the candidates, and then to the final
// output.
pub fn map_dimension(output_kind_filter: &Vec<&OutputKind>, dimension: &mut Dimension) {

    match dimension {

        Dimension::Datetime(datetime_value) => {
            // If Datetime IS NOT in the OutputKind filter, then SOME specific subtyping is required.
            // Technically if the client filter is None, then the filter will contain all possible
            // output kinds. This case is equivalent to check if the subkind in question is
            // the filter or not.
            // Find the subkind: Figure out the Datetime subtype from the Form, Grain and other
            // stuff contained in the Dimension::Datetime(datetime_value)
            let date_time_grain = (datetime_value.constraint.grain_left().date_grain() &&
                datetime_value.constraint.grain_right().time_grain()) ||
                (datetime_value.constraint.grain_right().date_grain() &&
                    datetime_value.constraint.grain_left().time_grain());
            let date_grain = !date_time_grain && datetime_value.constraint.grain_min().date_grain();
            let time_grain = !date_time_grain && datetime_value.constraint.grain_min().time_grain();
            let period_form = datetime_value.period_form().unwrap_or(false);

            // Assign the relevant Datetime subtype (field datetime_kind of the datetime_value)
            if get_datetime_subkind(output_kind_filter, &&OutputKind::Date) &&
                !period_form && date_grain {
                datetime_value.set_datetime_kind(DatetimeKind::Date);
            } else if get_datetime_subkind(output_kind_filter, &&OutputKind::Time) &&
                !period_form && time_grain {
                datetime_value.set_datetime_kind(DatetimeKind::Time);
            } else if get_datetime_subkind(output_kind_filter, &&OutputKind::DatePeriod) &&
                period_form && date_grain {
                datetime_value.set_datetime_kind(DatetimeKind::DatePeriod);
            } else if get_datetime_subkind(output_kind_filter, &&OutputKind::TimePeriod) &&
                period_form && time_grain {
                datetime_value.set_datetime_kind(DatetimeKind::TimePeriod);
            } else {
                // If the dimension is datetime and none of the 4 subtypes, then it's the
                // complement subtype, hence Datetime
                datetime_value.set_datetime_kind(DatetimeKind::Datetime);
            }
        },
        // If the dimension is other than Datetime, then no specific mapping is required.
        _ => {},
    }
}

pub fn get_datetime_subkind(output_kind_filter: &Vec<&OutputKind>, output_kind: &OutputKind) -> bool {
    output_kind_filter.is_empty() || output_kind_filter.contains(&&output_kind)
}