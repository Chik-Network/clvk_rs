#![no_main]

use clvk_fuzzing::ArbitraryClvkTree;
use clvkr::serde::is_canonical_serialization;
use clvkr::serde::node_to_bytes_backrefs;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|tree: ArbitraryClvkTree<1000, true>| {
    let buffer = node_to_bytes_backrefs(&tree.allocator, tree.tree)
        .expect("internal error, failed to serialize");
    // out serializer should always produce canonical serialization
    assert!(is_canonical_serialization(&buffer));
});
