#[cfg(feature = "impl")]
pub use scoper_impl::*;

#[cfg(not(feature = "impl"))]
pub use scoper_noop::*;

#[cfg(test)]
mod test
{
    use std::{path::Path, thread::sleep, time::Duration};

    use crate::{InstantScopeSize, RecordScope, macros::*, record_custom_value};
    fn wait_30_ms()
    {
        use crate::{Info, Scope, TraceInfo};
        static SCOPE_INFO: Info = &TraceInfo {
            //event_typ: EventType::Scope,
            name: "30 Millis",
            category: "inlinetest",
            header: "30 Millis",
            args: "",
        };

        let _profiling_scope = Scope::start(&SCOPE_INFO);
        let value = 0.8;
        record_custom_value(SCOPE_INFO, value);
        sleep(Duration::from_millis(30));
    }

    #[record]
    fn wait_30_ms_macro() { sleep(Duration::from_millis(30)); }

    #[record(custom header)]
    fn wait_30_ms_custom_header() { sleep(Duration::from_millis(30)); }

    #[test]
    fn basic_test_explicit_drop()
    {
        let record = RecordScope::start(Path::new("results/basic_test.json"));
        for _ in 0..10
        {
            wait_30_ms();
            sleep(Duration::from_millis(5));
        }
        drop(record);
    }

    #[test]
    fn implicit_test_explicit_drop()
    {
        let _record = RecordScope::start(Path::new("results/implicit_drop.json"));
        for _ in 0..10
        {
            wait_30_ms();
            sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn macro_test()
    {
        let _record = RecordScope::start(Path::new("results/macro_test.json"));
        for i in 0_i32..10
        {
            wait_30_ms_macro();
            sleep(Duration::from_millis(5));
            record_value!("", "test_value", 0.1 * f64::from(i + 1));
        }
    }

    #[test]
    fn custom_header_test()
    {
        let _record = RecordScope::start(Path::new("results/custom_header_test.json"));
        for _ in 0_i32..10
        {
            wait_30_ms_custom_header();
            sleep(Duration::from_millis(5));
        }
    }


    #[test]
    fn threads_test()
    {
        let mut record = RecordScope::start(Path::new("results/threads_test.json"));
        record
            .add_meta_data("test".to_string(), &String::from("SomeExtraInfoHere"))
            .ok();

        {
            record_scope!("Some setup");
            wait_30_ms();
        }

        std::thread::scope(|s| {
            s.spawn(|| {
                sleep(Duration::from_millis(20));
                record.set_starting_time();
                for _ in 0..10
                {
                    record_scope!("Myheader", "Thread C");
                    wait_30_ms_macro();
                    sleep(Duration::from_millis(5));
                }
            });
            s.spawn(|| {
                for _ in 0..10
                {
                    record_scope!("Thread D");
                    wait_30_ms();
                    sleep(Duration::from_millis(5));
                }
            });
        });
        record_instant!("First join", InstantScopeSize::Process);
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10
                {
                    record_scope!("Thread E");
                    wait_30_ms_macro();
                    sleep(Duration::from_millis(5));
                }
            });
        });
        record
            .add_meta_data("test".to_string(), &String::from("SomeExtraInfoHere"))
            .ok();
    }
}
