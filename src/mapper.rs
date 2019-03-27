use rustling_ontology_values::dimension::{Dimension, DatetimeKind};


// Internal cooking to determine the subtype of a Datetime value. This could ideally be performed
// by the client itself, it doesn't have (shouldn't have?) to be an internal rustling thing.
// We would prefer not changing the Dimension itself, but for now that's how we carry the mapping
// info throughout the tagger and context, to be passed to the candidates, and then to the final
// output.
pub fn map_node(dimension: &mut Dimension) {
    match dimension {
        Dimension::Datetime(datetime_value) => {

            // Figure out the Datetime subtype from the Form, Grain and other stuff contained in the
            // Dimension::Datetime(datetime_value)
            let date_time_grain = (datetime_value.constraint.grain_left().date_grain() &&
                datetime_value.constraint.grain_right().time_grain()) ||
                (datetime_value.constraint.grain_right().date_grain() &&
                    datetime_value.constraint.grain_left().time_grain());
            let has_date_grain = !date_time_grain && datetime_value.constraint.grain_min().date_grain();
            let has_time_grain = !date_time_grain && datetime_value.constraint.grain_min().time_grain();
            let is_span = Some(true) == datetime_value.period_form();

            // Assign the relevant Datetime subtype (field datetime_kind of the datetime_value)
            if !is_span && has_date_grain {
                datetime_value.set_datetime_kind(DatetimeKind::Date);
            } else if !is_span && has_time_grain {
                datetime_value.set_datetime_kind(DatetimeKind::Time);
            } else if is_span && has_date_grain {
                datetime_value.set_datetime_kind(DatetimeKind::DatePeriod);
            } else if is_span && has_time_grain {
                datetime_value.set_datetime_kind(DatetimeKind::TimePeriod);
            } else {
                // If the dimension is datetime and none of the 4 subtypes, then it's the
                // complement subtype, hence Datetime
                datetime_value.set_datetime_kind(DatetimeKind::Datetime)
            }
        },
        _ => {},
    }
}