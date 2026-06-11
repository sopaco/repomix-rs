use indicatif::{ProgressBar, ProgressStyle};
use repomix_core::packager::ProgressCallback;

/// 进度指示器
pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        Self { pb }
    }

    pub fn update(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    #[allow(dead_code)]
    pub fn finish(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }

    #[allow(dead_code)]
    pub fn finish_with_success(&self, message: &str) {
        self.pb.finish_with_message(format!("✔ {}", message));
    }

    pub fn finish_with_error(&self, message: &str) {
        self.pb.finish_with_message(format!("✖ {}", message));
    }
}

/// 实现 ProgressCallback，将 packager 内部进度事件桥接到 spinner。
/// on_complete 不结束 spinner——由调用方决定何时 finish（保留 println! 报告）。
impl ProgressCallback for Spinner {
    fn on_progress(&self, message: &str) {
        self.update(message);
    }

    fn on_complete(&self, _message: &str) {
        // 故意为空：spinner 由调用方在打印报告前手动 finish
    }

    fn on_error(&self, message: &str) {
        self.finish_with_error(message);
    }
}

/// 带进度的进度条
#[allow(dead_code)]
pub struct ProgressBar2 {
    pb: ProgressBar,
}

#[allow(dead_code)]
impl ProgressBar2 {
    pub fn new(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.set_message(message.to_string());

        Self { pb }
    }

    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    pub fn finish(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }
}
