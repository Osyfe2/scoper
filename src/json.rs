use serde_json::{Map, Number, Value as JsonValue};

use crate::{
    EventType, MetaTrace, RecordScope, TaggedTrace, TimePoint, Trace, TraceInfo, Value,
    global::{self},
};

impl RecordScope
{
    pub(crate) fn fetch_data(&mut self) -> Map<String, JsonValue>
    {
        let traces = global::flush_buffers();

        //Not needed but might be faster on load
        //Needed per thread -> but should hold -> Testing
        //traces.sort_by(Trace::cmp_start);

        let mut data = Map::new();
        let traces = self
            .meta_traces
            .iter()
            .map(MetaTrace::json_format)
            .chain(traces.map(|t| t.json_format(self.record_start)))
            .collect();
        data.insert("traceEvents".to_string(), JsonValue::Array(traces));
        data.insert("displayTimeUnit".to_string(), JsonValue::String("ms".to_string())); //ns allowed as well
        data.append(&mut self.meta_data);
        data
    }
}

impl TaggedTrace
{
    fn json_format(&self, zero: TimePoint) -> JsonValue
    {
        match self
        {
            TaggedTrace::Scope(trace) => trace.json_format(zero),
            TaggedTrace::Counter(trace) => trace.json_format(zero),
            TaggedTrace::Instant(trace) => trace.json_format(zero),
        }
    }
}

impl<const TYPE: EventType> Trace<TYPE>
{
    fn code() -> char { TYPE.code() }

    fn json_format(&self, zero: TimePoint) -> JsonValue
    {
        let time_stamp = self.start.duration_since(zero).as_micros();
        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = self.info;

        let mut ret = serde_json::json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": self.thread_id.as_u64(),
            "ph": Self::code(),
            "ts": time_stamp,
            "args": args,
        });

        match TYPE
        {
            EventType::Scope =>
            {
                let dur = unsafe { self.addition.end }.duration_since(self.start).as_micros();
                ret["dur"] = serde_json::json!(dur);
            },
            EventType::Counter =>
            {
                let args = &mut ret["args"];
                let value = unsafe { &self.addition.value }.as_number();
                let mut extra_args = std::mem::replace(
                    args,
                    serde_json::json!({
                        name: value,
                    }),
                );

                if let Some(valid_map) = extra_args.as_object_mut()
                {
                    args.as_object_mut().unwrap().append(valid_map);
                }
                else if !extra_args.is_null()
                {
                    args["args"] = extra_args;
                }
            },
            EventType::Instant =>
            {
                let scope_size = unsafe { self.addition.scope_size };
                ret["s"] = serde_json::json!(scope_size.code());
            },
        }

        ret
    }
}

impl Value
{
    fn as_number(self) -> Number
    {
        use Value::*;
        match self
        {
            UInt(uint) => Number::from_u128(uint.into()),
            IInt(iint) => Number::from_i128(iint.into()),
            Float(float) => Number::from_f64(float),
        }
        .unwrap()
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
