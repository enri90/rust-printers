// https://github.com/apple/cups/blob/a8968fc4257322b1e4e191c4bccedea98d7b053e/cups/cups.h#L68

#[derive(Debug, Clone)]
pub enum CupStates {
    CupsWhichjobsAll = -1,
    CupsWhichjobsActive = 0,
    CupsWhichjobsCompleted = 1,
}

/**
 * Enum of the Job states
 */
#[derive(Debug, Clone)]
pub enum JobState {
    /**
     * The job is waiting to be printed
     */
    PENDING = 3,

    /**
     * Job is held for printing
     */
    HELD,

    /**
     * Job is currently printing
     */
    PROCESSING,

    /**
     * Job has been stopped
     */
    STOPPED,

    /**
     * Job has been canceled
     */
    CANCELED,

    /**
     * Job has aborted due to error
     */
    ABORTED,

    /**
     * Job has completed successfully
     */
    COMPLETED,

    /**
     * All other status like error, resources, manual intervention, etc...
     */
    UNKNOWN,
}

pub struct PrintJob {
    pub id: String,
    pub dest: String,
    pub title: String,
    pub user: String,
    pub format: String,
    pub state: JobState,
    pub size: String,
    pub priority: String,
    pub completed_time: String,
    pub creation_time: String,
    pub processing_time: String,
}

impl std::fmt::Debug for PrintJob {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "PrintJob {{
                \r  id: {:?},
                \r  dest: {:?},
                \r  title: {:?},
                \r  user: {:?},
                \r  format: {:?},
                \r  state: {:?},
                \r  size: {:?},
                \r  priority: {:?},
                \r  completed_time: {:?},
                \r  creation_time: {:?},
                \r  processing_time: {:?},
            \r}}",
            self.id,
            self.dest,
            self.title,
            self.user,
            self.format,
            self.state,
            self.size,
            self.priority,
            self.completed_time,
            self.creation_time,
            self.processing_time
        )
    }
}

impl Clone for PrintJob {
    fn clone(&self) -> PrintJob {
        return PrintJob {
            id: self.id.clone(),
            dest: self.dest.clone(),
            title: self.title.clone(),
            user: self.user.clone(),
            format: self.format.clone(),
            state: self.state.clone(),
            size: self.size.clone(),
            priority: self.priority.clone(),
            completed_time: self.completed_time.clone(),
            creation_time: self.creation_time.clone(),
            processing_time: self.processing_time.clone(),
        };
    }
}
