use crate::helpers;

pub struct Page {
    pub data: Vec<u8>,
    pub header: Header,
}

/*
page header
| u16 free space start | u16 free space length |
*/
pub struct Header {
    pub free_space: u16,
    pub free_space_off: u16,
}

impl Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.free_space.to_be_bytes());
        result.extend(self.free_space_off.to_be_bytes());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Header {
        assert_eq!(4, bytes.len());
        Header {
            free_space: helpers::to_u16(&bytes[..2]),
            free_space_off: helpers::to_u16(&bytes[2..4]),
        }
    }
}

impl Page {
    pub fn new() -> Page {
        Page {
            data: vec![0; 8188],
            header: Header {
                free_space: 8188,
                free_space_off: 8188,
            },
        }
    }

    pub fn append(&mut self, data: Vec<u8>) -> &Page {
        let data_len = data.len() as u16;
        let new_offset = (self.header.free_space_off - data_len) as usize;
        let _: Vec<u8> = self
            .data
            .splice(new_offset..self.header.free_space_off as usize, data)
            .collect();
        self.header.free_space_off = new_offset as u16;
        self.header.free_space = self.header.free_space - data_len;
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.header.to_bytes();
        result.extend(&self.data);
        result
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Page {
        assert_eq!(8192, bytes.len());
        Page {
            header: Header::from_bytes(&bytes[..4]),
            data: bytes[4..].to_vec(),
        }
    }
}

#[test]
fn new_page_has_correct_header() {
    //
}
