import numpy as np
from typing import List
from ultralytics import YOLO

from domain.player import Player

class YOLODetector:
    def __init__(self, model_path: str, device: str = "cuda"):
        self.model = YOLO(model_path).to(device)
        self.model.fuse()

    def detect(self, frame: np.ndarray) -> List[Player]:
        return list()