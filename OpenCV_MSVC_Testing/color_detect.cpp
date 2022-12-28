#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <iostream>

using namespace cv;
using namespace std;

/// imgs///

Mat imgHSV, mask;
int hmin = 0, smin = 110, vmin = 153;
int hmax = 19, smax = 240, vmax = 255;

int main() {

	//! 1.) Get Image
	string path2Img = "Resources/black_box_no_cast.png";
	Mat img = imread(path2Img); //OpenCV file reader?

	cvtColor(img, imgHSV, COLOR_BGR2HSV);

	namedWindow("Trackbars", (500, 500)); //Trackbar window

	//! SLIDERS FOR HSV VALUE QUICK EDITING (This way you can isolate colors using HSV Ranges)

	//Hue
	createTrackbar("Hue Min", "Trackbars", &hmin, 179);
	createTrackbar("Hue Max", "Trackbars", &hmax, 179);

	// Saturation & V
	createTrackbar("Sat Min", "Trackbars", &smin, 255);
	createTrackbar("Sat Max", "Trackbars", &smax, 255);

	createTrackbar("Val Min", "Trackbars", &vmin, 255);
	createTrackbar("Val Max", "Trackbars", &vmax, 255);

	while (true) {
		Scalar lower(hmin, smin, vmin);
		Scalar upper(hmax, smax, vmax);
		inRange(imgHSV, lower, upper, mask);

		imshow("img mask", mask); //popup gui img
		waitKey(1);
	}

	return 0;
}