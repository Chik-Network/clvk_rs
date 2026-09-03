#![no_main]

use clvk_fuzzing::{make_clvk_program, make_tree_limits};
use libfuzzer_sys::{Corpus, fuzz_target};

use clvkr::allocator::Allocator;
use clvkr::chik_dialect::{ChikDialect, ClvkFlags, MEMPOOL_MODE};
use clvkr::cost::Cost;
use clvkr::error::EvalErr;
use clvkr::reduction::Reduction;
use clvkr::run_program::run_program;

fuzz_target!(|data: &[u8]| -> Corpus {
    let mut unstructured = arbitrary::Unstructured::new(data);
    let mut allocator = Allocator::new();
    let (args, _) =
        make_tree_limits(&mut allocator, &mut unstructured, 100, true).expect("out of memory");
    let Ok(program) = make_clvk_program(&mut allocator, &mut unstructured, args, 100_000) else {
        return Corpus::Reject;
    };

    let allocator_checkpoint = allocator.checkpoint();

    for flags in [
        ClvkFlags::ENABLE_GC,
        ClvkFlags::empty(),
        ClvkFlags::NO_UNKNOWN_OPS,
        MEMPOOL_MODE,
        ClvkFlags::LIMIT_SOFTFORK,
    ] {
        let dialect = ChikDialect::new(flags.union(ClvkFlags::DISABLE_OP));
        allocator.restore_checkpoint(&allocator_checkpoint);

        let result = run_program(
            &mut allocator,
            &dialect,
            program,
            args,
            11_000_000_000 as Cost,
        );

        match &result {
            Ok(Reduction(cost, _node)) => assert!(*cost < 11_000_000_000),
            Err(EvalErr::InternalError(..)) => {
                panic!("run_program returned InternalError: {:?}", result)
            }
            Err(_) => {}
        }
    }
    Corpus::Keep
});
