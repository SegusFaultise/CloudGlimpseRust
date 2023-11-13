use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;#[derive(Debug)]

/// Represents a 3D point with x, y, and z coordinates.
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents the header of a LAS file containing metadata about the file.
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

/// Represents a single point record in a LAS file.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PointRecord {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub intensity: u16,
    pub return_number: u8,
    pub number_of_returns: u8,

    pub classification_flags: u8,
    pub classification: u8,

    pub user_data: u8,
    pub scan_angle: i16,
    pub point_source_id: u8,
    pub gps_time: f64, 
}

/// Reads a LAS file and returns a vector of `Point3D` representing the point data.
/// 
/// # Arguments
/// * `file_path` - Path to the LAS file.
/// 
/// # Returns
/// Result containing either a Vector of `Point3D` or an error.
pub fn read_las_file(file_path: &Path) -> Result<Vec<Point3D>, Box<dyn Error>> {
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path.display(), e))?;

    let mut reader = BufReader::new(file);
    let las_file_header = read_las_file_header(&mut reader)
        .map_err(|e| format!("Failed to read LAS file header: {}", e))?;

    reader.seek(SeekFrom::Start(las_file_header.offset_to_point_data as u64))
        .map_err(|e| format!("Failed to seek to point data: {}", e))?;

    let mut point_records = Vec::new();

    loop {
        match read_point_record(&mut reader) {
            Ok(point_record) => point_records.push(convert_to_point3d(&point_record, &las_file_header)),
            Err(_) => break,
        }
    }
    print_las_header_info(&las_file_header);
    Ok(point_records)
}

/// Reads the header of a LAS file.
/// 
/// # Arguments
/// * `reader` - A mutable reference to a type implementing `Read`.
/// 
/// # Returns
/// Result containing either a `LasFileHeader` or an IO error.
pub fn read_las_file_header<R: Read>(reader: &mut R) -> io::Result<LasFileHeader> {
    let signature = reader.read_u32::<LittleEndian>()?;
    let file_source_id = reader.read_u16::<LittleEndian>()?;
    let global_encoding = reader.read_u16::<LittleEndian>()?;

    let project_id_guid_data_1 = reader.read_u32::<LittleEndian>()?;
    let project_id_guid_data_2 = reader.read_u16::<LittleEndian>()?;
    let project_id_guid_data_3 = reader.read_u16::<LittleEndian>()?;
    let mut project_id_guid_data_4 = [0u8; 8];

    reader.read_exact(&mut project_id_guid_data_4)?;

    let version_major = reader.read_u8()?;
    let version_minor = reader.read_u8()?;
    let mut system_identifier = [0u8; 32];

    reader.read_exact(&mut system_identifier)?;

    let mut generating_software = [0u8; 32];

    reader.read_exact(&mut generating_software)?;

    let file_creation_day_of_year = reader.read_u16::<LittleEndian>()?;
    let file_creation_year = reader.read_u16::<LittleEndian>()?;

    let header_size = reader.read_u16::<LittleEndian>()?;
    let offset_to_point_data = reader.read_u32::<LittleEndian>()?;
    let number_of_variable_length_records = reader.read_u32::<LittleEndian>()?;

    let point_data_record_format = reader.read_u8()?;
    let point_data_record_length = reader.read_u16::<LittleEndian>()?;

    let legacy_number_of_point_records = reader.read_u32::<LittleEndian>()?;
    let mut legacy_number_of_points_by_return = [0u32; 5];

    for num in legacy_number_of_points_by_return.iter_mut() {
        *num = reader.read_u32::<LittleEndian>()?;
    }

    let x_scale_factor = reader.read_f64::<LittleEndian>()?;
    let y_scale_factor = reader.read_f64::<LittleEndian>()?;
    let z_scale_factor = reader.read_f64::<LittleEndian>()?;

    let x_offset = reader.read_f64::<LittleEndian>()?;
    let y_offset = reader.read_f64::<LittleEndian>()?;
    let z_offset = reader.read_f64::<LittleEndian>()?;

    let max_x = reader.read_f64::<LittleEndian>()?;
    let max_y = reader.read_f64::<LittleEndian>()?;
    let max_z = reader.read_f64::<LittleEndian>()?;

    let min_x = reader.read_f64::<LittleEndian>()?;
    let min_y = reader.read_f64::<LittleEndian>()?;
    let min_z = reader.read_f64::<LittleEndian>()?;
    
    let start_of_waveform_data_packet_record = reader.read_u64::<LittleEndian>()?;
    let start_of_first_extended_variable_length_record = reader.read_u64::<LittleEndian>()?;

    let number_of_extended_variable_length_records = reader.read_u32::<LittleEndian>()?;

    Ok(LasFileHeader {
        signature: signature.to_le_bytes(),
        file_source_id,
        global_encoding,

        project_id_guid_data_1,
        project_id_guid_data_2,
        project_id_guid_data_3,
        project_id_guid_data_4,

        version_major,
        version_minor,

        system_identifier,
        generating_software,

        file_creation_day_of_year,
        file_creation_year,

        header_size,
        offset_to_point_data,
        number_of_variable_length_records,

        point_data_record_format,
        point_data_record_length,

        legacy_number_of_point_records,
        legacy_number_of_points_by_return,

        x_scale_factor,
        y_scale_factor,
        z_scale_factor,

        x_offset,
        y_offset,
        z_offset,

        max_x,
        max_y,
        max_z,

        min_x,
        min_y,
        min_z,

        start_of_waveform_data_packet_record,
        start_of_first_extended_variable_length_record,

        number_of_extended_variable_length_records,
    })
}

