enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

// fn format_size(size: u64) -> String {
//     let filesize = match size {
//         0..=999 => FileSize::Bytes(size),
//         1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
//         1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
//         _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
//     };

//     match filesize {
//         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
//         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
//         FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
//         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
//     }
// }




// fn main() {
//     println!("\n<=====================================>\n");
//     let result = format_size(6888837399);
//     println!("{}", result);


//     println!("<=====================================>");
//     let result2 = format_size(6888837);
//     println!("{}", result2);

//     println!("<=====================================>");
//     let result3 = format_size(6888);
//     println!("{}", result3);

// }






impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", *kb as f64 / 1000.0),
            FileSize::Megabytes(mb) => format!("{:.2} MB", *mb as f64 / 1000.0),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", *gb as f64 / 1000.0),
        }
    }
}


fn main() {
    let size = 2000000000;
    println!("<=====================================>");
    let filesize = match size {
        0..=999 => FileSize:: Bytes(size),
        1000..=999_999 => FileSize:: Kilobytes (size as f64 / 1000.0),
        1_000_000..=999_999_999=> FileSize:: Megabytes (size as f64 / 1_000_000.0),
        _ => FileSize:: Gigabytes (size as f64 / 1_000_000_000.0),
    };

    println!("File size: {}", filesize.format_size());
}