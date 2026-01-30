from dataclasses import dataclass

@dataclass(frozen=True)
class Rectangle:
    xMin: int
    yMin: int
    xMax: int
    yMax: int

    @property
    def width(self) -> int:
        return int(self.xMax - self.xMin)

    @property
    def height(self) -> int:
        return int(self.yMax - self.yMin)