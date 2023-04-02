import cv2 as cv

vid = cv.VideoCapture(0)

while True:
    _, cap = vid.read()
    cv.imshow("frame", cap)
    if cv.waitKey(1) & 0xFF == ord("q"):
        cv.destroyAllWindows()
        break
