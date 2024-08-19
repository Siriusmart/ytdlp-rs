pub enum Progress {
    Finished {
        downloaded_bytes: u64,
        total_bytes: u64,
        filename: String,
        status: ProgressStatus,
        elapsed: f64,
        ctx_id: Option<u32>,
        speed: f64,

        speed_str: String,
        total_bytes_str: String,
        elapsed_str: String,
        percent_str: String,
        default_template: String,
    },
    Downloading {
        downloaded_bytes: u64,
        total_bytes: u64,
        tempfilename: String,
        filename: String,
        eta: Option<u64>,
        speed: Option<f64>,
        elapsed: Option<f64>,
        ctx_id: Option<u32>,

        eta_str: String,
        speed_str: String,
        percent_str: String,
        total_bytes_str: String,
        total_bytes_estimate_str: String,
        downloaded_bytes_str: String,
        elapsed_str: String,
        default_template: String,
    }, // there is also "failed", but I cannot reproduce it
       // and there are no docs
}

pub enum ProgressStatus {}
