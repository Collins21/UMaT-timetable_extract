mod generate_pdf;
mod model;

use calamine::{self, DataType, Dimensions, Reader, Xlsx, open_workbook};

use crate::model::Period;

fn main() {
    let path = "assets/tables.xlsx";
    let mut time_table: Xlsx<_> = open_workbook(path).expect("Unable to open file");
    let mut classes: Vec<Period> = Vec::new();
    let sheets = time_table.worksheets();

    for (sheet, value) in sheets {
        for (row, col, data) in value.used_cells() {
            let class_name = data
                .as_string()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            let is_match = data.as_string().unwrap().to_lowercase().contains("ce 4");
            let merged_cells = time_table.worksheet_merge_cells(&sheet).unwrap().unwrap();
            let is_merged = &merged_cells.contains(&Dimensions {
                start: (row as u32, col as u32),
                end: (row as u32, col as u32 + 1),
            });
            if !data.is_empty() && is_match {
                let period = Period {
                    name: String::from(class_name),
                    day: sheet.clone(),
                    start_time: value.get((7, col)).unwrap().to_string(),
                    end_time: if *is_merged {
                        value.get((7, col + 1)).unwrap().to_string()
                    } else {
                        value.get((7, col)).unwrap().to_string()
                    },
                    class_room: value.get((row, 0)).unwrap().to_string(),
                };
                println!("{:?}", period);

                classes.push(period);
            }
        }
    }
    generate_pdf::generate_pdf(&classes);
}
