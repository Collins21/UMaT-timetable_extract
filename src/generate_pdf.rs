use dirs2;

use crate::model::Period;

pub fn generate_timetable_pdf(classes: &Vec<Period>, class: &str) {
    let font_family = genpdf::fonts::from_files("./assets", "OpenSans", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Class Timetable");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.push(genpdf::elements::Text::new(format!("{} TIMETABLE", class)));
    doc.set_line_spacing(1.5);

    let mut table = genpdf::elements::TableLayout::new(vec![2, 3, 2, 2, 2]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, true));

    // Add header row
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Day"))
        .element(genpdf::elements::Paragraph::new("Class Name"))
        .element(genpdf::elements::Paragraph::new("Start Time"))
        .element(genpdf::elements::Paragraph::new("End Time"))
        .element(genpdf::elements::Paragraph::new("Class Room"))
        .push()
        .expect("Failed to add header row");

    // Group classes by day and track previous day to avoid repetition
    let mut current_day: Option<String> = None;

    for period in classes {
        let day_display = if current_day.as_ref() == Some(&period.day) {
            // Day is the same, show empty cell
            String::new()
        } else {
            // Day changed, update current_day and show the day name
            current_day = Some(period.day.clone());
            period.day.clone()
        };

        table
            .row()
            .element(genpdf::elements::Paragraph::new(day_display))
            .element(genpdf::elements::Paragraph::new(&period.name))
            .element(genpdf::elements::Paragraph::new(&period.start_time))
            .element(genpdf::elements::Paragraph::new(&period.end_time))
            .element(genpdf::elements::Paragraph::new(&period.class_room))
            .push()
            .expect("Failed to add class row");
    }

    doc.push(table);
    let docs_dir = dirs2::document_dir().unwrap();

    doc.render_to_file(format!(
        "{}/{} timetable.pdf",
        docs_dir.as_path().display(),
        class
    ))
    .expect("Failed to render PDF");
    println!(
        "PDF generated successfully at {}/{} timetable.pdf",
        docs_dir.as_path().display(),
        class
    );
}

use crate::model::Exam;
use genpdf::{
    Alignment, Document, Element, SimplePageDecorator,
    elements::{Break, FrameCellDecorator, Paragraph, TableLayout},
    fonts,
    style::{Color, Style},
};

const NAVY: Color = Color::Rgb(15, 40, 80);
const STEEL: Color = Color::Rgb(41, 98, 163);
const DARK_GREY: Color = Color::Rgb(40, 40, 43);

fn header_cell(text: impl Into<String>) -> impl genpdf::Element {
    Paragraph::new(text.into())
        .aligned(Alignment::Center)
        .styled(Style::new().bold().with_color(STEEL).with_font_size(10))
}

fn body_cell(text: impl Into<String>, _alt: bool) -> impl genpdf::Element {
    Paragraph::new(text.into())
        .aligned(Alignment::Center)
        .styled(Style::new().with_color(DARK_GREY).with_font_size(9))
}

fn date_cell(text: impl Into<String>) -> impl genpdf::Element {
    Paragraph::new(text.into())
        .aligned(Alignment::Center)
        .styled(Style::new().bold().with_color(NAVY).with_font_size(9))
}
pub fn generate_exam_pdf(exams: &Vec<Exam>, department: &str) {
    let font_family = fonts::from_files("./assets", "OpenSans", None)
        .expect("Failed to load OpenSans from ./assets");

    let mut doc = Document::new(font_family);
    doc.set_title(format!("{} Exam Schedule", department));
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.5);

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(
        Paragraph::new("EXAMINATION SCHEDULE")
            .aligned(Alignment::Center)
            .styled(Style::new().bold().with_color(STEEL).with_font_size(14)),
    );

    doc.push(Break::new(0.5));

    let mut table = TableLayout::new(vec![2, 2, 4, 1, 2, 3]);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    table
        .row()
        .element(header_cell("Date"))
        .element(header_cell("Code"))
        .element(header_cell("Course Name"))
        //   .element(header_cell("Class"))
        .element(header_cell("Time"))
        .element(header_cell("Room"))
        .element(header_cell("Examiner"))
        .push()
        .expect("Failed to push header row");

    let mut prev_date = String::new();
    for (i, exam) in exams.iter().enumerate() {
        let alt = i % 2 == 0;

        let date_display = if exam.date == prev_date {
            String::new()
        } else {
            prev_date = exam.date.clone();
            exam.date.clone()
        };

        table
            .row()
            .element(date_cell(date_display))
            .element(body_cell(exam.course_num.clone(), alt))
            .element(body_cell(exam.course_name.clone(), alt))
            // .element(body_cell(exam.class.clone(), alt))
            .element(body_cell(exam.time.clone(), alt))
            .element(body_cell(exam.room.clone(), alt))
            .element(body_cell(exam.examiner.clone(), alt))
            .push()
            .expect("Failed to push exam row");
    }

    doc.push(table);

    // ── save ──────────────────────────────────────────────────────────────────
    let out_dir = dirs2::document_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let filename = format!(
        "{}/{}_exam_schedule.pdf",
        out_dir.display(),
        department.replace(' ', "_")
    );

    doc.render_to_file(&filename).expect("Failed to render PDF");
    println!("✅ Exam schedule saved to: {}", filename);
}
