#[derive(PartialEq)]
enum DiskType {
    SSD,
    HDD,
}
#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    println!("\n<=====================================>\n");
    let disk_type = DiskType::SSD;
    // Can't compare them like this!
    if disk_type == DiskType::SSD 
    {
        println!("Disk type is SSD");
    } 
    else 
    {
        println!("Disk type is HDD");
    }


    println!("\n<=====================================>\n");
    match disk_type 
    {
        DiskType::SSD => println!("Disk type is SSD"),
        DiskType::HDD => println!("Disk type is HDD"),
        _ => println!("Unknown disk type"),
    }
    let disk_size = DiskSize::GB(128);
    println!("{:?}", disk_size);
}
