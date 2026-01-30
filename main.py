from dataclasses import dataclass
from typing import Optional

@dataclass
class Vector3:
    x: float
    y: float
    z: float

@dataclass
class BodyPart:
    position: Vector3

@dataclass
class Head(BodyPart):
    pass


@dataclass
class Body(BodyPart):
    pass

@dataclass
class Player:
    isEnemy: bool = True
    head: Head | None = None
    body: Body | None = None
