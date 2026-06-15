use crate::{checkpoint::get_task_checkpoint, models::model_checkpoint::FileDescription};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{
    fmt::Write,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use tokio::task::yield_now;

/// 进度条，使用时在主线程之外的线程使用
pub async fn quantify_processbar(
    total: u64,
    stop_mark: Arc<AtomicBool>,
    checkpoint_file_path: &str,
    list_files: Option<Vec<FileDescription>>,
) {
    let pb = ProgressBar::new(total);
    let progress_style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("#>-");
    pb.set_style(progress_style);

    while !stop_mark.load(std::sync::atomic::Ordering::Relaxed) {
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
        let checkpoint = match get_task_checkpoint(checkpoint_file_path) {
            Ok(c) => c,
            Err(e) => {
                log::error!("{:?}", e);
                continue;
            }
        };

        let mut executed = checkpoint.executing_file_position.line_num;

        if let Some(list_files) = &list_files {
            if let Ok(f_n) = TryInto::<usize>::try_into(checkpoint.executing_file_position.file_num)
            {
                executed += list_files
                    .iter()
                    .map(|f| f.total_lines)
                    .take(f_n)
                    .sum::<u64>();
            }
        }
        pb.set_position(executed);
        log::info!("executed:{},total:{}", executed, total);

        yield_now().await;
    }
    log::info!("total:{},executed:{}", total, total);
    pb.set_position(total);
    pb.finish_with_message("Finish");
}

pub fn prompt_processbar(message: &str) -> ProgressBar {
    let pd = ProgressBar::new_spinner();
    pd.enable_steady_tick(Duration::from_millis(200));
    pd.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&[
                "▰▱▱▱▱▱▱",
                "▰▰▱▱▱▱▱",
                "▰▰▰▱▱▱▱",
                "▰▰▰▰▱▱▱",
                "▰▰▰▰▰▱▱",
                "▰▰▰▰▰▰▱",
                "▰▰▰▰▰▰▰",
            ]),
    );
    pd.set_message(message.to_string());
    pd
}
