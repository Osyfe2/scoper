use serde_json::{Map, Value};

use crate::{EventType, MetaTrace, RecordScope, TimePoint, Trace, TraceInfo, global};

impl RecordScope
{
    pub(crate) fn fetch_data(&mut self) -> Map<String, Value>
    {
        let traces = global::flush_traces();
        let counters = global::flush_counters();
        let instances = global::flush_instances();

        //Not needed but might be faster on load
        //Needed per thread but should hold -> Testing
        //traces.sort_by(Trace::cmp_start);

        let mut data = Map::new();
        let traces = self
            .meta_traces
            .iter()
            .map(MetaTrace::json_format)
            .chain(traces.iter().map(|t| t.json_format(self.record_start)))
            .chain(counters.iter().map(|t| t.json_format(self.record_start)))
            .chain(instances.iter().map(|t| t.json_format(self.record_start)))
            .collect();
        data.insert("traceEvents".to_string(), Value::Array(traces));
        data.insert("displayTimeUnit".to_string(), Value::String("ms".to_string())); //ns allowed as well
        data.append(&mut self.meta_data);
        data
    }
}

impl Trace<{ EventType::Scope }>
{
    fn json_format(&self, zero: TimePoint) -> Value
    {
        let time_stamp = self.start.duration_since(zero).as_micros();
        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = self.info;

        let dur = unsafe { self.addition.end }.duration_since(self.start).as_micros();

        serde_json::json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": self.thread_id.as_u64(),
            "ph": EventType::Scope.code(),
            "ts": time_stamp,
            "args": args,
            "dur": dur
        })
    }
}

impl Trace<{ EventType::Counter }>
{
    fn json_format(&self, zero: TimePoint) -> Value
    {
        let time_stamp = self.start.duration_since(zero).as_micros();
        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = self.info;

        let (i, f) = unsafe { self.addition.int_float };
        let mut counter = serde_json::json!({
            "Int": i,
            "Float": f,
        });

        if !args.is_empty()
        {
            if let Some(valid_json) = serde_json::json!(args).as_object_mut()
            {
                counter.as_object_mut().unwrap().append(valid_json);
            }
            else
            {
                let mut extra_args = serde_json::json!({"args": args});
                counter.as_object_mut().unwrap().append(extra_args.as_object_mut().unwrap());
            }
        }

        serde_json::json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": self.thread_id.as_u64(),
            "ph": EventType::Counter.code(),
            "ts": time_stamp,
            "args": counter,
        })
    }
}

impl Trace<{ EventType::Instant }>
{
    fn json_format(&self, zero: TimePoint) -> Value
    {
        let time_stamp = self.start.duration_since(zero).as_micros();
        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = self.info;

        let scope_size = unsafe { self.addition.scope_size };

        serde_json::json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": self.thread_id.as_u64(),
            "ph": EventType::Instant.code(),
            "ts": time_stamp,
            "s": scope_size.code(),
            "args": args,
        })
    }
}

/*
impl Trace {
    fn cmp_start(&self, other: &Self) -> Ordering
    {
    self.start.cmp(&other.start)
        match self.start.cmp(&other.start)
        {
        Ordering::Equal => other.end.cmp(&self.end),
            o => o,
        }
    }
}
*/

impl MetaTrace
{
    fn json_format(&self) -> Value
    {
        match self
        {
            MetaTrace::ThreadName(pid, tid, name) => serde_json::json!({
                "args": {"name": name},
                "cat": "__metadata",
                "name": "thread_name",
                "ph": "M",
                "pid": pid,
                "tid": tid,
                "ts": 0,
            }),
            MetaTrace::ProcessName(pid, name) => serde_json::json!({
                "args": {"name": name},
                "cat": "__metadata",
                "name": "process_name",
                "ph": "M",
                "pid": pid,
                "tid": 0,
                "ts": 0,
            }),
            /* Not in doc
            MetaEvent::ProcessUptimeSeconds(pid, uptime) => serde_json::json!({
                "args": {"uptime": uptime},
                "cat": "__metadata",
                "name": "process_uptime_seconds",
                "ph": "M",
                "pid": pid,
                "tid": 0,
                "ts": 0,
            }),*/
            /* Not in the doc
            MetaEvent::ActiveProcesses(vec, time) => serde_json::json!({
                "args": {"chrome_active_processes": vec},
                "cat": "__metadata",
                "name": "ActiveProcesses",
                "ph": "I",
                "pid": 0,
                "s": "g",
                "tid": 0,
                "ts": time,
            }),*/
        }
    }
}
