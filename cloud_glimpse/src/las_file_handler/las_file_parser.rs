use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;#[derive(Clone, Copy)]

#[derive(Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LasFileHeader {
    pub signature: [u8; 4],
    pub file_source_id: u16,
    pub global_encoding: u16,
    pub project_id_guid_data_1: u32,
    pub project_id_guid_data_2: u16,
    pub project_id_guid_data_3: u16,
    pub project_id_guid_data_4: [u8; 8],
    pub version_major: u8,
    pub version_minor: u8,
    pub system_identifier: [u8; 32],
    pub generating_software: [u8; 32],
    pub file_creation_day_of_year: u16,
    pub file_creation_year: u16,
    pub header_size: u16,
    pub offset_to_point_data: u32,
    pub number_of_variable_length_records: u32,
    pub point_data_record_format: u8,
    pub point_data_record_length: u16,
    pub legacy_number_of_point_records: u32,
    pub legacy_number_of_points_by_return: [u32; 5],
    pub x_scale_factor: f64,
    pub y_scale_factor: f64,
    pub z_scale_factor: f64,
    pub x_offset: f64,
    pub y_offset: f64,
    pub z_offset: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub start_of_waveform_data_packet_record: u64,
    pub start_of_first_extended_variable_length_record: u64,
    pub number_of_extended_variable_length_records: u32,
}

#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PointRecord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub intensity: u16,
    pub return_info: u8,
    pub classification_flags: u8,
    pub scanner_channel: u8,
    pub scan_direction_flag: bool,
    pub edge_of_flight_line: bool,
    pub classification: u8,
    pub user_data: u8,
    pub scan_angle: i16,
    pub point_source_id: u16,
    pub gps_time: f64,
}

pub fn read_las_file_header(file: &mut File) -> io::Result<LasFileHeader> {
    let mut buffer = [0; std::mem::size_of::<LasFileHeader>()];
    file.read_exact(&mut buffer)?;
    let header = LasFileHeader::from_le_bytes(&buffer)?;
    Ok(header)
}

impl LasFileHeader {
    fn from_le_bytes(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() != std::mem::size_of::<Self>() {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Incorrect length for header data",
                    ));
        }
        Ok(unsafe { std::ptr::read(bytes.as_ptr() as *const _) })
    }
}

impl PointRecord {
    pub fn new(data: &[u8]) -> io::Result<Self> {
        if data.len() != mem::size_of::<Self>() {
            return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Incorrect length for point record data",
                    ));
        }
        Ok(unsafe { std::ptr::read(data.as_ptr() as *const _) })
    }

    pub fn return_number(&self) -> u8 {
        self.return_info & 0x0F // Extracts the lower 4 bits
    }

    pub fn number_of_returns(&self) -> u8 {
        (self.return_info >> 4) & 0x0F // Extracts the upper 4 bits
    }
}

pub fn read_point_record3(
    file: &mut File,
    las_file_header: &LasFileHeader,
    ) -> io::Result<Point3D> {

    let mut buffer = [0; mem::size_of::<PointRecord>()];
    file.read_exact(&mut buffer)?;

    let point_record = PointRecord::new(&buffer)?;

    Ok(Point3D::new(
            f64::from(point_record.x) / las_file_header.x_scale_factor + las_file_header.x_offset,
            f64::from(point_record.y) / las_file_header.y_scale_factor + las_file_header.y_offset,
            f64::from(point_record.z) / las_file_header.z_scale_factor + las_file_header.z_offset,
            ))
}

pub fn read_point_record(file: &mut File) -> io::Result<Point3D> {
    let mut buffer = [0; std::mem::size_of::<PointRecord>()];
    file.read_exact(&mut buffer)?;

    // Assuming PointRecord6 is defined as described
    let point_record = PointRecord {
        x: i32::from_le_bytes(buffer[0..4].try_into().unwrap()),
        y: i32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        z: i32::from_le_bytes(buffer[8..12].try_into().unwrap()),
        intensity: u16::from_le_bytes(buffer[12..14].try_into().unwrap()),
        return_info: buffer[14] & 0x0F, // Extract lower 4 bits
        classification_flags: buffer[14] >> 4 & 0x0F, // Extract upper 4 bits
        scanner_channel: buffer[15] & 0x03, // Extract bits 0-1
        scan_direction_flag: (buffer[15] & 0x40) != 0, // Extract bit 6
        edge_of_flight_line: (buffer[15] & 0x80) != 0, // Extract bit 7
        classification: buffer[16],
        user_data: buffer[17],
        scan_angle: i16::from_le_bytes(buffer[18..20].try_into().unwrap()),
        point_source_id: u16::from_le_bytes(buffer[20..22].try_into().unwrap()),
        gps_time: f64::from_le_bytes(buffer[22..30].try_into().unwrap()),
    };

    Ok(Point3D::new(
        f64::from(point_record.x),
        f64::from(point_record.y),
        f64::from(point_record.z),
    ))
}

pub fn print_las_header_info(header: &LasFileHeader) {
    println!("Version: {}.{}", header.version_major, header.version_minor);
    println!("Header Size: {}", header.header_size);
    println!("Point Data Record Format: {}", header.point_data_record_format);
    println!("Number of Point Records: {}", header.legacy_number_of_point_records);
}

pub fn read_las_file(file_path: &Path) -> Result<Vec<Point3D>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let las_file_header = read_las_file_header(&mut file)?;

    print_las_header_info(&las_file_header);

    file.seek(SeekFrom::Start(las_file_header.offset_to_point_data as u64))?;

    let mut point_records: Vec<Point3D> = Vec::new();

    while let Ok(point_record) = read_point_record(&mut file) {
        println!(
            "Point: X = {:.2}, Y = {:.2}, Z = {:.2}",
            point_record.x,
            point_record.y,
            point_record.z
            );
        point_records.push(point_record);
    }

    Ok(point_records)
}