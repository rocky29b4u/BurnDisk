//! Optical drive enumeration via IMAPI2 (Windows).
use anyhow::Result;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED};
use windows::Win32::Storage::Imapi::{MsftDiscMaster2, IDiscMaster2};

#[derive(Debug, Clone)]
pub struct OpticalDrive {
    pub id: String,
    pub is_supported: bool,
}

pub fn enumerate_drives() -> Result<Vec<OpticalDrive>> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let master: IDiscMaster2 = CoCreateInstance(&MsftDiscMaster2, None, CLSCTX_ALL)?;
        let count = master.Count()?;
        let mut out = Vec::new();
        for i in 0..count {
            let id = master.get_Item(i)?;
            // Best-effort: treat all enumerated devices as supported recorders.
            out.push(OpticalDrive {
                id: id.to_string(),
                is_supported: true,
            });
        }
        Ok(out)
    }
}