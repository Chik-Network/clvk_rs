from typing import List, Optional, Tuple

from .clvk_storage import CLVKStorage

def run_serialized_chik_program(
    program: bytes, environment: bytes, max_cost: int, flags: int
) -> Tuple[int, CLVKStorage]: ...
def deserialize_as_tree(
    blob: bytes, calculate_tree_hashes: bool
) -> Tuple[List[Tuple[int, int, int]], Optional[List[bytes]]]: ...
def serialized_length(blob: bytes) -> int: ...

# --- Deserialize functions ---
def deser_legacy(blob: bytes) -> "LazyNode": ...
def deser_backrefs(blob: bytes) -> "LazyNode": ...
def deser_2026(
    blob: bytes,
    *,
    max_atom_len: int = ...,
    strict: bool = True,
) -> "LazyNode": ...
def deser_auto(
    blob: bytes,
    *,
    max_atom_len: int = ...,
    strict: bool = True,
) -> "LazyNode": ...

# --- Serialize functions ---
def ser_legacy(node: "LazyNode") -> bytes: ...
def ser_backrefs(node: "LazyNode") -> bytes: ...
def ser_2026(
    node: "LazyNode",
    *,
    level: int = 0,
) -> bytes: ...

# --- Tree conversion ---
def clvk_tree_to_lazy_node(obj: CLVKStorage) -> "LazyNode": ...

NO_UNKNOWN_OPS: int
LIMIT_HEAP: int
MEMPOOL_MODE: int
ENABLE_SHA256_TREE: int
ENABLE_SECP_OPS: int
DISABLE_OP: int
CANONICAL_INTS: int

class LazyNode(CLVKStorage):
    atom: Optional[bytes]

    @property
    def pair(self) -> Optional[Tuple[CLVKStorage, CLVKStorage]]: ...
