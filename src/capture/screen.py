import cv2
import numpy as np
import mss

class ScreenCapture:
    def __init__(self, box: int, monitor: int = 1):
        self.box = box
        self.monitor = monitor
        self.sct = mss.mss()

        mon = self.sct.monitors[monitor]
        w, h = mon["width"], mon["height"]

        self.region = {
            "top": h // 2 - box // 2,
            "left": w // 2 - box // 2,
            "width": box,
            "height": box,
        }

    def grab(self) -> np.ndarray:
        img = self.sct.grab(self.region)
        return cv2.cvtColor(np.asarray(img), cv2.COLOR_BGRA2BGR)

    def close(self):
        self.sct.close()

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()