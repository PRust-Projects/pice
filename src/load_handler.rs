extern crate sciter;

use sciter::host;
use std::fs::File;
use std::io::Read;

pub struct LoadHandler {
    archive: host::Archive,
}

impl LoadHandler {

    pub fn new(archive: &[u8]) -> Self {
        Self {
            archive: host::Archive::open(archive).expect("Unable to open archive."),
        }
    }

}

impl sciter::HostHandler for LoadHandler {

    fn on_data_load(&mut self, pnm: &mut host::SCN_LOAD_DATA) -> Option<host::LOAD_RESULT> {
        let uri = w2s!(pnm.uri);

        if uri.starts_with("this://app/") {
            if let Some(data) = self.archive.get(&uri) {
                self.data_ready(pnm.hwnd, &uri, data, None);
            } else {
                let filename = uri["this://app".to_string().len()..].to_owned();
                match File::open(filename.clone()) {
                    Ok(mut file) => {
                        let mut content = Vec::new();
                        file.read_to_end(&mut content).expect("Something went wrong while reading file");
                        self.data_ready(pnm.hwnd, &uri, &content, None);
                    }
                    Err(err) => { eprintln!("[handler] error: can't load {} because of {}", filename, err); }
                }
            }
        }
        return Some(host::LOAD_RESULT::LOAD_DEFAULT);
    }

}
