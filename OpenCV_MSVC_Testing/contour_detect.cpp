#include <opencv2/imgcodecs.hpp>
#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>
#include <iostream>


//Trying to write this file without namescapes ( In larger projects I probably wont declare a namespace for cv)



void getContours(cv::Mat imgDil, cv::Mat img) {

	std::vector<std::vector<cv::Point>> contours;
	std::vector<cv::Vec4i> hierarchy;

	cv::findContours(imgDil, contours, hierarchy, cv::RETR_EXTERNAL, cv::CHAIN_APPROX_SIMPLE);
	cv::drawContours(img, contours, -1, cv::Scalar(255, 0, 255), 5);
}


int main() {

	//reading in img

	std::string path2Img = "Resources/shapes.png";
	
	cv::Mat img = cv::imread(path2Img); //OpenCV file reader?

	//Declares some image matrices
	cv::Mat imgGrayscale, imgBlur, imgCanny, imgDilated, imgErode;


	//? >>> PREPROCESSING !!!

	//GrayScaling
	cv::cvtColor(img, imgGrayscale, cv::COLOR_BGR2GRAY); //output: imgGrayscale

	//Gaussian parameters
	int size = 5;
	double sigX = 5; double sigY = 2;
	//in, //out, Size(width,height), sigma X , sigma Y
	cv::GaussianBlur(img, imgBlur, cv::Size(size, size), sigX, sigY);

	//Canny, this is basically a "draw img contours" type of thing, outputs a black img with white lines as contour
	//in, out, threshold 1, threshold 2
	cv::Canny(imgBlur, imgCanny, 25, 75);

	//Kernel is some kind of matrix the dilate function re-structures the img by? maybe similar to polarization ?
	cv::Mat kernel = cv::getStructuringElement(cv::MORPH_RECT, cv::Size(5, 5));

	//This function basically dilates the canny's contours to make them thicker and more visible, it helps OpenCV's contour detection algorithm to work better
	cv::dilate(imgCanny, imgDilated, kernel);

	//? >>> END OF PREPROCESSING !!!

	cv::imshow("Image Preview", imgDilated);
	cv::waitKey(0);
	return 0;
}

