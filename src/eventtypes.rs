use std::marker::ConstParamTy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]
pub enum EventType
{
    Scope,
    //Begin, //Scope is sufficient
    //End,
    Instant,

    //AsyncStart,
    //AsyncProgress,
    //AsyncFinish,

    //FlowStart,
    //FlowProgress,
    //FlowFinish,

    //ObjectCreated,
    //ObjectSnapshot,
    //ObjectDestroyed,
    Counter,
    //Sample, //deprecated
    //ClockSync,
    //Mark, too similar to Instant
    //MemoryDumpGlobal,
    //MemoryDumpProcess,
    //ContextStart,
    //ContextEnd,
}

impl EventType
{
    #[must_use]
    pub fn code(self) -> char
    {
        #[allow(clippy::enum_glob_use)]
        use EventType::*;
        match self
        {
            Scope => 'X',
            //Begin => 'B',
            //End => 'E',
            Instant => 'i',

            //AsyncStart => 'b',
            //AsyncProgress => 'n',
            //AsyncFinish => 'e',

            //FlowStart => 's',
            //FlowProgress => 't',
            //FlowFinish => 'f',

            //ObjectCreated => 'N',
            //ObjectSnapshot => 'O',
            //ObjectDestroyed => 'D',
            Counter => 'C',
            //in source Sample => P
            //ClockSync => 'c',
            //Mark => 'R',
            //in doc ContextStart => '(',
            //in doc ContextEnd => ')',
        }
    }
}
