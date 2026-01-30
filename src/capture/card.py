import cv2
import numpy as np

class CardCapture:
    def __init__(self, index=1, box=416):
        self.cap = cv2.VideoCapture(index)
        if not self.cap.isOpened():
            raise RuntimeError("Unable connect to capture card")
        
        self.box = box

        self.width = int(self.cap.get(cv2.CAP_PROP_FRAME_WIDTH))
        self.height = int(self.cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

    def grab(self) -> np.ndarray:
        ret, frame = self.cap.read()
        if not ret:
            raise RuntimeError("Unable to read from capture card")

        # todo: Crop logic
        return frame
    

    def close(self):
        self.cap.release()


    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()