/// Reads a single point record from a LAS file.
/// 
/// # Arguments
/// * `reader` - A mutable reference to a type implementing `Read`.
/// 
/// # Returns
/// Result containing either a `PointRecord` or an IO error.
pub fn read_point_record<R: Read>(reader: &mut R) -> io::Result<PointRecord> {
    let x = reader.read_i32::<LittleEndian>()?;
    let y = reader.read_i32::<LittleEndian>()?;
    let z = reader.read_i32::<LittleEndian>()?;

    let intensity = reader.read_u16::<LittleEndian>()?;
    let return_number = reader.read_u8()?;
    let number_of_returns = reader.read_u8()?;
    let classification_flags = reader.read_u8()?;
    let classification = reader.read_u8()?;
    let user_data = reader.read_u8()?;
    let scan_angle = reader.read_i16::<LittleEndian>()?;
    let point_source_id = reader.read_u8()?;
    let gps_time = reader.read_f64::<LittleEndian>()?;

    Ok(PointRecord {
        x,
        y,
        z,

        intensity,

        return_number,
        number_of_returns,

        classification_flags,
        classification,

        user_data,
        scan_angle,
        point_source_id,
        gps_time,
    })
}

/// Converts a `PointRecord` to `Point3D` using the scale factors and offsets from the header.
/// 
/// # Arguments
/// * `record` - A reference to the point record.
/// * `header` - A reference to the LAS file header.
/// 
/// # Returns
/// `Point3D` representing the scaled and offset point.
fn convert_to_point3d(record: &PointRecord, header: &LasFileHeader) -> Point3D {
    //println!("X: {} | Y: {} | Z: {} ", record.x, record.y, record.z);
    Point3D {
        x: record.x as f64 * header.x_scale_factor + header.x_offset,
        y: record.y as f64 * header.y_scale_factor + header.y_offset,
        z: record.z as f64 * header.z_scale_factor + header.z_offset,
    }
}

/// Prints information from the LAS file header.
/// 
/// # Arguments
/// * `header` - A reference to the `LasFileHeader`.
//#[warn(dead_code)]
pub fn print_las_header_info(header: &LasFileHeader) {
    //println!("Z Offset: {:2} Y Offset: {:2} X Offset: {:2}", header.z_offset, header.y_offset, header.x_offset);
    //println!("Version: {}.{}", header.version_major, header.version_minor);
    //println!("Header Size: {}", header.header_size);
    //println!("Point Data Record Format: {}", header.point_data_record_format);
    //println!("Number of Point Records: {}", header.legacy_number_of_point_records);
}
