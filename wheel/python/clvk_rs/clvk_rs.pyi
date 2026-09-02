from typing import List, Optional, Tuple

from .clvk_storage import CLVKStorage

def run_serialized_chik_program(
    program: bytes, environment: bytes, max_cost: int, flags: int
) -> Tuple[int, CLVKStorage]: ...
def deserialize_as_tree(
    blob: bytes, calculate_tree_hashes: bool
) -> Tuple[List[Tuple[int, int, int]], Optional[List[bytes]]]: ...
def serialized_length(blob: bytes) -> int: ...

NO_NEG_DIV: int
NO_UNKNOWN_OPS: int
LIMIT_HEAP: int
MEMPOOL_MODE: int

class LazyNode(CLVKStorage):
    atom: Optional[bytes]

    @property
    def pair(self) -> Optional[Tuple[CLVKStorage, CLVKStorage]]: ...
