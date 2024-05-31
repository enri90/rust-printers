pub trait PlatformPrinterGetters {
    fn get_name(&self) -> String;

    fn get_is_default(&self) -> bool;

    fn get_system_name(&self) -> String;

    fn get_marker_and_model(&self) -> String;

    fn get_is_shared(&self) -> bool;

    fn get_uri(&self) -> String;

    fn get_location(&self) -> String;

    fn get_state(&self) -> String;
}

pub trait JobGetters {
    fn get_id(&self) -> String;

    fn get_dest(&self) -> String;

    fn get_title(&self) -> String;

    fn get_user(&self) -> String;

    fn get_format(&self) -> String;

    fn get_state(&self) -> String;

    fn get_size(&self) -> String;

    fn get_priority(&self) -> String;

    fn get_creation_time(&self) -> String;

    fn get_completed_time(&self) -> String;

    fn get_processing_time(&self) -> String;
}
