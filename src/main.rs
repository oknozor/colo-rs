use ansi_term::Colour::{self, *};
use ansi_term::Style;

const DOTS: &str = "  •••  ";
const ANSI_COLORS: &[Colour] = &[Black, Red, Green, Yellow, Blue, Purple, Cyan, White];

fn main() {
    print_titles();

    ANSI_COLORS.iter().for_each(|color| {
        print_line(color.to_owned(), false);
        print_line(color.to_owned(), true);
    });
}

fn print_titles() {
    print!("          ");
    for i in 40..=47 {
        print!("   {}m  ", i)
    }

    println!()
}

fn print_line(color: Colour, bold: bool) {
    let style = if bold {
        Style::new().fg(color).bold()
    } else {
        Style::new().fg(color)
    };

    let prefix = style.prefix().to_string();

    if bold {
        print!("{}", &prefix[2..prefix.len() - 1]);
    } else {
        print!(" {}m", &prefix[2..prefix.len() - 1]);
    }

    print!("{}", style.paint(DOTS));

    ANSI_COLORS
        .iter()
        .map(|bg| style.on(*bg))
        .for_each(|style| print!("{} ", style.paint(DOTS)));

    println!()
}
