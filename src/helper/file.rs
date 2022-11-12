
use std::fs::File;
use std::io::{Read, Write};

struct CacheObj{
    path: String,
    data: String
}

impl CacheObj{
    pub fn new(path: &str, data: String)-> Self{
        CacheObj { path: path.to_string(), data}
    }

    pub fn is_at_path(self, path2: &str)->bool{
        self.path == path2

    }

    pub fn read_data(self)->String{
        self.data.to_string()
    }
}

static mut CACHE: Vec<CacheObj> = Vec::new();

pub fn read(path: &str, buf: &mut String) -> bool{
    File::open(path).unwrap() // source
    .read_to_string(buf) // get value
    .unwrap(); // convert to string
    unsafe{
        CACHE.insert(CACHE.len(), CacheObj::new(path, buf.to_string()));
    }
    true
}

pub fn write(path: &str, data: &[u8]) -> bool{
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(data).expect("write failed");
    true
}
