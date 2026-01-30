from dataclasses import dataclass
from .geometry import Rectangle

@dataclass
class Confidence:
    confidence: float


@dataclass
class BodyPart(Confidence):
    position: Rectangle

@dataclass
class Head(BodyPart):
    pass


@dataclass
class Body(BodyPart):
    pass