use serde_json::{json, Map, Number, Value as JsonValue};

use crate::{
    eventtypes::EventType, MetaTrace, RecordScope, TaggedTrace, TimePoint, TraceInfo, Value,
    global::{self},
};

impl RecordScope
{
    pub(crate) fn fetch_data(&mut self) -> Map<String, JsonValue>
    {
        let traces = global::flush_buffers().map(|t| t.json_format(self.record_start));

        //Sorting not needed but might be faster on load
        //But needed per thread -> should hold without sort -> Testing
        //traces.sort_by(Trace::cmp_start);

        let traces = self
        .meta_traces
        .iter()
        .map(MetaTrace::json_format)
        .chain(traces)
        .collect();

        let mut data = Map::new();
        data["traceEvents"] = JsonValue::Array(traces);
        data["displayTimeUnit"] = json!("ms"); //ns allowed as well
        data.append(&mut self.meta_data);
        data
    }
}

impl TaggedTrace
{
    fn code(&self) -> char { 
        match self {
            TaggedTrace::Scope(_) => EventType::Scope.code(),
            TaggedTrace::Counter(_) => EventType::Counter.code(),
            TaggedTrace::Instant(_) => EventType::Instant.code(),
        }
     }

    fn json_format(&self, zero: TimePoint) -> JsonValue
    {
        use TaggedTrace::*;
        let base = match self
        {
            Scope(trace) => &trace.base,
            Counter(trace) => &trace.base,
            Instant(trace) => &trace.base,
        };

        let time_stamp = base.start.duration_since(zero).as_micros();
        let &TraceInfo {
            name,
            category,
            header,
            args,
        } = base.info;

        let mut ret = json!({
            "name": name,
            "cat": category,
            "pid": header,
            "tid": base.thread_id.as_u64(),
            "ph": self.code(),
            "ts": time_stamp,
            "args": args,
        });

        match self
        {
            Scope(scope) =>
            {
                let dur = scope.end.duration_since(scope.base.start).as_micros();
                ret["dur"] = json!(dur);
            },
            Counter(counter) =>
            {
                let args = &mut ret["args"];
                let value = counter.value.as_number();
                let mut extra_args = std::mem::replace(
                    args,
                    json!({
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
            Instant(instant) =>
            {
                ret["s"] = json!(instant.scope_size.code());
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
