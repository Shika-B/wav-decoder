/// Reference: http://soundfile.sapp.org/doc/WaveFormat/
use crate::utility::{slice_to_array2, slice_to_array4};

const RIFF_ASCII: [u8; 4] = ['R' as u8, 'I' as u8, 'F' as u8, 'F' as u8];
const WAVE_ASCII: [u8; 4] = ['W' as u8, 'A' as u8, 'V' as u8, 'E' as u8];
const DATA_ASCII: [u8; 4] = ['d' as u8, 'a' as u8, 't' as u8, 'a' as u8];
const FMT_ASCII: [u8; 4] = ['f' as u8, 'm' as u8, 't' as u8, ' ' as u8];

#[derive(Debug)]
pub struct Wave {
    pub header: WaveHeader,
    pub data: WaveData,
}

impl Wave {
    pub fn new(file: &[u8]) -> Self {
        // Split the file as two slice of bytes, the header and the actual data
        let (header, data) = file.split_at(44);

        // Checks the header starts with the right ASCII word
        if &header[0..4] != RIFF_ASCII {
            panic!("Incorrect header")
        }

        let chunk_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

        // Checks the declared chunk size is correct. The minus eight is because the declared chunk size
        // isn't supposed to take in account the ChunkID and ChunkSize fields
        // (see reference, link in file wave_object.rs)
        if header.len() + data.len() - 8 != chunk_size as usize {
            panic!("Invalid declared chunk_size")
        }

        if &header[8..12] != WAVE_ASCII {
            panic!("Incorrect header")
        }

        if &header[12..16] != FMT_ASCII {
            panic!("Incorrect header")
        }
        // If the SubChunk1Size isn't set to 16, the value it is supposed to have when the AudioFormat
        // is set to 1 (PCM), raise an error.
        if slice_to_array4(16, &header) != [16, 0, 0, 0] {
            panic!("Invalid SubChunk1Size value in the header. Only PCM is supported")
        }
        if slice_to_array2(20, &header) != [1, 0] {
            panic!("Invalid AudioFormat")
        }

        let num_channels = u16::from_le_bytes(slice_to_array2(22, header));
        let sample_rate = u32::from_le_bytes(slice_to_array4(24, header));
        let byte_rate = u32::from_le_bytes(slice_to_array4(28, header));
        let block_align = u16::from_le_bytes(slice_to_array2(32, header));
        let bits_per_sample = u16::from_le_bytes(slice_to_array2(34, header));

        if &header[36..40] != DATA_ASCII {
            panic!("Incorrect header")
        }
        let data_size = u32::from_le_bytes(slice_to_array4(40, header));
        Wave {
            header: WaveHeader {
                chunk_size,
                num_channels,
                byte_rate,
                sample_rate,
                block_align,
                bits_per_sample,
            },
            data: WaveData {
                data_size,
                data: data.into(),
            },
        }
    }
}

#[derive(Debug)]
pub struct WaveHeader {
    chunk_size: u32,
    num_channels: u16,
    byte_rate: u32,
    sample_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[derive(Debug)]
pub struct WaveData {
    data_size: u32, // in the reference this field is called Subchunk2Size
    data: Vec<u8>,
}
