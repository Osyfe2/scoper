use serde_json::{Map, Number, Value as JsonValue, json};

use crate::{
    event_types::EventType, global::{self}, record_scope::MetaTrace, types::{TaggedData, TaggedTrace, Trace, Value}, RecordScope, TimePoint, TraceInfo
};

impl RecordScope
{
    pub(crate) fn fetch_data(&mut self) -> Map<String, JsonValue>
    {
        let mut traces: Vec<_> = global::flush_buffers().collect();

        traces.sort_by(TaggedTrace::cmp_start);

        let traces = traces.iter().filter_map(|t| t.json_format(self.record_start));

        let traces: Vec<_> = self.meta_traces.iter().map(MetaTrace::json_format).chain(traces).collect();
        let mut data = Map::new();
        data.insert("traceEvents".to_string(), traces.into());
        data.insert("displayTimeUnit".to_string(), json!("ms")); //ns allowed as well
        data.append(&mut self.meta_data);
        
        data
    }
}

#[allow(dead_code)]
// Viewer do not handle negative well
fn signed_time(earlier: TimePoint, later: TimePoint) -> i128
{
    if let Some(pdur) = later.checked_duration_since(earlier)
    {
        pdur.as_micros().try_into().unwrap()
    }
    else
    {
        -TryInto::<i128>::try_into(earlier.duration_since(later).as_micros()).unwrap()
    }
}

impl TaggedTrace
{
    fn start(&self) -> &TimePoint
    {
        match &self.1
        {
            TaggedData::Scope(start) => &start.0,
            _ => &self.end(),
        }
    }

    fn end(&self) -> &TimePoint
    {
        &self.0.time_point
    }

    fn cmp_start(&self, other: &TaggedTrace) -> std::cmp::Ordering
    {
        match self.start().cmp(other.start())
        {
            std::cmp::Ordering::Equal => other.end().cmp(self.end()),
            c => c,
        }
    }
}

impl TaggedTrace
{
    fn code(&self) -> char
    {
        match self.1
        {
            TaggedData::Scope(_) => EventType::Scope.code(),
            TaggedData::Counter(_) => EventType::Counter.code(),
            TaggedData::Instant(_) => EventType::Instant.code(),
        }
    }

    fn json_format(&self, zero: TimePoint) -> Option<JsonValue>
    {
        // Viewer do not handle negative well
        //let time_stamp = signed_time(zero, base.start);
        let time_point = self.0.time_point.checked_duration_since(zero)?.as_micros();

        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = self.0.info;

        let mut ret = json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": self.0.thread_id.as_u64(),
            "ph": self.code(),
            "ts": time_point,
            "args": args,
        });

        adjust_specific_atributes(ret.as_object_mut().unwrap(), self, zero);
        fn adjust_specific_atributes(ret: &mut Map<String, JsonValue>, Trace(base, tag): &TaggedTrace, zero: TimePoint)
        {
            use TaggedData::*;
            match tag
            {
                Scope(start) =>
                {
                    let start = start.0.duration_since(zero).as_micros();
                    let dur = base.time_point.duration_since(zero).as_micros() - start;
                    ret["ts"] = json!(start);
                    ret.insert("dur".to_string(), json!(dur));
                },
                Counter(value) =>
                {
                    let args = &mut ret["args"];
                    let mut extra_args = std::mem::replace(args, value.as_args());

                    if let Some(valid_map) = extra_args.as_object_mut()
                    {
                        args.as_object_mut().unwrap().append(valid_map);
                    }
                    else if !extra_args.is_null()
                    {
                        args["args"] = extra_args;
                    }
                },
                Instant(scope_size) =>
                {
                    ret.insert("s".to_string(), json!(scope_size.code()));
                },
            }
        }

        Some(ret)
    }
}

impl Value
{
    fn as_number(&self) -> Number
    {
        use Value::*;
        match self
        {
            &UInt(uint) => Number::from_u128(uint.into()),
            &IInt(iint) => Number::from_i128(iint.into()),
            &Float(float) => Number::from_f64(float),
        }
        .unwrap()
    }

    fn as_args(&self) -> JsonValue
    {
        json!({
            "": self.as_number(),
        })
    }
}

impl MetaTrace
{
    fn json_format(&self) -> JsonValue
    {
        let (pid, tid, name) = match self
        {
            MetaTrace::ProcessName(pid, name) => (pid, 0_u64, name),
            MetaTrace::ThreadName(pid, tid, name) => (pid, tid.get(), name),
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
        };

        serde_json::json!({
            "args": {"name": name},
            "cat": "__metadata",
            "name": "thread_name",
            "ph": "M",
            "pid": pid,
            "tid": tid,
            "ts": 0,
        })
    }
}
