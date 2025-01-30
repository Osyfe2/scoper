
pub use const_format::str_replace;

#[macro_export]
macro_rules! record_scope {
    ($header: expr, $name: expr) => {
        static TRACE_SCOPE_INFO: $crate::TraceInfo = $crate::TraceInfo {
            name: $name,
            category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
            header: $header,
            args: "",
        };
        #[allow(unused)]
        let _profiling_scope = $crate::Scope::start(&TRACE_SCOPE_INFO);
    };
    ($name: expr) => {
        record_scope!("", $name)
    };
}

#[macro_export]
macro_rules! record_value {
    ($header: expr, $name: expr, $value: expr) => {
        {
            static TRACE_COUNTER_INFO: $crate::TraceInfo = $crate::TraceInfo {
                name: $name,
                category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
                header: $header,
                args: "",
            };
            $crate::record_custom_value(&TRACE_COUNTER_INFO, $value);
        }
    };
    /*
    ($name: expr) => {
        record_value!("", $name)
    };
    */
}

#[macro_export]
macro_rules! record_instant {
    ($header: expr, $name: expr, $scope_size: expr) => {
        {
            static TRACE_INSTANT_INFO: $crate::TraceInfo = $crate::TraceInfo {
                name: $name,
                category: $crate::macro_rules::str_replace!(::std::module_path!(), "::", ","),
                header: $header,
                args: "",
            };
            $crate::record_custom_instant(&TRACE_INSTANT_INFO, $scope_size);
        }
    };
    ($name: expr, $scope_size: expr) => {
        record_instant!("", $name, $scope_size);
    };
    ($name: expr) => {
        record_instant!($name, $crate::InstantScopeSize::Process);
    };
}