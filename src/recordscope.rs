use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
    thread::ThreadId,
};

use serde_json as json;

use crate::{MetaTrace, Pid, TimePoint};

pub struct RecordScope
{
    pub(crate) path: PathBuf,
    pub(crate) record_start: TimePoint,
    pub(crate) meta_data: json::Map<String, json::Value>,
    pub(crate) meta_traces: Vec<MetaTrace>,
}

impl RecordScope
{
    pub fn start(path: impl AsRef<Path>) -> Self
    {
        Self {
            path: path.as_ref().with_extension("json"),
            record_start: TimePoint::now(),
            meta_data: json::Map::default(),
            meta_traces: Vec::default(),
            //section: "",
        }
    }

    pub fn set_starting_time(&mut self)
    {
        self.record_start = TimePoint::now()
    }
}

impl Drop for RecordScope
{
    fn drop(&mut self) { self.write().unwrap_or_else(|err| println!("Failed dump - Reason: {err}")); }
}

impl RecordScope
{
    pub(super) fn write(&mut self) -> std::io::Result<()>
    {
        //TODO allow appending as different process instead of overwriting
        let writer = &mut BufWriter::new(File::create(&self.path)?);
        let data = self.fetch_data();
        json::to_writer(writer, &data)?;

        Ok(())
    }

    /// Adds a metadata field to the scope
    /// Returns the existing json-value with that name if any is present
    ///
    /// # Errors
    /// Returns an Error if the Data is invalid json
    pub fn add_meta_data(&mut self, name: String, data: &impl serde::Serialize) -> Result<Option<json::Value>, serde_json::Error>
    {
        let json = json::value::to_value(data)?;
        Ok(self.meta_data.insert(name, json))
    }

    pub fn name_thread(&mut self, thread_id: ThreadId, header: Pid, name: String)
    {
        self.meta_traces.push(MetaTrace::ThreadName(header, thread_id.as_u64(), name));
    }

    pub fn final_header(&mut self, old_header: Pid, new_header: String)
    {
        self.meta_traces.push(MetaTrace::ProcessName(old_header, new_header));
    }
}
