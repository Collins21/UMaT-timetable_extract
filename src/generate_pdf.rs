use dirs2;

use crate::model::Period;

pub fn generate_pdf(classes: &Vec<Period>, class: &str) {
    let font_family = genpdf::fonts::from_files("./assets", "OpenSans", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Class Timetable");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.push(genpdf::elements::Text::new(format!("{} TIMETABLE", class)));
    doc.set_line_spacing(1.3);
    let mut table = genpdf::elements::TableLayout::new(vec![2, 3, 2, 2, 2]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, true));
    table
        .row()
        .element(genpdf::elements::Paragraph::new("Day"))
        .element(genpdf::elements::Paragraph::new("Class Name"))
        .element(genpdf::elements::Paragraph::new("Start Time"))
        .element(genpdf::elements::Paragraph::new("End Time"))
        .element(genpdf::elements::Paragraph::new("Class Room"))
        .push()
        .expect("Failed to add header row");
    for class in classes {
        table
            .row()
            .element(genpdf::elements::Paragraph::new(&class.day))
            .element(genpdf::elements::Paragraph::new(&class.name))
            .element(genpdf::elements::Paragraph::new(&class.start_time))
            .element(genpdf::elements::Paragraph::new(&class.end_time))
            .element(genpdf::elements::Paragraph::new(&class.class_room))
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
    //   bar.finish();
}
