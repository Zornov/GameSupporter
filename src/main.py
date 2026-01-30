import cv2
import torch

from capture.card import CardCapture
from capture.screen import ScreenCapture
from vision.detector import YOLODetector
from domain.geometry import Rectangle

def main():
    torch.set_num_threads(1)

    cv2.namedWindow("Capture", cv2.WINDOW_NORMAL)
    cv2.setWindowProperty("Capture", cv2.WND_PROP_TOPMOST, 1)

    # detector = YOLODetector("test")

    # with ScreenCapture(416) as capture:
    #     while True:
    #         frame = capture.grab()
    #         players = detector.detect(frame)


    #         cv2.imshow("Capture", frame)

    #         if cv2.waitKey(1) == 27:
    #             break


    with CardCapture(1, 416) as capture:
        while True:
            frame = capture.grab()
            
            cv2.imshow("Capture", frame)

            if cv2.waitKey(1) == 27:
                break

    cv2.destroyAllWindows()


if __name__ == "__main__":
    main()