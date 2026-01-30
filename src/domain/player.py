from dataclasses import dataclass
from typing import Optional
from .body_parts import Head, Body

@dataclass
class Player:
    isEnemy: bool = True
    head: Optional[Head] = None
    body: Optional[Body] = None