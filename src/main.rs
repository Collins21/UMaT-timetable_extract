mod display;
mod generate_pdf;
mod input_mod;
mod model;
mod utils;
// use indicatif::ProgressBar;
use std::sync::Arc;

use calamine::{self, DataType, Dimensions, Reader, Xlsx, open_workbook};

use crate::{model::Period, utils::sort_table};

fn main() {
    let input = display::display();
    let mut time_table: Xlsx<_> =
        open_workbook(input.file_path.as_path()).expect("Unable to open file");
    let mut classes: Vec<Period> = Vec::new();
    let sheets = time_table.worksheets();
    // let progress_bar = ProgressBar::wrap_read();
    //progress_bar.set_message("Processing files...");
    for (sheet, value) in sheets {
        let merged_cells = time_table.worksheet_merge_cells(&sheet).unwrap().unwrap();

        for (row, col, data) in value.used_cells() {
            let class_name = data
                .as_string()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            let is_match = data
                .as_string()
                .unwrap()
                .to_lowercase()
                .contains(input.level.to_lowercase().as_str());
            let is_merged = &merged_cells.contains(&Dimensions {
                start: (row as u32, col as u32),
                end: (row as u32, col as u32 + 1),
            });

            let sheet_name = Arc::clone(&Arc::new(sheet.to_string()));
            if !data.is_empty() && is_match {
                let period = Period {
                    name: String::from(class_name),
                    day: sheet_name.to_string(),
                    is_morning: col < 7,
                    start_time: value.get((7, col)).unwrap().to_string(),
                    end_time: if *is_merged {
                        value.get((7, col + 1)).unwrap().to_string()
                    } else {
                        value.get((7, col)).unwrap().to_string()
                    },
                    class_room: value.get((row, 0)).unwrap().to_string(),
                };

                classes.push(period);
            }
        }
    }
    sort_table(&mut classes);
    generate_pdf::generate_pdf(&classes, &input.level.to_uppercase());
}
