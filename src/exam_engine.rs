use calamine::{DataType, Reader, Xlsx, open_workbook};

use crate::{generate_pdf::generate_exam_pdf, input_mod::InputMod, model::Exam};

pub fn extract_exam_schedule(args: InputMod) {
    let mut exam_sheet: Xlsx<_> =
        open_workbook(args.file_path.to_str().unwrap_or("Failed to open file"))
            .expect("Unable to open file");
    // let mut exam_sheet: Xlsx<_> = open_workbook("C:/Users/USER/Desktop/exxams.xlsx").unwrap();
    let mut papers: Vec<Exam> = Vec::new();
    if let Some(sheet) = exam_sheet.worksheet_range_at(0) {
        let sheet_value = sheet.unwrap();
        for (row, _, data) in sheet_value.used_cells() {
            let date = if sheet_value
                .get_value((row as u32, 0))
                .unwrap()
                .is_datetime()
            {
                sheet_value
                    .get_value((row as u32, 0))
                    .unwrap()
                    .get_datetime()
                    .unwrap()
                    .as_f64() as i32
            } else {
                0.0 as i32
            };

            let course_name = sheet_value.get_value((row as u32, 2)).unwrap().to_string();
            let course_num = sheet_value.get_value((row as u32, 1)).unwrap().to_string();
            let time = sheet_value.get_value((row as u32, 8)).unwrap().to_string();

            let room = sheet_value.get_value((row as u32, 6)).unwrap().to_string();
            let examiner = sheet_value.get_value((row as u32, 5)).unwrap().to_string();

            let class = sheet_value.get_value((row as u32, 3)).unwrap().to_string();
            if data.to_string().to_lowercase().contains("ce iv") {
                let exam = Exam {
                    course_name: course_name,
                    date: excel_serial_to_date(date),
                    course_num: course_num,
                    time: time,
                    room: room,
                    examiner: examiner,
                    class: class,
                };
                papers.push(exam);
            }
        }
        generate_exam_pdf(&papers, "Computing and Data Analytics");
    }

    //   generate_exam_pdf(&papers, "CE", "2026");
}

fn excel_serial_to_date(serial: i32) -> String {
    // Excel epoch is Jan 1, 1900 = serial 1
    // Adjust for the Lotus 1-2-3 leap year bug (serial 60 = fake Feb 29 1900)
    let days = if serial >= 60 { serial - 2 } else { serial - 1 };

    // Days since Unix epoch (Jan 1, 1970)
    // From Jan 1, 1900 to Jan 1, 1970 = 25567 days
    let unix_days = days - 25567;

    // Calculate year, month, day from unix day count
    let mut remaining = unix_days;
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let months = [
        31,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let mut month = 0;
    for days_in_month in months {
        if remaining < days_in_month {
            break;
        }
        remaining -= days_in_month;
        month += 1;
    }

    let day = remaining + 1;
    let short_year = year % 100;

    format!("{}-{}-{:02}", day, month_names[month], short_year)
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
